use crate::server::JHApiServerState;
use oracle::pool;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, State};

use crate::functions::authentication::signin;
use crate::functions::authentication::structs::LoginParams;

use crate::utils::structs::APIErrors;

#[post("/login", data = "<params>")]
pub async fn sign(
    params: Json<LoginParams>,
    state: &State<JHApiServerState>
) -> Result<String, Status> {
    info!("Sign Request: {:?}", params.0.p_username);
    let pool = &state.pool;
    let sql_manager = &state.sql_manager;
    match signin(params, &pool, &sql_manager).await {
        Ok(token) => {
            info!("Valid User Data, Token Sent");
            Ok(token.to_string())
        }
        Err(e) => {
            error!("Error authorizing, Token Not Sent");
            match e {
                APIErrors::InvalidData => Err(Status::Unauthorized),
                APIErrors::DBError => Err(Status::InternalServerError),
                APIErrors::UserNotFound => Err(Status::Unauthorized),
                APIErrors::InvalidCredentials => Err(Status::Unauthorized),
                _ => Err(Status::InternalServerError),
            }
        }
    }
}
