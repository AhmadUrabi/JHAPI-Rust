use std::net::IpAddr;

use oracle::pool::Pool;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, State};

use crate::signing::signin;
use crate::signing::structs::LoginParams;

use crate::utils::logging::{get_timestamp, log_data};
use crate::utils::structs::APIErrors;

#[post("/login", data = "<params>")]
pub async fn sign(
    params: Json<LoginParams>,
    pool: &State<Pool>,
    client_ip: Option<IpAddr>,
) -> Result<String, Status> {
    info!("Sign Request: {:?}", params.0.p_username);
    info!("Client IP: {:?}", client_ip);

    let username = params.0.p_username.clone();

    match signin(params, pool).await {
        Ok(token) => {
            info!("Valid User Data, Token Sent");
            log_data(
                pool,
                username,
                client_ip.unwrap().to_string(),
                "/login".to_string(),
                None,
                get_timestamp(),
                "None".to_string(),
                "Token Generated".to_string(),
                "POST".to_string(),
            );
            Ok(token.to_string())
        }
        Err(e) => {
            log_data(
                pool,
                username,
                client_ip.unwrap().to_string(),
                "/login".to_string(),
                None,
                get_timestamp(),
                "None".to_string(),
                match e {
                    APIErrors::InvalidData => "Invalid User Data, Token Not Sent".to_string(),
                    APIErrors::DBError => "Database Error".to_string(),
                    APIErrors::UserNotFound => "User Not Found".to_string(),
                    _ => "Error, Token Not Sent".to_string(),
                },
                "POST".to_string(),
            );
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
