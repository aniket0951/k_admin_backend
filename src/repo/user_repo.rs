use bson::{doc, oid::ObjectId, Document};
use futures::stream::TryStreamExt; //add this
use mongodb::{results::{DeleteResult, InsertOneResult, UpdateResult}, Collection, Database};

use crate::models;
use models::user_models::Users;
use crate::helper::app_errors::AppError;
use crate::dto::user_dto::*;

pub struct  UserRepo {
    user_col:Collection<Document>
}

impl UserRepo {
    
    pub async fn init(db:Database) -> Self {
        let user_col = db.collection("users");
        UserRepo { user_col }
    }

    pub async fn add_user(&self, user:Users) -> Result<InsertOneResult, AppError> {
        let bson_user = match user.to_docmunet(){
            Ok(bson_doc) => bson_doc,
            Err(e) => {
                return Err(AppError::CustomError(e.to_string()));
            },
        };

        match self.user_col.insert_one(bson_user, None).await {
            Ok(result_id) => return Ok(result_id),
            Err(e) => {
                Err(AppError::CustomError(e.to_string()))
            },
        }
    }

    pub async fn get_user_by_mail(&self, email:String) -> Result<Users, AppError> {

        let user = match self.user_col.find_one(doc! { "email":email }, None).await{
            Ok(Some(document)) => document,
            Ok(None) => return Err(AppError::DataNotFoundError),
            Err(e) => return Err(AppError::CustomError(e.to_string())),
        };

        bson::from_document(user).map_err(|e| AppError::CustomError(e.to_string()))
    
    }

    pub async fn get_users(&self) -> Result<Vec<Users>, AppError> {
        let mut result = match self.user_col.find(None, None).await {
            Ok(documets) => documets,
            Err(e) => return Err(AppError::CustomError(e.to_string())),
        };

        let mut users:Vec<Users> = Vec::new();

        while let Some(user) = result
            .try_next()
            .await
            .ok()
            .expect("Error mapping through the cursor")
        {
            users.push(bson::from_document(user).unwrap());
        }

        Ok(users)
    }

    pub async fn update_user(&self, user_id:ObjectId, user_data:UpdateUserRequestDTO) -> Result<UpdateResult, AppError> {
        let update = doc! {
            "$set": {
                "name":user_data.name,
                "email":user_data.email,
                "mobile_number":user_data.mobile_number,
                "is_active":user_data.is_active,
                "updated_at":bson::DateTime::now()
            }
        };

        match self.user_col.update_one(doc! {"_id":user_id}, update, None).await {
            Ok(result) => Ok(result),
            Err(e) => {
                return Err(AppError::CustomError(e.to_string()))
            },
        }
    } 

    #[allow(non_snake_case)]
    pub async fn delete_user(&self, userID:ObjectId) -> Result<DeleteResult, AppError> {
        
        match self.user_col.delete_one(doc! { "_id":userID }, None).await {
            Ok(result) => Ok(result),
            Err(e) => Err(AppError::CustomError(e.to_string())),
        }
    }

}