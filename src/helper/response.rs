use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct ResponseBuilder<T> {
    status:bool,
    message:String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error:Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data:Option<T>
}

#[allow(non_snake_case)]
impl<T> ResponseBuilder<T> {
    
    const MSG: &'static str = "Failed to process";

    pub fn SuccessResponse(msg:String, data:Option<T>) -> Self {
        ResponseBuilder {
            status: true,
            message: msg,
            error: Some("".to_string()),
            data,
        }
    }

    pub fn FailedResponse(err:String) -> Self {
        ResponseBuilder {
            status: false,
            message: Self::MSG.to_string(),
            error: Some(err),
            data: None,
        }
    }

    pub fn InValidIdResponse() -> Self {
        ResponseBuilder {
            status: false,
            message: Self::MSG.to_string(),
            error: Some("Invalid id provided !".to_string()),
            data: None,
        }
    }
}