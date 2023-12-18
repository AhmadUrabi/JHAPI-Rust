use std::net::IpAddr;

use oracle::pool::Pool;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, State};

use crate::utils::logging::{get_timestamp, log_data};

use crate::version_check::structs::*;
use crate::version_check::get_latest_version;

#[post("/version", data = "<params>")]
pub async fn route_version_check(
    params: Json<VersionParams>,
    pool: &State<Pool>,
    client_ip: Option<IpAddr>,
) -> Result<Json<Version>, Status> {
    println!("Version Check Requested");
    println!("User Data: {}", params.0.p_current_version);
    println!("Platform: {}", params.0.p_platform);
    match get_latest_version(&params.0.p_platform, pool) {
        Ok(version) => {
            info!("Valid User Data, Version Sent");
            log_data(
                pool,
                "".to_string(),
                client_ip.unwrap().to_string(),
                "/version".to_string(),
                None,
                get_timestamp(),
                "None".to_string(),
                "Version Sent".to_string(),
                "POST".to_string(),
            );
            Ok(version)
        }
        Err(err) => {
            log_data(
                pool,
                "".to_string(),
                client_ip.unwrap().to_string(),
                "/version".to_string(),
                None,
                get_timestamp(),
                "None".to_string(),
                format!("internal_error, Version Not Sent: {}", err).to_string(),
                "POST".to_string(),
            );
            error!("internal_error, Version Not Sent: {}", err);
            Err(Status::InternalServerError)
        }
    }

}
