use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct ResponseBuilder<T> {
    status:bool,
    message:String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error:Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data:Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pagination:Option<PaginationData>
}
#[derive(Serialize, Deserialize)]
pub struct PaginationData {
    pub total:i64
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
            pagination: None
            
        }
    }

    pub fn FailedResponse(err:String) -> Self {
        ResponseBuilder {
            status: false,
            message: Self::MSG.to_string(),
            error: Some(err),
            data: None,
            pagination: None
        }
    }

    pub fn InValidIdResponse() -> Self {
        ResponseBuilder {
            status: false,
            message: Self::MSG.to_string(),
            error: Some("Invalid id provided !".to_string()),
            data: None,
            pagination: None
        }
    }

    pub fn SuccessResponseWithPagination(msg:String, data:Option<T>, total:i64) -> Self {
        ResponseBuilder {
            status: true,
            message: msg,
            error: Some("".to_string()),
            data,
            pagination: Some(PaginationData{ total })
        }
    }

}