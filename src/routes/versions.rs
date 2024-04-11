use oracle::pool::Pool;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, State};


use crate::functions::versions::get_latest_version;
use crate::functions::versions::structs::*;

#[post("/version", data = "<params>")]
pub async fn route_version_check(
    params: Json<VersionParams>,
    pool: &State<Pool>,
) -> Result<Json<Version>, Status> {
    println!("Version Check Requested");
    match get_latest_version(&params.0.p_platform, pool).await {
        Ok(version) => {
            info!("Valid User Data, Version Sent");
            Ok(version)
        }
        Err(err) => {
            error!("internal_error, Version Not Sent: {}", err);
            Err(Status::InternalServerError)
        }
    }
}
