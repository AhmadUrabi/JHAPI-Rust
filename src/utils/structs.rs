#[derive(Debug)]
pub enum APIErrors {
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
    IOError
}

use std::fmt;

impl fmt::Display for APIErrors {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            APIErrors::DBError => write!(f, "Database Error"),
            APIErrors::UserNotFound => write!(f, "User Not Found"),
            APIErrors::UserExists => write!(f, "User Already Exists"),
            APIErrors::InvalidData => write!(f, "Invalid Data"),
            APIErrors::InvalidToken => write!(f, "Invalid Token"),
            APIErrors::SFTPError => write!(f, "SFTP Error"),
            APIErrors::InternalServerError => write!(f, "Internal Server Error"),
            APIErrors::FileNotFound => write!(f, "File Not Found"),
            APIErrors::InvalidCredentials => write!(f, "Invalid Credentials"),
            APIErrors::NoData => write!(f, "No Data Found"),
            APIErrors::IOError => write!(f, "IO Error"),
        }
    }
}