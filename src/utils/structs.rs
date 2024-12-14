#[derive(Debug)]
pub enum APIError {
    DBError,
    DataNotFound,
    DataExists,
    InvalidData,
    InvalidToken,
    SFTPError,
    InternalServerError,
    InvalidCredentials,
    NoData,
    IOError,
}

use std::fmt;

use crate::server::response::ApiResponse;

impl fmt::Display for APIError {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            APIError::DBError => write!(f, "Database Error"),
            APIError::DataNotFound => write!(f, "User Not Found"),
            APIError::DataExists => write!(f, "User Already Exists"),
            APIError::InvalidData => write!(f, "Invalid Data"),
            APIError::InvalidToken => write!(f, "Invalid Token"),
            APIError::SFTPError => write!(f, "SFTP Error"),
            APIError::InternalServerError => write!(f, "Internal Server Error"),
            APIError::InvalidCredentials => write!(f, "Invalid Credentials"),
            APIError::NoData => write!(f, "No Data Found"),
            APIError::IOError => write!(f, "IO Error"),
        }
    }
}

impl Into<ApiResponse> for APIError {
    fn into(self) -> ApiResponse {
        match self {
            APIError::DBError => ApiResponse::new(500, Some("Database Error".to_string()), None),
            APIError::DataNotFound => {
                ApiResponse::new(404, Some("Resource Not Found".to_string()), None)
            }
            APIError::DataExists => {
                ApiResponse::new(409, Some("Resource Already Exists".to_string()), None)
            }
            APIError::InvalidData => {
                ApiResponse::new(422, Some("Unprocessable Entity".to_string()), None)
            }
            APIError::InvalidToken => {
                ApiResponse::new(401, Some("Authentication Error".to_string()), None)
            }
            APIError::SFTPError => {
                ApiResponse::new(500, Some("File Transfer Error".to_string()), None)
            }
            APIError::InternalServerError => {
                ApiResponse::new(500, Some("Internal Server Error".to_string()), None)
            }
            APIError::InvalidCredentials => {
                ApiResponse::new(401, Some("Invalid Credentials".to_string()), None)
            }
            APIError::NoData => ApiResponse::new(204, Some("No Content Found".to_string()), None),
            APIError::IOError => ApiResponse::new(500, Some("IO Error".to_string()), None),
        }
    }
}
