use std::result;

use actix_web::App;
use bson::{doc, document, oid::ObjectId, Document};
use futures::TryStreamExt;
use mongodb::{options::{self, IndexOptions}, results::{DeleteResult, InsertOneResult, UpdateResult}, Collection, Database, IndexModel, error::ErrorKind};

use crate::{dto::app_dto::{CreateBranchDTO, CreateCourseDTO}, helper::app_errors::AppError, models::app::{Branches, Courses, Fees}, StudentRepo};

use super::events_repo::EventRepo;

#[allow(non_snake_case)]
pub struct AppRepo {
    branch_col:Collection<Document>,
    fees_col:Collection<Document>,
    course_col:Collection<Document>,
    studentRepo:StudentRepo,
    eventRepo:EventRepo,
    
}
#[allow(non_snake_case)]
impl AppRepo {
    pub async fn init(db:Database, studentRepo:StudentRepo, eventRepo:EventRepo) -> Self {
        let branch_col = db.collection("branches");
        let fees_col = db.collection("fees_col");
        let course_col = db.collection("courses");

        Self::createUniqueIndex(course_col.clone(), "name".to_string(), true).await;

        AppRepo{ branch_col, fees_col ,studentRepo, eventRepo, course_col }
    }

    pub async fn createUniqueIndex(collection:Collection<Document>, filedName:String,isUnique:bool) {
        let index_model = IndexModel::builder()
            .keys(doc! { filedName.clone():1})
            .options(IndexOptions::builder().unique(isUnique).build())
            .build();

        let result =  collection.create_index(index_model, None).await;
        if result.is_err() {
            println!("Index is not create on collection index filed {:?}",result.err());
        }else {
            println!("Index has been created")
        }
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

    pub async fn total_branches(&self) -> Result<u64, AppError> {
        match self.branch_col.count_documents(None, None).await {
            Ok(count) => Ok(count),
            Err(e) => Err(AppError::CustomError(e.to_string())),
        }
    }

    pub async fn total_students(&self) -> Result<u64, AppError> {
        let result = self.studentRepo.total_students().await;
        Ok(result)
    }

    pub async fn last_month_admission_count(&self) -> Result<u64, AppError> {
        let result = self.studentRepo.last_month_admission_count().await;
        Ok(result)
    }

    pub async fn total_events(&self) -> Result<u64, AppError> {
        let result = self.eventRepo.total_event().await;
        result
    }

    pub async fn upcommint_event_count(&self) -> Result<u64, AppError> {
        self.eventRepo.upcommint_event_count().await
    }

    // ------------------------------ FEES ------------------------------------ //
    pub async fn add_fee(&self, fees:Fees) -> Result<InsertOneResult, AppError> {
        let bson_document = match fees.to_document() {
            Ok(document) => document,
            Err(e) => return Err(AppError::CustomError(e.to_string())),
        };

        match self.fees_col.insert_one(bson_document, None).await {
            Ok(result) => Ok(result),
            Err(e) => Err(AppError::CustomError(e.to_string())),
        }
    }

    pub async fn get_fee(&self) -> Result<Vec<Fees>, AppError> {
        let mut cursor = match self.fees_col.find(None, None).await {
            Ok(cursor) => cursor,
            Err(e) => return Err(AppError::CustomError(e.to_string())),
        };

        let mut fees:Vec<Fees> = Vec::new();

        while let Some(fee) = cursor 
        .try_next()
        .await
        .ok()
        .expect("Mapping Error")

        {
            fees.push(bson::from_document(fee).unwrap())   
        }

        Ok(fees)
    }

    pub async fn make_discount_Active(&self, feeId:ObjectId) -> Result<UpdateResult, AppError> {
        let update = doc! {
            "$set": {
                "is_discount":true,
            }
        };
        
        match self.fees_col.update_one(doc! { "_id":feeId }, update, None).await {
            Ok(result) => Ok(result),
            Err(e) => Err(AppError::CustomError(e.to_string())),
        }
    } 

    pub async fn delete_fee(&self, feeId:ObjectId) -> Result<DeleteResult, AppError> {
        match self.fees_col.delete_one(doc! { "_id":feeId }, None).await {
            Ok(result) => Ok(result),
            Err(e) => Err(AppError::CustomError(e.to_string())),
        }
    }

    // ------------------------------- COURSES ------------------------------------- //
    pub async fn add_course(&self, course:Courses) -> Result<InsertOneResult, AppError> {
        let bson_doc = match course.to_document() {
            Ok(document) => document,
            Err(e) => return Err(AppError::CustomError(e.to_string())),
        };

        match self.course_col.insert_one(bson_doc, None).await {
            Ok(result) => {
                Ok(result)
            },
            Err(err) => match *err.kind {
                ErrorKind::Write(mongodb::error::WriteFailure::WriteError(ref write_error))
                    if write_error.code == 11000 =>{
                    return Err(AppError::CustomError("course already exists".to_string()));
                }
                _ => {
                    Err(AppError::CustomError(err.to_string()))
                }
            },
        }
    }

    pub async fn list_course(&self) -> Result<Vec<Courses>, AppError> {
        let opt = options::FindOptions::builder()
            .sort(doc! { "updated_at": -1})
            .build();

        let mut cursor = match self.course_col.find( None, opt).await {
            Ok(cursor) => cursor,
            Err(e) => return Err(AppError::CustomError(e.to_string())),
        };

        let mut courses:Vec<Courses> = Vec::new();

        while let Some(course) = cursor 
            .try_next()
            .await
            .ok()
            .expect("Mapping Error")
        {
            courses.push(bson::from_document(course).unwrap())
        }

        Ok(courses)
    }

    pub async fn active_course(&self, active:bool, courseId:ObjectId) -> Result<UpdateResult, AppError> {
        let update = doc! {
            "$set": {
                "is_active":active,
                "updated_at":bson::DateTime::now(),
            }
        };

        match self.course_col.update_one(doc! { "_id": courseId }, update, None).await {
            Ok(result) => Ok(result),
            Err(e) => {
                Err(AppError::CustomError(e.to_string()))
            },
        }
    }

    pub async fn update_course(&self, courseId:ObjectId, course:CreateCourseDTO) -> Result<UpdateResult, AppError> {
        let update = doc! {
            "$set" : {
                "name":course.name,
                "description":course.description,
                "course_duration":course.course_duration,
                "updated_at":bson::DateTime::now()
            }
        };

        match self.course_col.update_one(doc! { "_id":courseId }, update, None).await {
            Ok(result) => Ok(result),
            Err(err) => match *err.kind {
                ErrorKind::Write(mongodb::error::WriteFailure::WriteError(ref write_error))
                    if write_error.code == 11000 =>{
                    return Err(AppError::CustomError("failed to update duplicate course".to_string()));
                }
                _ => {
                    Err(AppError::CustomError(err.to_string()))
                }
            },
        }
    } 


}