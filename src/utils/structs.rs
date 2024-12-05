#[derive(Debug)]
pub enum APIError {
    DBError,
    UserNotFound,
    UserExists,
    InvalidData,
    InvalidToken,
    SFTPError,
    InternalServerError,
    FileNotFound,
    InvalidCredentials,
    NoData,
    IOError,
}

use std::fmt;

impl fmt::Display for APIError {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            APIError::DBError => write!(f, "Database Error"),
            APIError::UserNotFound => write!(f, "User Not Found"),
            APIError::UserExists => write!(f, "User Already Exists"),
            APIError::InvalidData => write!(f, "Invalid Data"),
            APIError::InvalidToken => write!(f, "Invalid Token"),
            APIError::SFTPError => write!(f, "SFTP Error"),
            APIError::InternalServerError => write!(f, "Internal Server Error"),
            APIError::FileNotFound => write!(f, "File Not Found"),
            APIError::InvalidCredentials => write!(f, "Invalid Credentials"),
            APIError::NoData => write!(f, "No Data Found"),
            APIError::IOError => write!(f, "IO Error"),
        }
    }
}
