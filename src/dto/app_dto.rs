use serde::{Deserialize, Serialize};
use serde::de::{self, Visitor};
use crate::models::app::{Branches, Courses, Enquiries, Facilities, Fees};
use std::fmt::{self};
use validator::Validate;

#[derive(Serialize, Deserialize)]
pub struct CreateBranchDTO{
    pub name:String,
    pub address:String,
    pub is_active:bool,
}
#[derive(Serialize, Deserialize)]
pub struct GetBranchDTO {
    pub id:String,
    pub name:String,
    pub address:String,
    pub is_active:bool,
    pub created_at:String,
    pub updated_at:String
}

impl GetBranchDTO {
    pub fn init(branch:Branches) -> Self {
        GetBranchDTO {
            id: branch.id.unwrap().to_string(),
            name: branch.name,
            address: branch.address,
            is_active: branch.is_active,
            created_at: branch.created_at.to_string(),
            updated_at: branch.updated_at.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct AppCountDTO {
    pub totalStudent:u64,
    pub lastMonthAdmission:u64,
    pub totalBranches:u64,
    pub upCommingEvents:u64,
    pub totalEvents:u64
}
#[derive(Serialize, Deserialize)]
pub struct CreateFeesDTO {
    #[serde(deserialize_with="deserialize_fee_types")]
    pub fee_type:FeeTypes,
    pub fee_amount:i64,
    pub fee_discount:f64
}

#[derive(Serialize, Deserialize)]
pub enum FeeTypes {
    MONTHLY,
    YEARLY,
    THREEMONTH,
    SIXMONTH
}

fn deserialize_fee_types<'de, D>(deserializer: D) -> Result<FeeTypes, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct StudentLevelVisitor;

    impl<'de> Visitor<'de> for StudentLevelVisitor {
        type Value = FeeTypes;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid student level string")
        }

        fn visit_str<E>(self, value: &str) -> Result<FeeTypes, E>
        where
            E: de::Error,
        {
            match value {
                "monthly" => Ok(FeeTypes::MONTHLY),
                "yearly" => Ok(FeeTypes::YEARLY),
                "three_month" => Ok(FeeTypes::THREEMONTH),
                "six_month" => Ok(FeeTypes::SIXMONTH),
                _ => Err(E::custom(format!("Invalid Student Level: {}", value))),
            }
        }
    }

    deserializer.deserialize_str(StudentLevelVisitor)
}


impl Default for FeeTypes {
    fn default() -> Self {
        FeeTypes::MONTHLY
    }
}


impl ToString for FeeTypes {
    fn to_string(&self) -> String {
        match self {
            FeeTypes::MONTHLY => String::from("MONTHLY"),
            FeeTypes::YEARLY => String::from("YEARLY"),
            FeeTypes::THREEMONTH => String::from("THREEMONTH"),
            FeeTypes::SIXMONTH => String::from("SIXMONTH"),
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct FeesDTO {
    pub id:String,
    pub fee_type:String,
    pub fee_amount:i64,
    pub is_discount:bool,
    pub fee_discount:f64,
    pub created_at:String,
    pub updated_at:String
}

#[allow(non_snake_case)]
impl FeesDTO {
    pub fn init(feeModel:Fees) -> Self {
        Self {
            id: feeModel.id.unwrap().to_string(),
            fee_type: feeModel.fee_type,
            fee_amount: feeModel.fee_amount,
            is_discount: feeModel.is_discount,
            fee_discount: feeModel.fee_discount,
            created_at: feeModel.created_at.to_string(),
            updated_at: feeModel.updated_at.to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateCourseDTO {
    pub id:String,
    #[validate(required, length(min=1, message="Name can not be empty"))]
    pub name:Option<String>,
    #[validate(required, length(min=1, message="description can not be empty"))]
    pub description:Option<String>,
    pub course_duration:String
}
#[derive(Serialize, Deserialize, Validate)]
#[allow(non_snake_case)]
pub struct ActiveCourseRequestDTO {
    #[validate(required, length(min=1, message="Id can not be empty"))]
    pub id:Option<String>,
    #[validate(required)]
    pub isActive:Option<bool>
}

#[derive(Serialize, Deserialize)]
pub struct CoursesDTO {
    pub id:String,
    pub name:String,
    pub description:String,
    pub course_duration:String,
    pub is_active:bool,
    pub created_at:String,
    pub updated_at:String
}

impl CoursesDTO {
    pub fn init(course:Courses) -> Self {
        Self {
            id: course.id.unwrap().to_string(),
            name: course.name.to_string(),
            description: course.description.to_string(),
            course_duration: course.course_duration.to_string(),
            created_at: course.created_at.to_string(),
            updated_at: course.updated_at.to_string(),
            is_active: course.is_active,
        }
    }
}
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateFacilities {
    #[validate(required,length(min=1,message="title can not be empty"))]
    pub title:Option<String>,
    #[validate(required, length(min=1,message="description can not be empty"))]
    pub description:Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub image_url:Option<String>,
}


#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct FacilitiesDTO {
    pub id:String,
    pub title:String,
    pub description:String,
    pub imageUrl:Option<String>,
    pub created_at:String,
    pub updated_at:String
}

#[allow(non_snake_case)]
impl FacilitiesDTO {
    
    pub fn init(facilities:Facilities) -> Self {
        let imgUrl = facilities.imageUrl.as_ref().map(|sl| sl.to_string());;
        Self {
            id: facilities.id.unwrap().to_hex(),
            title: facilities.title,
            description: facilities.description,
            imageUrl: imgUrl,
            created_at: facilities.created_at.to_string(),
            updated_at: facilities.updated_at.to_string(),
        }
    }
}
#[derive(Serialize,Deserialize)]
pub struct CreateEnquiryDTO {
    pub name:String,
    pub email:String,
    pub contact:String,
    pub subject:String,
    pub message:String
}

#[derive(Serialize, Deserialize)]
pub struct EnquiriesDTO {
    pub id:String,
    pub name:String,
    pub email:String,
    pub contact:String,
    pub subject:String,
    pub message:String,
    pub created_at:String,
    pub updated_at:String
}

impl EnquiriesDTO {
    
    pub fn init(enquire:Enquiries) -> Self {
        Self {
            id: enquire.id.unwrap().to_string(),
            name: enquire.name,
            email: enquire.email,
            contact: enquire.contact,
            subject: enquire.subject,
            message: enquire.message,
            created_at: enquire.created_at.to_string(),
            updated_at: enquire.updated_at.to_string()
        }
    }
}