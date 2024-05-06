
use actix_web::App;
use bson::{doc, oid::ObjectId, Document};
use mongodb::{ options, results::{DeleteResult, InsertOneResult, UpdateResult}, Collection, Database};
use crate::{dto::{app_dto::CreateBranchDTO, student_dto::CreateStudentDTO}, helper::app_errors::AppError, models::student_model::{Parents, Students}};
use futures::stream::TryStreamExt; 
pub struct StudentRepo {
    student_col:Collection<Document>
}

#[allow(non_snake_case)]
impl StudentRepo {
    
    pub fn init(db:Database) -> Self {
        let student_col = db.collection("students");
        StudentRepo { student_col }
    }

    pub async fn add_student(&self, student:Students) -> Result<InsertOneResult, AppError> {
        let bson_user = match student.to_docmunet() {
            Ok(user) => user,
            Err(e) => return Err(AppError::CustomError(e.to_string())),
        };

        match self.student_col.insert_one(bson_user, None).await {
            Ok(result) => Ok(result),
            Err(e) => Err(AppError::CustomError(e.to_string())),
        }
    }

    pub async fn get_student(&self, studentId:ObjectId) -> Result<Students, AppError> {
        let result = match self.student_col.find_one(doc! { "_id":studentId}, None).await {
            Ok(Some(docResult)) => docResult,
            Ok(None) => return Err(AppError::DataNotFoundError),
            Err(e) => return Err(AppError::CustomError(e.to_string())),
        };

        bson::from_document(result).map_err(|e| AppError::CustomError(e.to_string()))
    }

    pub async fn update_profile_pic(&self, filePath:String, userId:ObjectId) -> Result<UpdateResult, AppError> {
        let update = doc! {
            "$set" : {
                "profile_pic":filePath,
                "created_at":bson::DateTime::now()
            }
        };
        match self.student_col.update_one(doc! { "_id": userId }, update, None).await {
            Ok(result) => Ok(result),
            Err(e) => Err(AppError::CustomError(e.to_string())),
        }
    }

    pub async fn get_students(&self, skip:i64, limit:i64) -> Result<Vec<Students>, AppError> {
        let opt = options::FindOptions::builder()
            .sort(doc!{"created_at":-1})
            .skip(skip as u64)
            .limit(limit)
            .build();
        let mut cursor = match self.student_col.find(None, opt).await{
            Ok(document) => document,
            Err(e) => return Err(AppError::CustomError(e.to_string())),
        };

        let mut students:Vec<Students> = Vec::new();

        while let Some(student) = cursor
            .try_next()
            .await
            .ok()
            .expect("Mapping Error")
         {
            students.push(bson::from_document(student).unwrap())
        }

        Ok(students)
    }

    pub async fn delete_student(&self, studentId:ObjectId) -> Result<DeleteResult, AppError> {
        match self.student_col.delete_one(doc! { "_id":studentId }, None).await {
            Ok(result) => Ok(result),
            Err(e) => Err(AppError::CustomError(e.to_string())),
        }
    }

    pub async fn add_parent(&self, parent:Parents) -> Result<UpdateResult, AppError> {
        let update = doc! {
            "$set": {
                "parent":parent.to_docmunet().unwrap(),
                "created_at":bson::DateTime::now()
            }
        };

        match self.student_col.update_one(doc! {"_id":parent.student_id.unwrap()}, update, None).await {
            Ok(result) => Ok(result),
            Err(e) => Err(AppError::CustomError(e.to_string())),
        }
    }

    pub async fn total_students(&self) -> u64 {
        let result = match self.student_col.count_documents(None, None).await{
            Ok(count) => count,
            Err(_) => 0,
        };

        result
    }

    pub async fn update_student(&self, studentId:ObjectId, student:CreateStudentDTO) -> Result<UpdateResult, AppError> {
        let update = doc! {
            "$set":{
                "name":student.name,
                "age":student.age,
                "date_of_birth":student.date_of_birth,
                "address": student.address,
                "class_branch": student.class_branch,
                "updated_at":bson::DateTime::now()
            }
        };

        match self.student_col.update_one(doc! { "_id":studentId }, update, None).await {
            Ok(result) => Ok(result),
            Err(e) => {
                Err(AppError::CustomError(e.to_string()))
            },
        }
    } 

}