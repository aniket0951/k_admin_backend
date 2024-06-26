use std::{path::PathBuf};
extern crate hex;
use actix_multipart::form::MultipartForm;
use actix_web::{ web::{Data, Path ,Json}, HttpResponse, Responder};
use bson::oid::ObjectId;
use validator::validate_email;
extern crate sanitize_filename;
use crate::{dto::student_dto::{CreateParentDTO, CreateStudentDTO, StudentsDTO, UploadProfileDTO}, helper::{self, app_errors::{AppError, Messages}, response::ResponseBuilder}, models::student_model::{Parents, Students}, repo::student_repo::StudentRepo};

use super::jwt_service;

#[allow(non_snake_case)]
pub async fn add_student(db:Data<StudentRepo>, request:Json<CreateStudentDTO>) -> impl Responder {
    if request.name.is_empty() {
        return HttpResponse::BadRequest().json(
            ResponseBuilder::<()>::FailedResponse("Invalid request params".to_string())
        );
    };

    if request.geneder.to_string().to_lowercase() != "male" && request.geneder.to_string().to_lowercase() != "female" {
        return HttpResponse::BadRequest().json(
            ResponseBuilder::<()>::FailedResponse("Invalid gender".to_string())
        )
    }

    let seq = helper::helper::Helper::generate_unique_number();

    let req_level = request.level.to_string();
    let mut class_branch = String::new();
    if !request.class_branch.is_none() {
        class_branch = request.class_branch.as_ref().unwrap().to_string();
    }
    let student = Students {
        id: None,
        name: request.name.to_string(),
        age: request.age,
        date_of_birth: request.date_of_birth.to_string(),
        address: request.address.to_string(),
        is_active_student: true,
        class_branch: Some(class_branch),
        parent: None,
        created_at: Some(bson::DateTime::now()),
        updated_at: Some(bson::DateTime::now()),
        profile_pic: None,
        level:Some( req_level.to_lowercase()),
        nationality: Some("INDIAN".to_string()),
        blood_group: Some(request.blood_group.to_string()),
        weight: Some(request.weight.into()),
        school_name: Some(request.school_name.to_string()),
        addhar_number: Some(request.addhar_number.to_string()),
        geneder: Some(request.geneder.to_string()),
        student_id: Some(seq.to_string()),
        registration_status: Some("PENDING".to_string()),
    };

    match db.add_student(student).await {
        Ok(resultId) => {
            HttpResponse::Ok().json(
                ResponseBuilder::SuccessResponse(Messages::DataAddedSuccess.to_string(), Some(
                    resultId.inserted_id.as_object_id().unwrap().to_hex()
                ))
            )
        },
        Err(e) => {
            HttpResponse::BadRequest().json(
                ResponseBuilder::<()>::FailedResponse(e.to_string())
            )
        },
    }
}

pub async fn get_students(db:Data<StudentRepo>, path:Path<(i64, i64, String)>) -> impl Responder {
    let (skip, limit,level) = path.into_inner();
    match db.get_students(skip, limit, level).await {
        Ok(students) => {
            if students.len() == 0 {
                return HttpResponse::NotFound().json(
                    ResponseBuilder::<()>::FailedResponse(Messages::DataFetchFailed.to_string())
                );
            }

            let mut students_dto:Vec<StudentsDTO> = Vec::new();

            for i in students {
                students_dto.push(StudentsDTO::init(i,"".to_string()))
            }

            HttpResponse::Ok().json(
                ResponseBuilder::SuccessResponse(
                    Messages::DataFetchSuccess.to_string(),
                    Some(students_dto)
                )
            )
        },
        Err(e) => {
            HttpResponse::BadRequest().json(
                ResponseBuilder::<()>::FailedResponse(e.to_string())
            )
        },
    }
}

pub async fn total_students(db:Data<StudentRepo>) -> impl Responder {
    let result = db.total_students().await;
    HttpResponse::Ok().json(
        ResponseBuilder::SuccessResponse(
            format!("Total Count fetched"),
            Some(result)
        )
    )
}

#[allow(non_snake_case)]
pub async fn get_student(db:Data<StudentRepo>, path:Path<String>) -> impl Responder {
    match ObjectId::parse_str(path.into_inner()) {
        Ok(objId) => {
            match db.get_student(objId).await {
                Ok(student) => {
                    HttpResponse::Ok().json(
                        ResponseBuilder::SuccessResponse(
                            Messages::DataFetchSuccess.to_string(),
                            Some(StudentsDTO::init(student,"".to_string()))
                        )
                    )
                },
                Err(err) => {
                    HttpResponse::BadRequest().json(
                        ResponseBuilder::<()>::FailedResponse(err.to_string())
                    )
                },
            }
        },
        Err(_) => {
            HttpResponse::BadRequest().json(
                ResponseBuilder::<()>::InValidIdResponse()
            )
        },
    }
}

#[allow(non_snake_case)]
pub async fn delete_student(db:Data<StudentRepo>, path:Path<String>) -> impl Responder {
    match ObjectId::parse_str(path.into_inner()) {
        Ok(objeId) => {
            match db.delete_student(objeId).await {
                Ok(result) => {
                    if result.deleted_count == 0 {
                        return HttpResponse::BadRequest().json(
                            ResponseBuilder::<()>::FailedResponse(Messages::DataDeleteFailed.to_string())
                        );
                    }

                    HttpResponse::Ok().json(
                        ResponseBuilder::<()>::SuccessResponse(
                            Messages::DataDeleteSucess.to_string(),
                            None
                        )
                    )
                },
                Err(e) => {
                    HttpResponse::BadRequest().json(
                        ResponseBuilder::<()>::FailedResponse(e.to_string())
                    )
                },
            }
        },
        Err(_) => {
            HttpResponse::BadRequest().json(
                ResponseBuilder::<()>::InValidIdResponse()
            )
        },
    }
}

#[allow(non_snake_case)]
pub async fn upload_profile(db:Data<StudentRepo>, path:Path<String> , payload:MultipartForm<UploadProfileDTO>) -> impl Responder {
    
    match ObjectId::parse_str(path.into_inner()) {
        Ok(objId) => {
                    
            let f_file_path = format!("/static/student/");
        
            let temp_file_path = payload.file.file.path();
            let file_name: &str = payload
                .file
                .file_name
                .as_ref()
                .map(|m| m.as_ref())
                .unwrap_or("null");
        
            let mut file_path = PathBuf::from(format!(".{}",f_file_path));
            file_path.push(&sanitize_filename::sanitize(&file_name));
            match std::fs::rename(temp_file_path, file_path.clone()) {
                Ok(_) => {

                    // save the old profile pic path if the user have
                    let student = match delete_old_profile_pic(db.clone(), objId).await {
                        Ok(s) => s,
                        Err(e) =>{
                            return HttpResponse::BadRequest().json(
                                ResponseBuilder::<()>::FailedResponse(e.to_string())
                            )
                        },
                    };

                    match db.update_profile_pic(format!("{}{}",f_file_path, file_name), objId).await {
                        Ok(result) => {
                            if result.matched_count == 0 {
                                let _ = std::fs::remove_file(file_path);
                                return HttpResponse::BadRequest().json(
                                    ResponseBuilder::<()>::FailedResponse(Messages::DataUpdateFailed.to_string())
                                )
                            }

                            // check if old profile pic there then remove old file
                            if !student.profile_pic.is_none() {
                                let res = std::fs::remove_file(format!(".{}",student.profile_pic.unwrap().to_string()));
                                if res.is_err() {
                                    println!("Error while deleting a old")
                                }
                            }

                            HttpResponse::Ok().json(
                                ResponseBuilder::<()>::SuccessResponse(
                                    Messages::DataUpdateSuccess.to_string(),
                                    None
                                )
                            )
                        },
                        Err(e) => {
                            let _ = std::fs::remove_file(file_path);
                            HttpResponse::InternalServerError().json(
                                ResponseBuilder::<()>::FailedResponse(e.to_string())
                            )
                        },
                    }
                },
                Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
            }
        
        },
        Err(_) => {
            HttpResponse::BadRequest().json(
                ResponseBuilder::<()>::InValidIdResponse()
            )
        },
    }

}

// will check if the use have a profile already then delete old pic
#[allow(non_snake_case)]
pub async fn delete_old_profile_pic(db:Data<StudentRepo>,studentId:ObjectId) -> Result<Students, AppError> {
    match db.get_student(studentId).await {
        Ok(student) => {
            Ok(student)
        },
        Err(e) =>  Err(e),
    }
}

#[allow(non_snake_case)]
pub async fn add_parent(db:Data<StudentRepo>, request:Json<CreateParentDTO>) -> impl Responder {
    if !request.email.is_none() {
        if !validate_email(request.email.as_ref().unwrap().to_string()) {
            return HttpResponse::BadRequest().json(
                ResponseBuilder::<()>::FailedResponse("Invalid email".to_string())
            );
        }
    };

    match ObjectId::parse_str(request.student_id.to_string()) {
        Ok(objId) => {
            let parent = Parents {
                student_id: Some(objId),
                name: request.name.to_string(),
                address: request.address.to_string(),
                mobile_number: request.mobile_number,
                email: request.email.as_ref().unwrap().to_string(),
                created_at: Some(bson::DateTime::now()),
                updated_at: Some(bson::DateTime::now()),
            };

            match db.add_parent(parent).await {
                Ok(result) => {
                    if result.matched_count == 0 {
                        return HttpResponse::NotFound().json(
                            ResponseBuilder::<()>::FailedResponse(
                                format!("Student {}", AppError::DataNotFoundError.to_string())
                            )
                        );
                    };
                    HttpResponse::Ok().json(
                        ResponseBuilder::<()>::SuccessResponse(
                            format!("Parent {}",Messages::DataUpdateSuccess.to_string()),
                            None
                        )
                    )
                },
                Err(e) => {
                    HttpResponse::BadRequest().json(
                        ResponseBuilder::<()>::FailedResponse(e.to_string())
                    )
                },
            }
        },
        Err(_) => {
            HttpResponse::BadRequest().json(
                ResponseBuilder::<()>::InValidIdResponse()
            )
        },
    }

}

#[allow(non_snake_case)]
pub async fn update_student(db:Data<StudentRepo>, path:Path<String>, request:Json<CreateStudentDTO>) -> impl Responder {
    match ObjectId::parse_str(path.into_inner()) {
        Ok(objeId) => {
            match db.update_student(objeId, request.into_inner()).await {
                Ok(result) => {
                    if result.matched_count == 0 {
                        return HttpResponse::BadRequest().json(
                            ResponseBuilder::<()>::FailedResponse(Messages::DataUpdateFailed.to_string())
                        );
                    }

                    HttpResponse::Ok().json(
                        ResponseBuilder::<()>::SuccessResponse(
                            Messages::DataUpdateSuccess.to_string(),
                            None
                        )
                    )
                },
                Err(e) => {
                    HttpResponse::BadRequest().json(
                        ResponseBuilder::<()>::FailedResponse(e.to_string())
                    )
                },
            }
        },
        Err(_) => {
            HttpResponse::BadRequest().json(
                ResponseBuilder::<()>::InValidIdResponse()
            )
        },
    }
}


#[allow(non_snake_case)]
pub async fn get_pending_registration(db:Data<StudentRepo>) -> impl Responder {
    match db.pending_registration().await {
        Ok(students) => {
            
            if students.len() == 0 {
                return  HttpResponse::BadRequest().json(
                    ResponseBuilder::<()>::FailedResponse(AppError::DataNotFoundError.to_string())
                );
            }

            let mut student_dto:Vec<StudentsDTO> = Vec::new();

            for i in students {
                student_dto.push(StudentsDTO::init(i, "".to_string()));
            }

            HttpResponse::Ok().json(
                ResponseBuilder::SuccessResponse(
                    Messages::DataFetchSuccess.to_string(),
                    Some(student_dto)
                )
            )
        },
        Err(e) => {
            HttpResponse::BadRequest().json(
                ResponseBuilder::<()>::FailedResponse(e.to_string())
            )
        },
    }  
} 

#[allow(non_snake_case)]
pub async fn studnent_login(db:Data<StudentRepo>, path:Path<String>) -> impl Responder {
    let studentId = path.into_inner();

    if studentId.is_empty() {
        return HttpResponse::BadRequest().json(
            ResponseBuilder::<()>::FailedResponse("invalid params".to_string())
        );
    }

    match db.student_login(studentId).await {
        Ok(student) => {
            let access_token = jwt_service::JwtService::GenerateToken(&student);
            let studenDto = StudentsDTO::init(student, access_token);

            HttpResponse::Ok().json(
                ResponseBuilder::SuccessResponse(
                    Messages::DataFetchSuccess.to_string(),
                    Some(studenDto)
                )
            )

        },
        Err(e) => {
            HttpResponse::BadRequest().json(
                ResponseBuilder::<()>::FailedResponse(e.to_string())
            )
        },
    }
}

