use crate::server::JHApiServerState;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, State};


use crate::functions::versions::get_latest_version;
use crate::functions::versions::structs::*;

#[post("/version", data = "<params>")]
pub async fn route_version_check(
    params: Json<VersionParams>,
    state: &State<JHApiServerState>,
) -> Result<Json<Version>, Status> {
    let pool = &state.pool;
    let sql_manager = &state.sql_manager;
    println!("Version Check Requested");
    match get_latest_version(&params.0.p_platform, sql_manager, pool).await {
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
