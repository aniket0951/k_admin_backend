use core::fmt;

#[derive(Debug)]
#[allow(non_snake_case)]
#[derive(PartialEq)]
pub enum AppError {
    InvalidIdError,
    DataNotFoundError,
    UserNotFoundError,
    DeserializationError,
    DatabaseError,
    AccountNotFoundError,
    CustomError(String)
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InvalidIdError => write!(f, "Invalid Id found"),
            AppError::DataNotFoundError => write!(f, "Data not found"),
            AppError::UserNotFoundError => write!(f, "User not found"),
            AppError::DeserializationError => write!(f, "Error while deserialize"),
            AppError::DatabaseError => write!(f, "Database Error"),
            AppError::CustomError(msg) => write!(f, "{}", msg),
            AppError::AccountNotFoundError => write!(f, "User Account Not Found !"),
        }
    }
}

// Success Messages
#[derive(Debug)]
pub enum Messages {
    DataFetchSuccess,
    DataFetchFailed,
    DataAddedSuccess,
    DataUpdateSuccess,
    DataUpdateFailed,
    DataDeleteSucess,

    DataDeleteFailed,
    CustomMessage(String)
}

impl fmt::Display for Messages {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Messages::DataFetchSuccess => write!(f, "Data fetch successfully !"),
            Messages::DataAddedSuccess => write!(f, "Data Added successfully !"),
            Messages::DataUpdateSuccess => write!(f, "Data Update successfully !"),
            Messages::DataDeleteSucess => write!(f, "Data Deleted successfully !"),
            Messages::DataDeleteFailed => write!(f, "failed to delete data !"),
            Messages::CustomMessage(msg) => write!(f, "{}", msg),
            Messages::DataUpdateFailed => write!(f, "Failed to update data !"),
            Messages::DataFetchFailed => write!(f, "Failed to fetch data !"),
        }
    }
}

impl std::error::Error for AppError {}