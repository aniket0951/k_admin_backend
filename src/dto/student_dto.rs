use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use serde::{Deserialize, Serialize};
use serde::de::{self, Visitor};
use std::fmt::{self};
use crate::models::student_model::{Parents, Students};



#[derive(Deserialize, Serialize)]
pub struct CreateStudentDTO {
    pub name:String,
    pub age:i64,
    #[serde(rename="dob")]
    pub date_of_birth:String,
    pub address:String,
    pub class_branch:String,
    #[serde(deserialize_with="deserialize_student_level")]
    pub level:StudentLevels,
    pub blood_group:String,
    pub weight:i64,
    pub school_name:String,
    pub addhar_number:String,
    pub geneder:String,
}

#[derive(Serialize, Deserialize)]
pub enum StudentLevels {
    OFFWHITE,
    YELLOW,
    ORANGE,
    GREEN,
    BLUE,
    PURPLE,
    BROWN,
    BROWNII,
    BROWNIII,
    BLACK
}


fn deserialize_student_level<'de, D>(deserializer: D) -> Result<StudentLevels, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct StudentLevelVisitor;

    impl<'de> Visitor<'de> for StudentLevelVisitor {
        type Value = StudentLevels;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid student level string")
        }

        fn visit_str<E>(self, value: &str) -> Result<StudentLevels, E>
        where
            E: de::Error,
        {
            match value {
                "Off_White" | "OffWhite" => Ok(StudentLevels::OFFWHITE),
                "yellow" => Ok(StudentLevels::YELLOW),
                "orange" => Ok(StudentLevels::ORANGE),
                "green" => Ok(StudentLevels::GREEN),
                "blue" => Ok(StudentLevels::BLUE),
                "purple" => Ok(StudentLevels::PURPLE),
                "brown" => Ok(StudentLevels::BROWN),
                "brownII" => Ok(StudentLevels::BROWNII),
                "brownIII" => Ok(StudentLevels::BROWNIII),
                "black" => Ok(StudentLevels::BLACK),
                _ => Err(E::custom(format!("Invalid Student Level: {}", value))),
            }
        }
    }

    deserializer.deserialize_str(StudentLevelVisitor)
}


impl Default for StudentLevels {
    fn default() -> Self {
        StudentLevels::OFFWHITE
    }
}


impl ToString for StudentLevels {
    fn to_string(&self) -> String {
        match self {
            StudentLevels::OFFWHITE => String::from("OFFWHITE"),
            StudentLevels::YELLOW => String::from("YELLOW"),
            StudentLevels::ORANGE => String::from("ORANGE"),
            StudentLevels::GREEN => String::from("GREEN"),
            StudentLevels::BLUE => String::from("BLUE"),
            StudentLevels::PURPLE => String::from("PURPLE"),
            StudentLevels::BROWN => String::from("BROWN"),
            StudentLevels::BROWNII => String::from("BROWNII"),
            StudentLevels::BROWNIII => String::from("BROWNIII"),
            StudentLevels::BLACK => String::from("BLACK"),
        }
    }
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
    pub level:Option<String>,
    pub nationality:Option<String>,
    pub blood_group:Option<String>,
    pub weight:Option<i64>,
    pub school_name:Option<String>,
    pub addhar_number:Option<String>,
    pub geneder:Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub student_id:Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_status:Option<String>,
    pub created_at:String,
    pub updated_at:String
}

impl StudentsDTO {
    pub fn init(student:Students) -> Self {
        
       let mut s =  Self {
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
            level: None,
            nationality: None,
            blood_group: None,
            weight: None,
            school_name: None,
            addhar_number: None,
            geneder: None,
            student_id: None,
            registration_status: None,
        };

        if !student.parent.is_none() {
            s.parent = Some(GetParentDTO::init(student.parent.unwrap()))
        }

        if !s.profile_pic.is_none() {
            s.profile_pic = Some(format!("http://192.168.0.119:8000{}" , s.profile_pic.unwrap()))
        }

        s.level = student.level.as_ref().map(|sl| sl.to_string());
        s.nationality = student.nationality.as_ref().map(|sn| sn.to_string());
        s.blood_group = student.blood_group.as_ref().map(|bg| bg.to_string());
        s.weight = student.weight.as_ref().map(|wg| wg * 2);
        s.school_name = student.school_name.as_ref().map(|sn| sn.to_string());
        s.addhar_number = student.addhar_number.as_ref().map(|an| an.to_string());
        s.geneder = student.geneder.as_ref().map(|sg| sg.to_string());
        s.student_id = student.student_id.as_ref().map(|si| si.to_string());
        s.registration_status = student.registration_status.as_ref().map(|sr| sr.to_string());

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

#[derive(MultipartForm)]
pub struct UploadProfileDTO {
    pub file:TempFile,
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