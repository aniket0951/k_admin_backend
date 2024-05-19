use bson::{oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Branches {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id:Option<ObjectId>,
    pub name:String,
    pub address:String,
    pub is_active:bool,
    pub created_at:bson::DateTime,
    pub updated_at:bson::DateTime
}

impl Branches {
    pub fn to_document(&self) -> Result<Document, mongodb::bson::ser::Error> {
        bson::to_document(self)
    }
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Fees {
    #[serde(rename="_id",skip_serializing_if="Option::is_none")]
    pub id:Option<ObjectId>,
    pub fee_type:String,
    pub fee_amount:i64,
    pub is_discount:bool,
    pub fee_discount:f64,
    pub created_at:bson::DateTime,
    pub updated_at:bson::DateTime
}

impl Fees {
    pub fn to_document(&self) -> Result<Document, mongodb::bson::ser::Error> {
        bson::to_document(self)
    }
}
#[derive(Serialize, Deserialize)]
pub struct Courses{
    #[serde(rename="_id",skip_serializing_if="Option::is_none")]
    pub id:Option<ObjectId>,
    pub name:String,
    pub description:String,
    pub is_active:bool,
    pub course_duration:String,
    pub created_at:bson::DateTime,
    pub updated_at:bson::DateTime
}

impl Courses {
    pub fn to_document(&self) -> Result<Document, mongodb::bson::ser::Error> {
        bson::to_document(self)
    }
}