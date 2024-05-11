use serde::{Deserialize, Serialize};

use crate::models::app::Branches;

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