use bson::{doc, oid::ObjectId, Document};
use futures::TryStreamExt;
use mongodb::{options, results::{DeleteResult, InsertOneResult, UpdateResult}, Collection, Database};

use crate::{dto::app_dto::CreateBranchDTO, helper::app_errors::AppError, models::app::Branches};


pub struct AppRepo {
    branch_col:Collection<Document>
}
#[allow(non_snake_case)]
impl AppRepo {

    pub fn init(db:Database) -> Self {
        let branch_col = db.collection("branches");
        AppRepo{ branch_col }
    }

    pub async fn add_branch(&self, branch:Branches) -> Result<InsertOneResult, AppError> {
        let branch_bson = match branch.to_document() {
            Ok(document) => document,
            Err(e) => return Err(AppError::CustomError(e.to_string())),
        };

        match self.branch_col.insert_one(branch_bson, None).await {
            Ok(result) => Ok(result),
            Err(e) => Err(AppError::CustomError(e.to_string())),
        }
    }

    pub async fn get_branches(&self) -> Result<Vec<Branches>, AppError> {
        let opt = options::FindOptions::builder()
            .sort(doc!{"created_at":-1})
            .build();

        let mut cursor = match self.branch_col.find( None, opt).await {
            Ok(result) => result,
            Err(e) => return Err(AppError::CustomError(e.to_string())),
        };

        let mut brancehs:Vec<Branches> = Vec::new();
        while let Some(branch) = cursor 
            .try_next()
            .await
            .ok()
            .expect("Mapping Error")
        {
            brancehs.push(bson::from_document(branch).unwrap())
        }

        Ok(brancehs)
    }

    pub async fn update_branches(&self, branchId:ObjectId, branch:CreateBranchDTO) -> Result<UpdateResult, AppError> {
        let update = doc! {
            "$set": {
                "name":branch.name,
                "address":branch.address,
                "is_active":branch.is_active,
                "updated_at": bson::DateTime::now()
            }
        };

        match self.branch_col.update_one(doc! { "_id":branchId }, update, None).await {
            Ok(result) => Ok(result),
            Err(e) => Err(AppError::CustomError(e.to_string())),
        }
    } 

    pub async fn delete_branch(&self, branchId:ObjectId) -> Result<DeleteResult, AppError> {
        match self.branch_col.delete_one(doc! { "_id":branchId }, None).await {
            Ok(result) => Ok(result),
            Err(e) => Err(AppError::CustomError(e.to_string())),
        }
    }

    pub async fn get_branch(&self, branchId:ObjectId) -> Result<Branches, AppError> {
        let branch = match self.branch_col.find_one(doc! { "_id":branchId }, None).await {
            Ok(Some(branch)) => branch,
            Ok(None) => return Err(AppError::DataNotFoundError),
            Err(e) => return Err(AppError::CustomError(e.to_string())),
        };

        bson::from_document(branch).map_err(|e| AppError::CustomError(e.to_string()))
    }


}