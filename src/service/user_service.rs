use bson::oid::ObjectId;

use actix_web::{web::{Data, Json, Path}, HttpResponse, Responder};
use bcrypt::{hash,verify, DEFAULT_COST};
use crate::dto::user_dto::*;
use validator::*;

use crate::{helper::{app_errors::Messages, response::ResponseBuilder}, models::user_models::{UserTypes, Users}, repo::user_repo::UserRepo};

use super::jwt_service;

pub async fn add_user(db:Data<UserRepo>, user:Json<Users>) -> impl Responder {
    
    let req_password = user.password.to_string();
    if req_password.is_empty() {
        let respose = ResponseBuilder::<()>::FailedResponse("password can't be empty".to_string());
        return HttpResponse::BadRequest().json(respose);
    };

    let pass = match hash(req_password, DEFAULT_COST) {
        Ok(pass) => pass,
        Err(_) => {
            let response = ResponseBuilder::<()>::FailedResponse("Somethink went's wrong".to_string());
            return HttpResponse::InternalServerError().json(response);
        },
    };

    if !validate_email(user.email.to_string()){
        let res = ResponseBuilder::<()>::FailedResponse("Invalid email".to_string());
        return HttpResponse::BadRequest().json(res);
    }

    let user_type = match user.user_type {
        UserTypes::ADMIN => UserTypes::ADMIN,
        UserTypes::SUBADMIN => UserTypes::SUBADMIN,
        UserTypes::ENDUSER => UserTypes::ENDUSER
    };

    let new_user = Users {
        id:None,
        name:user.name.to_string(),
        email:user.email.to_string(),
        mobile_number:user.mobile_number.to_string(),
        password:pass,
        user_type,
        is_active:user.is_active.to_owned(),
        created_at:Some(bson::DateTime::now()),
        updated_at:Some(bson::DateTime::now())
    };

    match db.add_user(new_user).await {
        Ok(result) => {
            let response = ResponseBuilder::SuccessResponse(Messages::DataAddedSuccess.to_string(), Some(result));
            HttpResponse::Ok().json(response)
        },
        Err(err) => {
            let response = ResponseBuilder::<()>::FailedResponse(err.to_string());
            HttpResponse::BadRequest().json(response)
        },
    }

}

#[allow(non_snake_case)]
pub async fn get_users(db:Data<UserRepo>) -> impl Responder {
    match db.get_users().await {
        Ok(users) =>{
            let mut userData:Vec<GetUserDTO> = Vec::new();

            for i in users {
                let temp = GetUserDTO::init(i, "".to_string());
                userData.push(temp)
            }

            let res = ResponseBuilder::SuccessResponse(Messages::DataFetchSuccess.to_string(), Some(userData));
            HttpResponse::Ok().json(res)
        },
        Err(err) => {
            let res = ResponseBuilder::<()>::FailedResponse(err.to_string());
            HttpResponse::BadRequest().json(res)
        },
    }
}

#[allow(non_snake_case)]
pub async fn update_user(db:Data<UserRepo>, path:Path<String>, userData:Json<UpdateUserRequestDTO> ) -> impl Responder {
    match ObjectId::parse_str(path.into_inner()) {
        Ok(obj_id) => {
            if !validate_email(&userData.email) || userData.mobile_number.is_empty() {
                return HttpResponse::BadRequest().json(
                    ResponseBuilder::<()>::FailedResponse("Invalid request params".to_string())
                )
            }

            match db.update_user(obj_id, userData.into_inner()).await {
                Ok(result) => {
                    println!("Match Count : {}", result.matched_count);
                    if result.matched_count == 0 {
                        return HttpResponse::BadRequest().json(
                            ResponseBuilder::<()>::FailedResponse(Messages::DataUpdateFailed.to_string())
                        );
                    }
                    HttpResponse::Ok().json(
                        ResponseBuilder::<()>::SuccessResponse(Messages::DataUpdateSuccess.to_string(), None)
                    )
                },
                Err(e) => {
                    HttpResponse::BadRequest().json(
                        ResponseBuilder::<()>::FailedResponse(e.to_string())
                    )
                },
            }
        },
        Err(_) =>{
            
            HttpResponse::BadRequest().json(
                ResponseBuilder::<()>::InValidIdResponse()
            )
        },
    }
}

#[allow(non_snake_case)]
pub async fn delete_user(db:Data<UserRepo>, path:Path<String>) -> impl Responder {
    match ObjectId::parse_str(path.into_inner()) {
        Ok(objId) => {
            match db.delete_user(objId).await {
                Ok(result) => {
                    if result.deleted_count == 0 {
                        return HttpResponse::UnprocessableEntity().json(
                            ResponseBuilder::<()>::FailedResponse(Messages::DataDeleteFailed.to_string())
                        );
                    }

                    HttpResponse::Ok().json(
                        ResponseBuilder::<()>::SuccessResponse(Messages::DataDeleteSucess.to_string(), None)
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
pub async fn login(db:Data<UserRepo>, request:Json<LoginRequestDTO>) -> impl Responder {
    
    if !validate_email(request.email.to_string()) || request.email.is_empty() || request.password.is_empty() {
        let res = ResponseBuilder::<()>::FailedResponse("Invalid request params".to_string());
        return HttpResponse::BadRequest().json(res);
    }
    let user = match db.get_user_by_mail(request.email.to_string()).await {
        Ok(result) => result,
        Err(e) => {
            let res = ResponseBuilder::<()>::FailedResponse(e.to_string());
            return HttpResponse::BadRequest().json(res);
        },
    };

    match verify(request.password.to_string(), &user.password) {
        Ok(true) => {},
        Ok(false) => {
            let res = ResponseBuilder::<()>::FailedResponse("password not matched".to_string());
            return HttpResponse::BadRequest().json(res);
        },
        Err(e) => {
            let res = ResponseBuilder::<()>::FailedResponse(e.to_string());
            return HttpResponse::BadRequest().json(res);
        },
    }
    
    let mut user_dto = GetUserDTO::init(user, String::new());
    let access_token = jwt_service::JwtService::GenerateToken(&user_dto);

    user_dto.access_token = Some(access_token);

    let res = ResponseBuilder::SuccessResponse(Messages::DataFetchSuccess.to_string(), Some(user_dto));
    HttpResponse::Ok().json(res)
}