use serde::{Deserialize, Serialize};
use serde::de::{self, Visitor};
use crate::models::app::{Branches, Fees};
use std::fmt::{self};

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