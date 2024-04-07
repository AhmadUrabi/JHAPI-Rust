use rocket::{http::Status, request::{FromRequest, Outcome}, Request};

use crate::functions::signing::validate_token;


// Start Request Guard Functions
#[derive(Debug, Clone)]
pub struct ApiKey<'r>(pub &'r str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Returns true if `key` is a valid JWT Token.

        match req.headers().get_one("Authorization") {
            None => {
                error!("No Authentication header found");
                Outcome::Error((
                    Status::Unauthorized,
                    "Please include an Authentication header".to_string(),
                ))
            }
            Some(key) if validate_token(key) => {
                info!("Valid Token Found");
                Outcome::Success(ApiKey(key))
            }
            Some(_) => {
                error!("Invalid Token Found");
                Outcome::Error((
                    Status::Unauthorized,
                    "Please include a valid Authentication header".to_string(),
                ))
            }
        }
    }
}