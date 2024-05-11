use actix_web::{ web::{Data,Path, Json}, HttpResponse, Responder};
use bson::oid::ObjectId;

use crate::{dto::app_dto::{AppCountDTO, CreateBranchDTO, GetBranchDTO}, helper::{app_errors::{AppError, Messages}, response::ResponseBuilder}, models::app::Branches, repo::app_repo::AppRepo};


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