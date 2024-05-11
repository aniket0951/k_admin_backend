use bson::{doc, oid::ObjectId, Document};
use futures::TryStreamExt;
use mongodb::{options, results::{DeleteResult, InsertOneResult, UpdateResult}, Collection, Database};

use crate::{dto::event_dto::UpdateEventDTO, helper::app_errors::AppError, models::events::{Events, FileData}};



pub struct EventRepo {
    event_col:Collection<Document>
}

#[allow(non_snake_case)]
impl EventRepo {

    pub fn init(db:Database) -> Self {
        let event_col = db.collection("events");
        EventRepo{ event_col }
    }

    pub async fn add_event(&self, event:Events) -> Result<InsertOneResult, AppError> {
        let event_bson = match event.to_docmunet() {
            Ok(document) => document,
            Err(e) => return Err(AppError::CustomError(e.to_string())),
        };

        match self.event_col.insert_one(event_bson, None).await {
            Ok(result) => Ok(result),
            Err(e) => Err(AppError::CustomError(e.to_string())),
        }
    }

    pub async fn add_file_data(&self, eventId:ObjectId, fileData:FileData) -> Result<UpdateResult, AppError> {
        let bson_fileData = match fileData.to_docmunet() {
            Ok(data) => data,
            Err(e) => return Err(AppError::CustomError(e.to_string())),
        };
        let update = doc! {
            "$push":doc! {
                "file_data":bson_fileData
            }
        };

        match self.event_col.update_one(doc! { "_id":eventId }, update, None).await {
            Ok(result) => Ok(result),
            Err(e) => Err(AppError::CustomError(e.to_string())),
        }
    }

    pub async fn get_events(&self, skip:i64, limit:i64) -> Result<Vec<Events>, AppError> {
        let opt = options::FindOptions::builder()
            .sort(doc! {"created_at":-1})
            .skip(skip as u64)
            .limit(limit)
            .build();

        let mut cursor = match self.event_col.find(doc!{"is_active":true}, opt).await {
            Ok(cursor) => cursor,
            Err(e) => {
                return Err(AppError::CustomError(e.to_string()));
            },
        };

        let mut events:Vec<Events> = Vec::new();

        while let Some(event) = cursor
            .try_next()
            .await
            .ok()
            .expect("Mapping Error")
        {
            events.push(bson::from_document(event).unwrap())
        }

        Ok(events)
    }

    pub async fn get_event(&self, eventId:ObjectId) -> Result<Events, AppError> {
        let event = match self.event_col.find_one(doc! { "_id":eventId }, None).await {
            Ok(Some(document)) => document,
            Ok(None) => return Err(AppError::DataNotFoundError),
            Err(e) => return Err(AppError::CustomError(e.to_string())),
        };

        bson::from_document(event).map_err(|e| AppError::CustomError(e.to_string()))
    }

    pub async fn delete_event(&self, eventId:ObjectId) -> Result<DeleteResult, AppError> {
        match self.event_col.delete_one(doc! { "_id":eventId } , None).await {
            Ok(result) => {
                Ok(result)
            },
            Err(e) => Err(AppError::CustomError(e.to_string())),
        }
    }

    pub async fn update_event(&self, eventdId:ObjectId, eventDTO:UpdateEventDTO) -> Result<UpdateResult, AppError> {
        let update = doc! {
            "$set":{
                "title":eventDTO.title.to_string(),
                "discription":eventDTO.discription,
                "location":eventDTO.location,
                "is_active":eventDTO.is_active,
                "start_date":eventDTO.start_date,
                "end_date":eventDTO.end_date,
                "updated_at":bson::DateTime::now()
            }
        };

        match self.event_col.update_one(doc! { "_id":eventdId }, update, None).await {
            Ok(result) => Ok(result),
            Err(e) =>Err(AppError::CustomError(e.to_string())),
        }
    }

    pub async fn total_event(&self) -> Result<u64, AppError> {
        match self.event_col.count_documents(None, None).await {
            Ok(result) => Ok(result),
            Err(e) => Err(AppError::CustomError(e.to_string())),
        }
    }

    pub async fn upcommint_event_count(&self) -> Result<u64, AppError> {
        
        let filter = doc! {
            "start_date": {
                "$gte":bson::DateTime::now(),
            }
        };
        match self.event_col.count_documents(filter, None).await {
            Ok(count) => {
                Ok(count)
            },
            Err(e) => Err(AppError::CustomError(e.to_string())),
        }
    }

}