use serde::{Deserialize, Serialize};
use crate::models::events::Events;

#[derive(Serialize,Deserialize)]
pub struct CreateEventDTO {
    pub title:String,
    pub discription:String,
    pub location:String,
    pub start_date:String,
    pub end_date:String,
}

#[derive(Serialize, Deserialize)]
pub struct GetEventsDTO {
    pub id:String,
    pub title:String,
    pub discription:String,
    pub location:String,
    pub file_data:Option<Vec<GetFileData>>,
    pub start_date:String,
    pub end_date:String,
    pub created_at:String,
    pub updated_at:String
}

#[derive(Serialize,Deserialize)]
pub struct GetFileData {
    pub file_type:String,
    pub file_path:String,
    pub created_at:String
}
#[derive(Serialize, Deserialize)]
pub struct UpdateEventDTO {
    pub title:String,
    pub discription:String,
    pub location:String,
    pub is_active:bool,
    pub start_date:String,
    pub end_date:String
}


impl GetEventsDTO {
    
    pub fn init(event:Events) -> Self {
        let mut event_dto =  GetEventsDTO {
            id: event.id.unwrap().to_string(),
            title: event.title,
            discription: event.discription,
            location: event.location,
            start_date: event.start_date.unwrap().to_string(),
            end_date: event.end_date.unwrap().to_string(),
            created_at: event.created_at.unwrap().to_string(),
            updated_at: event.updated_at.unwrap().to_string(),
            file_data: None,
        };

        if !event.file_data.is_none() {
            let mut data:Vec<GetFileData> = Vec::new();
            for i in event.file_data.unwrap() {
                data.push( GetFileData {
                    file_type: i.file_type,
                    file_path:  format!("http://192.168.0.119:8000{}",i.file_path),
                    created_at: i.created_at.unwrap().to_string(),
                })
            }
            event_dto.file_data = Some(data)
        }

        event_dto
    }
}