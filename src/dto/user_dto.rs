
use serde::{Deserialize, Serialize};

use crate::models::user_models::Users;
#[derive(Serialize, Deserialize)]
pub struct CreateUserRequestDTO {
    pub name:String,
    pub email:String,
    pub mobile_number:String,
    pub user_type:String,
    pub password:String,
    pub is_active:bool,
}

#[derive(Serialize,Deserialize, Default)]
pub struct UpdateUserRequestDTO {
    pub name:String,
    pub email:String,
    pub mobile_number:String,
    pub is_active:bool
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequestDTO {
    pub email:String,
    pub password:String
}

#[derive(Serialize, Deserialize)]
pub struct GetUserDTO {
    pub id:String,
    pub name:String,
    pub email:String,
    pub mobile_number:String,
    pub user_type:String,
    pub is_active:bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub access_token:Option<String>,
    pub created_at:Option<String>,
    pub updated_at:Option<String>
}

impl GetUserDTO {
    
    pub fn init(user:Users, access_token:String) -> Self {

        let access_token = Some(access_token).filter(|s| !s.is_empty());

        GetUserDTO {
            id: user.id.unwrap().to_string(),
            name: user.name,
            email: user.email,
            mobile_number: user.mobile_number,
            user_type: user.user_type.to_string(),
            is_active: user.is_active,
            created_at: Some(user.created_at.unwrap().to_string()),
            updated_at: Some(user.updated_at.unwrap().to_string()),
            access_token,
        }
    }
}