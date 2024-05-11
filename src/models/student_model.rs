use bson::{oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Students {
    #[serde(skip_serializing_if="Option::is_none", rename="_id")]
    pub id:Option<ObjectId>,
    pub student_id:Option<String>,
    pub name:String,
    pub age:i64,
    pub date_of_birth:String,
    pub address:String,
    pub is_active_student:bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_pic:Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub class_branch:Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent:Option<Parents>,
    pub level:Option<String>,
    pub nationality:Option<String>,
    pub blood_group:Option<String>,
    pub weight:Option<i64>,
    pub school_name:Option<String>,
    pub addhar_number:Option<String>,
    pub geneder:Option<String>,
    pub registration_status:Option<String>,
    pub created_at:Option<bson::DateTime>,
    pub updated_at:Option<bson::DateTime>
}

impl Students {
    pub fn to_docmunet(&self) -> Result<Document, mongodb::bson::ser::Error> {
        bson::to_document(self)
    }
}
#[derive(Serialize, Deserialize)]
pub struct Parents {
    pub student_id:Option<ObjectId>,
    pub name:String,
    pub address:String,
    pub mobile_number:i64,
    pub email:String,
    pub created_at:Option<bson::DateTime>,
    pub updated_at:Option<bson::DateTime>
}

impl Parents {
    pub fn to_docmunet(&self) -> Result<Document, mongodb::bson::ser::Error> {
        bson::to_document(self)
    }
}