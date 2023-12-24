#[derive(Debug)]
pub enum APIErrors {
    DBError,
    UserNotFound,
    UserExists,
    InvalidData,
    InvalideToken,
}