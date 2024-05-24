use std::{path::PathBuf};
use actix_multipart::form::MultipartForm;

use actix_web::{ web::{Data,Path, Json}, HttpResponse, Responder};
use bson::oid::ObjectId;
use serde::{ser::SerializeStruct, Serialize};
use validator::Validate;

use crate::{dto::{app_dto::{ActiveCourseRequestDTO, AppCountDTO, CoursesDTO, CreateBranchDTO, CreateCourseDTO, CreateEnquiryDTO, CreateFacilities, CreateFeesDTO, EnquiriesDTO, FacilitiesDTO, FeesDTO, GetBranchDTO}, student_dto::UploadProfileDTO}, helper::{app_errors::{AppError, Messages}, response::ResponseBuilder}, models::app::{Branches, Courses, Enquiries, Facilities, Fees}, repo::app_repo::AppRepo};

use super::jwt_service;


pub async fn add_branch(db:Data<AppRepo>, request:Json<CreateBranchDTO>) -> impl Responder {
    if request.name.is_empty() || request.address.is_empty() {
        return HttpResponse::BadRequest().json(
            ResponseBuilder::<()>::FailedResponse("Invalid request params".to_string())
        );
    };

    let branch = Branches {
        id: None,
        name: request.name.to_string(),
        address: request.address.to_string(),
        is_active: request.is_active,
        created_at: bson::DateTime::now(),
        updated_at: bson::DateTime::now(),
    };

    match db.add_branch(branch).await {
        Ok(_) => {
            HttpResponse::Ok().json(
                ResponseBuilder::<()>::SuccessResponse(
                    Messages::DataAddedSuccess.to_string(),
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
}

pub async fn get_branches(db:Data<AppRepo>) -> impl Responder {
    match db.get_branches().await {
        Ok(branches) => {
            if branches.len() == 0 {
                return HttpResponse::NotFound().json(
                    ResponseBuilder::<()>::FailedResponse(format!("Branches {}", AppError::DataNotFoundError.to_string()))
                );
            }

            let mut branches_dto:Vec<GetBranchDTO> = Vec::new();

            for i in branches {
                branches_dto.push(GetBranchDTO::init(i))
            }

            HttpResponse::Ok().json(
                ResponseBuilder::SuccessResponse(
                    Messages::DataFetchSuccess.to_string(),
                    Some(branches_dto)
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
pub async fn update_branch(db:Data<AppRepo>, path:Path<String>, request:Json<CreateBranchDTO>) -> impl Responder {
    match ObjectId::parse_str(path.into_inner()) {
        Ok(objId) => {
            match db.update_branches(objId, request.into_inner()).await {
                Ok(result) => {
                    if result.matched_count == 0 {
                        return HttpResponse::NotFound().json(
                            ResponseBuilder::<()>::FailedResponse(
                                Messages::DataUpdateFailed.to_string()
                            )
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
pub async fn delete_branch(db:Data<AppRepo>, path:Path<String>) -> impl Responder {
    match ObjectId::parse_str(path.into_inner()) {
        Ok(objId) => {
            match db.delete_branch(objId).await {
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
pub async fn app_counts(db:Data<AppRepo>) -> impl Responder {
    // student counts
    let totalStudent =  match db.total_students().await   {
        Ok(count) => count,
        Err(_) => 0,
    };

    let lastMonthAdmission = match db.last_month_admission_count().await {
        Ok(count) => count,
        Err(_) => 0,
    };

    let totalBranches = match db.total_branches().await {
        Ok(count) => count,
        Err(_) => 0,
    };

    let totalEvents = match db.total_events().await {
        Ok(count) => count,
        Err(_) => 0,
    };

    let totalUpCommingEvents = match db.upcommint_event_count().await {
        Ok(count) => count,
        Err(_) => 0,
    };

    let result = AppCountDTO {
        totalStudent,
        lastMonthAdmission,
        totalBranches,
        upCommingEvents: totalUpCommingEvents,
        totalEvents
    };

    let response = ResponseBuilder::SuccessResponse(
        Messages::DataFetchSuccess.to_string(),
        Some(result)
    );

    HttpResponse::Ok().json(response)
}

#[allow(non_snake_case)]
pub async fn get_branch(db:Data<AppRepo>, path:Path<String>) -> impl Responder {
    match ObjectId::parse_str(path.into_inner()) {
        Ok(objId) => {
            match db.get_branch(objId).await {
                Ok(branch) => {
                    HttpResponse::Ok().json(
                        ResponseBuilder::SuccessResponse(
                            Messages::DataFetchSuccess.to_string(),
                            Some(GetBranchDTO::init(branch))
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
        Err(_) =>  {
            HttpResponse::BadRequest().json(
                ResponseBuilder::<()>::InValidIdResponse()
            )
        },
    }
}

#[allow(non_snake_case)]
pub async fn guest_access_token() -> impl Responder {
    #[derive(Serialize)]
    struct GeustUser {
        pub name:String,
        pub userType:String,
    };

    let guestUser = GeustUser{
        name: String::from("Guest"),
        userType: String::from("Guest"),
    };

    let accessToken = jwt_service::JwtService::GenerateToken(&guestUser);
    HttpResponse::Ok().json(
        ResponseBuilder::SuccessResponse(
            String::from("Access token has been generate for Guest User"),
            Some(accessToken)
        )
    )
}

#[allow(non_snake_case)]
pub async fn add_fee(db:Data<AppRepo>, fee:Json<CreateFeesDTO>) -> impl Responder {
    if fee.fee_amount == 0 {
        return HttpResponse::BadRequest().json(
            ResponseBuilder::<()>::FailedResponse("invalid fee amount".to_string())
        );
    }

    let feeModel = Fees {
        id: None,
        fee_type: fee.fee_type.to_string(),
        fee_amount: fee.fee_amount.into(),
        is_discount: false,
        fee_discount: fee.fee_discount.into(),
        created_at: bson::DateTime::now(),
        updated_at: bson::DateTime::now(),
    };

    match db.add_fee(feeModel).await {
        Ok(result) => {
            HttpResponse::Ok().json(
                ResponseBuilder::SuccessResponse(
                    Messages::DataAddedSuccess.to_string(),
                    Some(result)
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
pub async fn get_fee(db:Data<AppRepo>) -> impl Responder {
    match db.get_fee().await {
        Ok(fees) => {
            if fees.len() == 0 {
                return HttpResponse::NotFound().json(
                    ResponseBuilder::<()>::FailedResponse(AppError::DataNotFoundError.to_string())
                )
            }

            let mut feeDto:Vec<FeesDTO> = Vec::new();

            for i in fees {
                feeDto.push(FeesDTO::init(i))
            }

            HttpResponse::Ok().json(
                ResponseBuilder::SuccessResponse(
                    Messages::DataFetchSuccess.to_string(),
                    Some(feeDto)
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
pub async fn make_discount_Active(db:Data<AppRepo>, path:Path<String>) -> impl Responder {
    match ObjectId::parse_str(path.into_inner()) {
        Ok(objID) => {
            match db.make_discount_Active(objID).await {
                Ok(result) => {
                    if result.matched_count == 0 {
                        return HttpResponse::NotFound().json(
                            ResponseBuilder::<()>::FailedResponse(AppError::DataNotFoundError.to_string())
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
pub async fn delete_fee(db:Data<AppRepo>, path:Path<String>) -> impl Responder {
    match ObjectId::parse_str(path.into_inner()) {
        Ok(objId) => {
            match db.delete_fee(objId).await {
                Ok(result) => {
                    if result.deleted_count == 0 {
                        return HttpResponse::BadRequest().json(
                            ResponseBuilder::<()>::FailedResponse(
                                Messages::DataDeleteFailed.to_string()
                            )
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
                        ResponseBuilder::<()>::FailedResponse(
                            e.to_string()
                        )
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


// ------------------------------ COURSES ------------------------------------- //
pub async fn add_course(db:Data<AppRepo>, course:Json<CreateCourseDTO>) -> impl Responder {
    match course.validate() {
        Ok(_) => {

            let course_model = Courses {
                id: None,
                name: course.name.clone().unwrap().to_string(),
                description: course.description.clone().unwrap().to_string(),
                is_active: false,
                course_duration: course.course_duration.to_owned(),
                created_at: bson::DateTime::now(),
                updated_at: bson::DateTime::now(),
            };

            match db.add_course(course_model).await {
                Ok(result) => {
                    HttpResponse::Ok().json(
                        ResponseBuilder::SuccessResponse(
                            Messages::DataAddedSuccess.to_string(),
                            Some(result)
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
}

pub async fn list_course(db:Data<AppRepo>) -> impl Responder {
    match db.list_course().await {
        Ok(courses) => {
            if courses.len() == 0 {
                return HttpResponse::NotFound().json(
                    ResponseBuilder::<()>::FailedResponse(AppError::DataNotFoundError.to_string())
                );
            }

            let mut course_dto:Vec<CoursesDTO> = Vec::new();

            for i in courses {
                course_dto.push(CoursesDTO::init(i))
            }

            HttpResponse::Ok().json(
                ResponseBuilder::SuccessResponse(
                    Messages::DataFetchSuccess.to_string(),
                    Some(course_dto)
                )
            )
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(
                ResponseBuilder::<()>::FailedResponse(e.to_string())
            )
        },
    }
}

#[allow(non_snake_case)]
pub async fn active_course(db:Data<AppRepo>, args:Json<ActiveCourseRequestDTO>) -> impl Responder {
    match args.validate() {
        Ok(_) => {
            match ObjectId::parse_str(args.id.as_ref().unwrap().to_string()) {
                Ok(objId) => {
                    match db.active_course(args.isActive.unwrap().to_owned(), objId).await {
                        Ok(result) => {
                            if result.matched_count == 0 {
                                return HttpResponse::NotFound().json(
                                    ResponseBuilder::<()>::FailedResponse(AppError::DataNotFoundError.to_string())
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
        },
        Err(e) => {
            HttpResponse::BadRequest().json(
                ResponseBuilder::<()>::FailedResponse(e.to_string())
            )
        },
    }
}
#[allow(non_snake_case)]
pub async fn update_course(db:Data<AppRepo>, args:Json<CreateCourseDTO>) -> impl Responder {
    match args.validate() {
        Ok(_) => {
            match ObjectId::parse_str(args.id.clone()) {
                Ok(objId) =>{
                    match db.update_course(objId, args.into_inner()).await {
                        Ok(result) => {
                            if result.matched_count == 0 {
                                return HttpResponse::NotFound().json(
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
        },
        Err(e) => {
            HttpResponse::BadRequest().json(
                ResponseBuilder::<()>::FailedResponse(e.to_string())
            )
        },
    }
}

#[allow(non_snake_case)]
pub async fn delete_course(db:Data<AppRepo>, path:Path<String>) -> impl Responder {
    match ObjectId::parse_str(path.into_inner()) {
        Ok(objId) => {
            match db.delete_course(objId).await {
                Ok(result) =>{
                    if result.deleted_count == 0 {
                        return HttpResponse::NotFound().json(
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
pub async fn get_course(db:Data<AppRepo>, path:Path<String>) -> impl Responder {
    match ObjectId::parse_str(path.into_inner()) {
        Ok(objId) => {
            match db.get_course(objId).await {
                Ok(course) => {
                    HttpResponse::Ok().json(
                        ResponseBuilder::SuccessResponse(
                            Messages::DataFetchSuccess.to_string(),
                            Some(CoursesDTO::init(course))
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

// ------------------------------ FACILITIES ------------------------------------- //
#[allow(non_snake_case)]
pub async fn add_facilities(db:Data<AppRepo>, request:Json<CreateFacilities>) -> impl Responder {
    match request.validate() {
        Ok(_) => {
            let faciliti = Facilities {
                id: None,
                title: request.title.to_owned().unwrap(),
                description: request.description.to_owned().unwrap(),
                imageUrl: None,
                created_at:bson::DateTime::now(),
                updated_at:bson::DateTime::now(),
            };
            match db.add_facilities(faciliti).await {
                Ok(result) => {
                    HttpResponse::Ok().json(
                        ResponseBuilder::SuccessResponse(
                            Messages::DataAddedSuccess.to_string(),
                            Some(result)
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
}
#[allow(non_snake_case)]
pub async fn get_facilities(db:Data<AppRepo>, path:Path<String>) -> impl Responder {
    match ObjectId::parse_str(path.into_inner()) {
        Ok(objId) => {
            match db.get_facilities(objId).await {
                Ok(document) => {
                    HttpResponse::Ok().json(
                        ResponseBuilder::SuccessResponse(
                            Messages::DataFetchSuccess.to_string(),
                            Some(FacilitiesDTO::init(document))
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
pub async fn list_facilities(db:Data<AppRepo>) -> impl Responder {
    match db.list_facilities().await {
        Ok(facilities) => {
            if facilities.len() == 0 {
                return  HttpResponse::NotFound().json(
                    ResponseBuilder::<()>::FailedResponse(AppError::DataNotFoundError.to_string())
                );
            }
            let mut facilitiesDTO:Vec<FacilitiesDTO> = Vec::new();
            for i in facilities {
                facilitiesDTO.push(FacilitiesDTO::init(i))
            }

            HttpResponse::Ok().json(
                ResponseBuilder::SuccessResponse(
                    Messages::DataFetchSuccess.to_string(),
                    Some(facilitiesDTO)
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
pub async fn update_facilities(db:Data<AppRepo>, path:Path<String>, facility:Json<CreateFacilities>) -> impl Responder {
    match ObjectId::parse_str(path.into_inner()) {
        Ok(objID) => {
            let isImage = !facility.image_url.is_none();
            match db.update_facilities(objID, facility.into_inner(), isImage).await {
                Ok(result) => {
                    if result.matched_count == 0 {
                        return HttpResponse::BadRequest().json(
                            ResponseBuilder::<()>::FailedResponse(AppError::DataNotFoundError.to_string())
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
pub async fn upload_facility_image(db:Data<AppRepo>,path:Path<String> , payload:MultipartForm<UploadProfileDTO>) -> impl Responder {
    match ObjectId::parse_str(path.into_inner()) {
        Ok(objId) => {
                    
            let f_file_path = format!("/static/facilities/");
        
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
                    let student = match delete_old_facility_pic(db.clone(), objId).await {
                        Ok(s) => s,
                        Err(e) =>{
                            return HttpResponse::BadRequest().json(
                                ResponseBuilder::<()>::FailedResponse(e.to_string())
                            )
                        },
                    };
                    let facility = CreateFacilities {
                        title: None,
                        description: None,
                        image_url: Some(format!("{}{}",f_file_path, file_name)),
                    };
                    match db.update_facilities(objId, facility, true).await {
                        Ok(result) => {
                            if result.matched_count == 0 {
                                let _ = std::fs::remove_file(file_path);
                                return HttpResponse::BadRequest().json(
                                    ResponseBuilder::<()>::FailedResponse(Messages::DataUpdateFailed.to_string())
                                )
                            }

                            // check if old profile pic there then remove old file
                            if !student.imageUrl.is_none() {
                                let res = std::fs::remove_file(format!(".{}",student.imageUrl.unwrap().to_string()));
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
#[allow(non_snake_case)]
pub async fn delete_facility(db:Data<AppRepo>, path:Path<String>) -> impl Responder {
    match ObjectId::parse_str(path.into_inner()) {
        Ok(objId) => {
            match db.delete_facility(objId).await {
                Ok(result) => {
                    if result.deleted_count  == 0 {
                        return HttpResponse::BadRequest().json(
                            ResponseBuilder::<()>::FailedResponse(AppError::DataNotFoundError.to_string())
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
pub async fn delete_old_facility_pic(db:Data<AppRepo>,facilityID:ObjectId) -> Result<Facilities, AppError> {
    match db.get_facilities(facilityID).await {
        Ok(facilities) => {
            Ok(facilities)
        },
        Err(e) =>  Err(e),
    }
}

// ------------------------------ ENQUIRES ------------------------------------- //

pub async fn add_enquiry(db:Data<AppRepo>, enquire:Json<CreateEnquiryDTO>) -> impl Responder {
    let enquire_m = Enquiries {
        id: None,
        name: enquire.name.to_string(),
        email: enquire.email.to_string(),
        contact: enquire.contact.to_string(),
        subject: enquire.subject.to_string(),
        message: enquire.message.to_string(),
        created_at: bson::DateTime::now(),
        updated_at: bson::DateTime::now(),
    };
    match db.add_enquiry(enquire_m).await {
        Ok(result) => {
            HttpResponse::Ok().json(
                ResponseBuilder::SuccessResponse(
                    Messages::DataAddedSuccess.to_string(),
                    Some(result)
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
pub async fn list_enquires(db:Data<AppRepo>) -> impl Responder {
    match db.list_enquires().await {
        Ok(enquires) => {
            if enquires.len() == 0 {
                return  HttpResponse::NotFound().json(
                    ResponseBuilder::<()>::FailedResponse(AppError::DataNotFoundError.to_string())
                );
            }
            let mut enquiriesDTO:Vec<EnquiriesDTO> = Vec::new();
            for i in enquires {
                enquiriesDTO.push(EnquiriesDTO::init(i))
            }

            HttpResponse::Ok().json(
                ResponseBuilder::SuccessResponse(
                    Messages::DataFetchSuccess.to_string(),
                    Some(enquiriesDTO)
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



