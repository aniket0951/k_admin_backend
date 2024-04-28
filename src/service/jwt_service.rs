use std::env;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation, errors::Error};

pub struct JwtService{
    secret_key:String,
    issued_at:i64,
    expired_at:i64
}

#[derive(Serialize, Deserialize, Default)]
pub struct TokenCliams<T>{
    user:Option<T>,
    iat:i64,
    exp:i64
}
#[allow(non_camel_case_types, non_snake_case)]

impl JwtService {
    
    fn init() -> Self  {

        dotenv().ok();

       let secret_key =  env::var("Jwt_Secrete_Key").unwrap();
        let now = Utc::now().timestamp();
        let expiration = now + 36000;

        JwtService {
            secret_key,
            issued_at: now,
            expired_at: expiration,
        }

    }

    pub fn GenerateToken<T>(user:&T) -> String
    where 
        T:serde::Serialize
    {
        let self_obj = Self::init();
            
        let tokenCliams = TokenCliams{
            user: Some(user),
            iat: self_obj.issued_at,
            exp: self_obj.expired_at,
        };

        let header = Header::new(jsonwebtoken::Algorithm::HS256);

        match encode(&header, &tokenCliams, &EncodingKey::from_secret(self_obj.secret_key.as_ref())) {
            Ok(token) => token,
            Err(e) => e.to_string(),
        }
    } 

    pub fn validate_token(token:&str) -> Result<TokenData<serde_json::Value>, Error> {
        let self_obj = Self::init();
        decode::<serde_json::Value>(&token, &DecodingKey::from_secret(self_obj.secret_key.as_ref()), &Validation::default())
    }

}
