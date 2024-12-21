use crate::server::JHApiServerState;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{Route, State};

use crate::server::request_guard::api_key::ApiKey;

use crate::utils::permissions::has_admin_perm;

use crate::controllers::logs::LogData;

pub fn routes() -> Vec<Route> {
    routes![
        get_all_logs,
        get_user_logs,
        delete_user_logs,
        delete_log_logs
    ]
}
//  TODO: Migrate to file based Logs
#[get("/logs?<limit>")]
pub async fn get_all_logs(
    state: &State<JHApiServerState>,
    _key: ApiKey<'_>,
    limit: Option<i32>,
) -> Result<Json<Vec<LogData>>, Status> {
    let pool = &state.pool;
    let sql_manager = &state.sql_manager;
    if !has_admin_perm(&_key, pool, &sql_manager).await {
        return Err(Status::Unauthorized);
    }

    match crate::controllers::logs::get_all_logs_fn(&pool, &sql_manager, limit).await {
        Ok(logs) => Ok(logs),
        Err(_err) => {
            return Err(Status::InternalServerError);
        }
    }
}

#[get("/logs/user/<username>?<limit>")]
pub async fn get_user_logs(
    state: &State<JHApiServerState>,
    _key: ApiKey<'_>,
    username: String,
    limit: Option<i32>,
) -> Result<Json<Vec<LogData>>, Status> {
    let pool = &state.pool;
    let sql_manager = &state.sql_manager;
    if !has_admin_perm(&_key, pool, &sql_manager).await {
        return Err(Status::Unauthorized);
    }

    match crate::controllers::logs::get_user_logs_fn(username, &pool, &sql_manager, limit).await {
        Ok(logs) => Ok(logs),
        Err(_err) => Err(Status::InternalServerError),
    }
}

// TODO: Fix this route
// Unused, should handle nested routes
/*
#[get("/logs/route/<route>?<limit>")]
pub async fn get_route_logs(state: &State<JHApiServerState>, _key: ApiKey<'_> , route: String,limit: Option<i32>, client_ip: Option<IpAddr>) -> Result<Json<Vec<LogData>>, Status> {
    let mut userId: String = "".to_string();

    match decode_token_data(_key.0) {
        Some(data) => {
            userId = data.USER_ID.unwrap();
        }
        None => info!("Token Data: None"),
    }

    let route_clone = route.clone();

    if !is_admin_perm(&_key, pool) {
        log_data(
            pool,
            userId,
            client_ip.unwrap().to_string(),
            ("/logs/route/".to_owned()+&route_clone).to_string(),
            None,
            get_timestamp(),
            _key.0.to_string(),
            "Unauthorized".to_string(),
            "GET".to_string()
        );
        return Err(Status::Unauthorized);
    }

    let r = "/".to_owned()+&route_clone;

    match crate::logs::get_route_logs_fn(r, pool, limit) {
        Ok(logs) => {
            log_data(
                pool,
                userId,
                client_ip.unwrap().to_string(),
                ("/logs/route/".to_owned()+&route_clone).to_string(),
                None,
                get_timestamp(),
                _key.0.to_string(),
                "Success".to_string(),
                "GET".to_string()
            );
            Ok(logs)
        }
        Err(_err) => {
            Err(Status::InternalServerError)
        }
    }


}
*/

#[delete("/logs/user/<username>?<limit>")]
pub async fn delete_user_logs(
    state: &State<JHApiServerState>,
    _key: ApiKey<'_>,
    username: String,
    limit: Option<i32>,
) -> Result<String, Status> {
    let pool = &state.pool;
    let sql_manager = &state.sql_manager;
    if !has_admin_perm(&_key, pool, &sql_manager).await {
        return Err(Status::Unauthorized);
    }

    match crate::controllers::logs::delete_user_logs_fn(username, &pool, &sql_manager, limit).await
    {
        Ok(_logs) => Ok("Logs Deleted".to_string()),
        Err(_err) => Err(Status::InternalServerError),
    }
}

#[delete("/logs/<log_id>")]
pub async fn delete_log_logs(
    state: &State<JHApiServerState>,
    _key: ApiKey<'_>,
    log_id: i32,
) -> Result<String, Status> {
    let pool = &state.pool;
    let sql_manager = &state.sql_manager;
    if !has_admin_perm(&_key, pool, &sql_manager).await {
        return Err(Status::Unauthorized);
    }

    match crate::controllers::logs::delete_log_logs_fn(log_id, &pool, &sql_manager).await {
        Ok(_logs) => Ok("Logs Deleted".to_string()),
        Err(_err) => Err(Status::InternalServerError),
    }
}
