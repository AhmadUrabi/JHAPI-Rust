use oracle::pool::Pool;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, State};

use crate::signing::signin;
use crate::signing::structs::LoginParams;

#[post("/Sign", data = "<params>")]
pub async fn sign(params: Json<LoginParams>, pool: &State<Pool>) -> Result<String, Status> {
    info!("Sign Request: {:?}", params.0.pUserName);
    match signin(params, pool).await {
        Some(token) => {
            info!("Valid User Data, Token Sent");
            Ok(token.to_string())
        }
        None => {
            error!("Invalid User Data, Token Not Sent");
            Err(Status::Unauthorized)
        }
    }
}
