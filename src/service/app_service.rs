use std::result;

use actix_web::{ web::{Data,Path, Json}, HttpResponse, Responder};
use bson::oid::ObjectId;
use serde::{ser::SerializeStruct, Serialize};
use validator::Validate;

use crate::{dto::app_dto::{ActiveCourseRequestDTO, AppCountDTO, CoursesDTO, CreateBranchDTO, CreateCourseDTO, CreateFeesDTO, FeesDTO, GetBranchDTO}, helper::{app_errors::{AppError, Messages}, response::ResponseBuilder}, models::app::{Branches, Courses, Fees}, repo::app_repo::AppRepo};

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

