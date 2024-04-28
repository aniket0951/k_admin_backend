use std::io::Write;

use actix_multipart::Multipart;
use actix_web::{ web::{Data, Path ,Json}, HttpResponse, Responder};
use bson::oid::ObjectId;
use futures::{StreamExt, TryStreamExt};
use validator::validate_email;

use crate::{dto::student_dto::{CreateParentDTO, CreateStudentDTO, StudentsDTO}, helper::{app_errors::{AppError, Messages}, response::ResponseBuilder}, models::student_model::{Parents, Students}, repo::student_repo::StudentRepo};

pub async fn add_student(db:Data<StudentRepo>, request:Json<CreateStudentDTO>) -> impl Responder {
    if request.name.is_empty() || request.class_branch.is_empty() {
        return HttpResponse::BadRequest().json(
            ResponseBuilder::<()>::FailedResponse("Invalid request params".to_string())
        );
    };

    let student = Students {
        id: None,
        name: request.name.to_string(),
        age: request.age,
        date_of_birth: request.date_of_birth.to_string(),
        address: request.address.to_string(),
        is_active_student: true,
        class_branch: Some(request.class_branch.to_string()),
        parent: None,
        created_at: Some(bson::DateTime::now()),
        updated_at: Some(bson::DateTime::now()),
        profile_pic: None,
    };

    match db.add_student(student).await {
        Ok(result) => {
            HttpResponse::Ok().json(
                ResponseBuilder::SuccessResponse(Messages::DataAddedSuccess.to_string(), Some(result))
            )
        },
        Err(e) => {
            HttpResponse::BadRequest().json(
                ResponseBuilder::<()>::FailedResponse(e.to_string())
            )
        },
    }
}

pub async fn get_students(db:Data<StudentRepo>) -> impl Responder {
    match db.get_students().await {
        Ok(students) => {
            if students.len() == 0 {
                return HttpResponse::NotFound().json(
                    ResponseBuilder::<()>::FailedResponse(Messages::DataFetchFailed.to_string())
                );
            }

            let mut students_dto:Vec<StudentsDTO> = Vec::new();

            for i in students {
                students_dto.push(StudentsDTO::init(i))
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
pub async fn upload_profile(db:Data<StudentRepo>, mut payload:Multipart) -> impl Responder {
    let mut text_data: Option<String> = None;
    let mut file_data: Option<Vec<u8>> = None;
    let mut filename: Option<String> = None;

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();
        let name = content_disposition.get_name().unwrap();

        if name == "userid" {
            // Process text field data
            let mut data = Vec::new();
            while let Some(chunk) = field.next().await {
                data.extend_from_slice(&chunk.unwrap());
            }
            text_data = Some(String::from_utf8_lossy(&data).to_string());
        } else {
            // Process file data
            let content_disposition = field.content_disposition();
            filename = content_disposition.get_filename().map(|f| f.to_string());
            let mut data = Vec::new();
            while let Some(chunk) = field.next().await {
                data.extend_from_slice(&chunk.unwrap());
            }
            file_data = Some(data);
        }
    }

    // checkt the validation for Object ID
    match ObjectId::parse_str(text_data.unwrap()) {
        Ok(objId) => {
            let  file_path = format!("/static/student/{}", filename.unwrap());

            match std::fs::File::create(format!(".{}",file_path.clone())) {
                Ok(mut file) => {
                    let err = file.write_all(&file_data.unwrap());
                    if err.is_err() {
                        return HttpResponse::BadRequest().json(
                            ResponseBuilder::<()>::FailedResponse(
                                Messages::DataUpdateFailed.to_string()
                            )
                        )
                    }

                    match db.update_profile_pic(file_path.clone() , objId).await {
                        Ok(result) => {
                            if result.matched_count == 0 {
                                _ = std::fs::remove_file(file_path);
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

    // HttpResponse::Ok().body("File(s) uploaded successfully")

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

