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