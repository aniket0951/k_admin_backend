use bson::{oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize, Default)]
pub struct Events {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id:Option<ObjectId>,
    pub title:String,
    pub discription:String,
    pub location:String,
    pub is_active:Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub file_data:Option<Vec<FileData>>,
    pub start_date:Option<bson::DateTime>,
    pub end_date:Option<bson::DateTime>,
    pub created_at:Option<bson::DateTime>,
    pub updated_at:Option<bson::DateTime>
}

impl Events {
    pub fn to_docmunet(&self) -> Result<Document, mongodb::bson::ser::Error> {
        bson::to_document(self)
    }
}

#[derive(Serialize, Deserialize)]
pub struct FileData {
    pub file_type:String,
    pub file_path:String,
    pub created_at:Option<bson::DateTime>
}

impl FileData {
    pub fn to_docmunet(&self) -> Result<Document, mongodb::bson::ser::Error> {
        bson::to_document(self)
    }
}