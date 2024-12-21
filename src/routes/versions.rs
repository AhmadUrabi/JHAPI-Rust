use crate::respond;
use crate::server::response::ApiResponse;
use crate::server::JHApiServerState;
use rocket::serde::json::Json;
use rocket::{post, Route, State};

use crate::controllers::versions::*;

pub fn routes() -> Vec<Route> {
    routes![route_version_check]
}

#[post("/version", data = "<params>")]
pub async fn route_version_check(
    params: Json<VersionParams>,
    state: &State<JHApiServerState>,
) -> ApiResponse {
    let pool = &state.pool;
    let sql_manager = &state.sql_manager;
    println!("Version Check Requested");
    match get_latest_version(&params.0.p_platform, sql_manager, pool).await {
        Ok(version) => {
            info!("Valid User Data, Version Sent");
            respond!(200, "Latest Version Found", version.0)
        }
        Err(err) => {
            error!("internal_error, Version Not Sent: {}", err);
            err.into()
        }
    }
}
