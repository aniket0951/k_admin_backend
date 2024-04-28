use bson::{oid::ObjectId, Document};
use serde::{Deserialize, Serialize};



#[derive(Deserialize, Serialize, Default)]
pub struct Users {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id:Option<ObjectId>,
    pub name:String,
    pub email:String,
    pub mobile_number:String,
    pub password:String,
    #[serde(deserialize_with="deserialize_user_type")]
    pub user_type:UserTypes,
    pub is_active:bool,
    pub created_at:Option<bson::DateTime>,
    pub updated_at:Option<bson::DateTime>
}


impl Users {
    pub fn to_docmunet(&self) -> Result<Document, mongodb::bson::ser::Error> {
        bson::to_document(self)
    }
}

fn deserialize_user_type<'de, D>(deserializer: D) -> Result<UserTypes, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    match value.as_str() {
        "AdminUser" | "ADMIN" => Ok(UserTypes::ADMIN),
        "SubAdminUser"  => Ok(UserTypes::SUBADMIN),
        "EndUser" | "end_user" => Ok(UserTypes::ENDUSER),
        _ => {
            Err(serde::de::Error::custom(format!("Unknown user type: {}", value)))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum UserTypes {
    ADMIN,
    SUBADMIN,
    ENDUSER
}

impl Default for UserTypes {
    fn default() -> Self {
        UserTypes::ENDUSER
    }
}

impl ToString for UserTypes {
    fn to_string(&self) -> String {
        match self {
            UserTypes::ADMIN => String::from("Admin"),
            UserTypes::SUBADMIN => String::from("SubAdmin"),
            UserTypes::ENDUSER => String::from("EndUser"),
        }
    }
}