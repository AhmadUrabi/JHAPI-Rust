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
}
