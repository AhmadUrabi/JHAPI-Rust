use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

use crate::controllers::auth::validate_token;

// Start Request Guard Functions
#[derive(Debug, Clone)]
pub struct ApiKey<'r>(pub &'r str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Returns true if `key` is a valid JWT Token.
        // First check cookies
        if let Some(cookie) = req.cookies().get("token") {
            if validate_token(cookie.value()) {
                info!("Valid Token Found in Cookie");
                return Outcome::Success(ApiKey(cookie.value()));
            } else {
                error!("Invalid Token Found in Cookie");
                return Outcome::Error((
                    Status::Unauthorized,
                    "Invalid authentication token in cookie".to_string(),
                ));
            }
        }

        // If no valid cookie, check headers (For Legacy Support)
        match req.headers().get_one("Authorization") {
            Some(key) if validate_token(key) => {
                info!("Valid Token Found in Header");
                Outcome::Success(ApiKey(key))
            }
            Some(_) => {
                error!("Invalid Token Found in Header");
                Outcome::Error((
                    Status::Unauthorized,
                    "Invalid authentication token in header".to_string(),
                ))
            }
            None => {
                error!("No Token Found");
                Outcome::Error((
                    Status::Unauthorized,
                    "No authentication token found".to_string(),
                ))
            }
        }
    }
}
