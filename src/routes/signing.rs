use rocket::serde::json::Json;
use rocket::{State, post};
use rocket::http::Status;
use oracle::pool::Pool;

use crate::signing::structs::LoginParams;
use crate::signing::signin;


#[post("/Sign", data = "<params>")]
pub async fn sign(params: Json<LoginParams>, pool: &State<Pool>) -> Result<String, Status> {
    info!("Sign Request: {:?}", params.0.pUserName);
    match signin(params, pool).await {
        Some(token) => {
            info!("Valid User Data, Token Sent");
            Ok(token.to_string())
        },
        None => {
            error!("Invalid User Data, Token Not Sent");
            Err(Status::Unauthorized)
        },
    }
}