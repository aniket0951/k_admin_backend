use std:: io::Write;
use actix_multipart::Multipart;
use actix_web::{web::{Data, Json, Path}, Handler, HttpResponse, Responder};
use bson::oid::ObjectId;
use chrono::{DateTime, Datelike, NaiveDateTime, Utc};
use futures::{StreamExt, TryStreamExt};
use crate::{dto::event_dto::{CreateEventDTO, CreateFileDataDTO, GetEventsDTO, UpdateEventDTO}, helper::{app_errors::{AppError, Messages}, response::ResponseBuilder}, models::events::{Events, FileData}, repo::events_repo::EventRepo};


pub async fn add_event(db:Data<EventRepo>, request:Json<CreateEventDTO>) -> impl Responder {
    let start_date =  match NaiveDateTime::parse_from_str(&request.start_date, "%Y%m%d%H%M") {
        Ok(datetime) => {
            let datetime_utc: DateTime<Utc> = chrono::TimeZone::from_utc_datetime(&Utc, &datetime);
            let bd = mongodb::bson::DateTime::builder().year(datetime_utc.year()).month(datetime_utc.month().try_into().unwrap()).day(datetime_utc.day().try_into().unwrap()).build().unwrap();
            bd
        },
        Err(_) => {
           return  HttpResponse::BadRequest().json(
                ResponseBuilder::<()>::FailedResponse("Invalid event date".to_string())
            );
        },
    };

    let end_date = match NaiveDateTime::parse_from_str(&request.end_date, "%Y%m%d%H%M") {
        Ok(datetime) => {
            let datetime_utc: DateTime<Utc> = chrono::TimeZone::from_utc_datetime(&Utc, &datetime);
            let bd = mongodb::bson::DateTime::builder().year(datetime_utc.year()).month(datetime_utc.month().try_into().unwrap()).day(datetime_utc.day().try_into().unwrap()).build().unwrap();
            bd
        },
        Err(_) => {
            return  HttpResponse::BadRequest().json(
                ResponseBuilder::<()>::FailedResponse("Invalid event date".to_string())
            );
        },
    };

    if end_date < start_date {
        return HttpResponse::BadRequest().json(
            ResponseBuilder::<()>::FailedResponse("end date should be next date of start date".to_string())
        );
    }

    
    let event = Events {
        id: None,
        title: request.title.to_string(),
        discription: request.discription.to_string(),
        location: request.location.to_string(),
        file_data: None,
        start_date: Some(start_date),
        end_date: Some(end_date),
        created_at: Some(bson::DateTime::now()),
        updated_at: Some(bson::DateTime::now()),
        is_active: Some(true),
    };

    match db.add_event(event).await {
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

#[allow(non_snake_case)]
pub async fn add_file_data(db:Data<EventRepo>, path:Path<String>, mut payload:Multipart) -> impl Responder {
    match ObjectId::parse_str(path.into_inner()) {
        Ok(objId) => {
            let mut file_type: Option<String> = None;
            let mut file_data: Option<Vec<u8>> = None;
            let mut filename: Option<String> = None;

            while let Ok(Some(mut field)) = payload.try_next().await {
                let cd = field.content_disposition();
                let name = cd.get_name().unwrap();

                if name == "file_type" {
                    let mut data = Vec::new();
                    while let Some(chunk) = field.next().await {
                        data.extend_from_slice(&chunk.unwrap())
                    }
                    file_type = Some(String::from_utf8_lossy(&data).to_string())
                }else {
                    filename = cd.get_filename().map(|f| f.to_string());
                    let mut data = Vec::new();
                    while let Some(chunk) = field.next().await {
                        data.extend_from_slice(&chunk.unwrap());
                    }
                    file_data = Some(data);
                }
            }

            // prepare for the request
            let file_path = format!("/static/event/{}", filename.unwrap()); 

            match std::fs::File::create(format!(".{}", file_path)){
                Ok(mut file) => {
                    let err = file.write_all(&file_data.unwrap());

                    if err.is_err() {
                        return HttpResponse::BadRequest().json(
                            ResponseBuilder::<()>::FailedResponse(
                                Messages::DataUpdateFailed.to_string()
                            )
                        )
                    };
                    

                    let fileData = FileData {
                        file_type: file_type.unwrap(),
                        file_path:file_path.clone(),
                        created_at: Some(bson::DateTime::now()),
                    };
        
                    match db.add_file_data(objId, fileData).await {
                        Ok(_) => {
                            HttpResponse::Ok().json(
                                ResponseBuilder::<()>::SuccessResponse(
                                    Messages::DataUpdateSuccess.to_string(),
                                    None
                                )
                            )
                        },
                        Err(e) => {
                            _ = std::fs::remove_file(format!(".{}", file_path));
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
}

pub async fn get_events(db:Data<EventRepo>, path:Path<(i64, i64)>) -> impl Responder {
    let (skip,limit) = path.into_inner();
    match db.get_events(skip, limit).await {
        Ok(events) => {
            if events.len() == 0 {
                return HttpResponse::NotFound().json(
                    ResponseBuilder::<()>::FailedResponse(format!("Event {}", AppError::DataNotFoundError.to_string()))
                );
            }

            let mut event_dto:Vec<GetEventsDTO> = Vec::new();

            for i in events {
                event_dto.push(GetEventsDTO::init(i))
            }

            HttpResponse::Ok().json(
                ResponseBuilder::SuccessResponse(
                    Messages::DataFetchSuccess.to_string(),
                    Some(event_dto)
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
pub async fn add_video_link(db:Data<EventRepo>, path:Path<String>, requestData:Json<CreateFileDataDTO>) -> impl Responder {
    match ObjectId::parse_str(path.into_inner()) {
        Ok(objId) => {

            let fileData = FileData {
                file_type: requestData.file_type.to_owned(),
                file_path: requestData.file_path.to_owned(),
                created_at: Some(bson::DateTime::now()),
            };

            match db.add_file_data(objId, fileData).await {
                Ok(updateResult) => {
                    if updateResult.matched_count == 0 {
                        return HttpResponse::NotFound().json(
                            ResponseBuilder::<()>::FailedResponse(AppError::DataNotFoundError.to_string())
                        )
                    }

                    HttpResponse::Ok().json(
                        ResponseBuilder::<()>::SuccessResponse(
                            Messages::DataUpdateSuccess.to_string(),
                            None,
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
pub async fn get_event(db:Data<EventRepo>, path:Path<String>) -> impl Responder {
    match ObjectId::parse_str(path.into_inner()) {
        Ok(objId) => {
            match db.get_event(objId).await {
                Ok(event) => {
                    let evenet_dto = GetEventsDTO::init(event);
                    HttpResponse::Ok().json(
                        ResponseBuilder::SuccessResponse(
                            Messages::DataFetchSuccess.to_string(),
                            Some(evenet_dto)
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
pub async fn delete_event(db:Data<EventRepo>, path:Path<String>) -> impl Responder {
    match ObjectId::parse_str(path.into_inner()) {
        Ok(objId) =>{
            match db.delete_event(objId).await {
                Ok(result) =>{
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
pub async fn update_event(db:Data<EventRepo>, path:Path<String>, mut request:Json<UpdateEventDTO>) -> impl Responder {
    match ObjectId::parse_str(path.into_inner()) {
        Ok(objId) => {
            let start_date =  match NaiveDateTime::parse_from_str(&request.start_date, "%Y%m%d%H%M") {
                Ok(datetime) => {
                    let datetime_utc: DateTime<Utc> = chrono::TimeZone::from_utc_datetime(&Utc, &datetime);
                    datetime_utc
                },
                Err(_) => {
                   return  HttpResponse::BadRequest().json(
                        ResponseBuilder::<()>::FailedResponse("Invalid event date".to_string())
                    );
                },
            };
        
            let end_date = match NaiveDateTime::parse_from_str(&request.end_date, "%Y%m%d%H%M") {
                Ok(datetime) => {
                    let datetime_utc: DateTime<Utc> = chrono::TimeZone::from_utc_datetime(&Utc, &datetime);
                    datetime_utc
                },
                Err(_) => {
                    return  HttpResponse::BadRequest().json(
                        ResponseBuilder::<()>::FailedResponse("Invalid event date".to_string())
                    );
                },
            };

            request.start_date = start_date.to_string();
            request.end_date = end_date.to_string();

            match db.update_event(objId, request.into_inner()).await {
                Ok(result) => {
                    if result.matched_count == 0 {
                        return HttpResponse::BadRequest().json(
                            ResponseBuilder::<()>::FailedResponse(format!("Event {}", AppError::DataNotFoundError.to_string()))
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

pub async fn total_event(db:Data<EventRepo>) -> impl Responder {
    match db.total_event().await {
        Ok(count) => {
            HttpResponse::Ok().json(
                ResponseBuilder::SuccessResponse(
                    Messages::DataFetchSuccess.to_string(),
                    Some(count)
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