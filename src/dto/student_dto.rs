use serde::{Deserialize, Serialize};

use crate::models::student_model::{Parents, Students};



#[derive(Deserialize, Serialize)]
pub struct CreateStudentDTO {
    pub name:String,
    pub age:i64,
    #[serde(rename="dob")]
    pub date_of_birth:String,
    pub address:String,
    pub class_branch:String
}

#[derive(Serialize,Deserialize)]
pub struct StudentsDTO  {
    pub id:String,
    pub name:String,
    pub age:i64,
    #[serde(rename="dob")]
    pub date_of_birth:String,
    pub address:String,
    pub is_active_student:bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_pic:Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub class_branch:Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub parent:Option<GetParentDTO>,
    pub created_at:String,
    pub updated_at:String
}

impl StudentsDTO {
    pub fn init(student:Students) -> Self {
        
       let mut s =  StudentsDTO {
            id: student.id.unwrap().to_string(),
            name: student.name,
            age: student.age,
            date_of_birth: student.date_of_birth,
            address: student.address,
            is_active_student: student.is_active_student,
            profile_pic: student.profile_pic,
            class_branch: student.class_branch,
            parent: None,
            created_at: student.created_at.unwrap().to_string(),
            updated_at: student.updated_at.unwrap().to_string(),
        };

        if !student.parent.is_none() {
            s.parent = Some(GetParentDTO::init(student.parent.unwrap()))
        }

        if !s.profile_pic.is_none() {
            s.profile_pic = Some(format!("http://localhost:8000 {}" , s.profile_pic.unwrap()))
        }

        s
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateParentDTO {
    pub student_id:String,
    pub name:String,
    pub address:String,
    pub mobile_number:i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email:Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct GetParentDTO {
    pub student_id:String,
    pub name:String,
    pub address:String,
    pub mobile_number:i64,
    pub email:String,
    pub created_at:String,
    pub updated_at:String
}

impl GetParentDTO {
    pub fn init(parent:Parents) -> Self {


        GetParentDTO {
            student_id: parent.student_id.unwrap().to_string(),
            name: parent.name,
            address: parent.address,
            mobile_number: parent.mobile_number,
            email: parent.email,
            created_at: parent.created_at.unwrap().to_string(),
            updated_at: parent.updated_at.unwrap().to_string(),
        }
    }
}