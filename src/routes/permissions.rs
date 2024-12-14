use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, Route, State};

use crate::server::JHApiServerState;

use crate::controllers::permissions::structs::{PermissionEditParams, Permissions};

use crate::controllers::auth::decode_token_data;

use crate::server::request_guard::api_key::ApiKey;

use crate::utils::permissions::{has_admin_perm, has_permissions_perm};
use crate::utils::structs::APIError;

pub fn routes() -> Vec<Route> {
    routes![get_permissions, edit_permissions]
}

#[get("/permissions/<username>")]
pub async fn get_permissions(
    username: String,
    state: &State<JHApiServerState>,
    _key: ApiKey<'_>,
) -> Result<Json<Permissions>, Status> {
    let mut user_id: String = "".to_string();
    let pool = &state.pool;
    let sql_manager = &state.sql_manager;
    info!("/permissions/<username> Get Request: {:?}", username);
    match decode_token_data(_key.0) {
        Some(data) => {
            user_id = data.USER_ID.unwrap();
            info!("Token User Id: {:?}", user_id);
        }
        None => info!("Token Data: None"),
    }

    if !has_permissions_perm(&_key, pool, &sql_manager).await
        && !has_admin_perm(&_key, pool, &sql_manager).await
        && username.to_lowercase() != user_id.to_lowercase()
    {
        return Err(Status::Unauthorized);
    }

    match crate::controllers::permissions::get_user_permissions(
        &username.to_lowercase(),
        &sql_manager,
        &pool,
    )
    .await
    {
        Ok(permissions) => Ok(Json(permissions)),
        Err(err) => match err {
            APIError::DataNotFound => Err(Status::NotFound),
            APIError::DBError => Err(Status::InternalServerError),
            _ => Err(Status::InternalServerError),
        },
    }
}

#[post("/permissions/<username>", data = "<params>")]
pub async fn edit_permissions(
    username: String,
    params: Json<PermissionEditParams>,
    state: &State<JHApiServerState>,
    _key: ApiKey<'_>,
) -> Result<String, Status> {
    info!("/permissions/{:?} Request: {:?}", username.clone(), params);
    let pool = &state.pool;
    let sql_manager = &state.sql_manager;
    if !has_permissions_perm(&_key, pool, &sql_manager).await
        && !has_admin_perm(&_key, pool, &sql_manager).await
    {
        return Err(Status::Unauthorized);
    }
    match crate::controllers::permissions::edit_user_permissions(
        (username.clone()).to_lowercase(),
        &pool,
        &sql_manager,
        params.p_permissions.clone(),
    )
    .await
    {
        Ok(permissions) => {
            info!("Permissions Edited");
            info!("New Permissions: {:?}", permissions);
            Ok("Permissions Edited".to_string())
        }
        Err(err) => match err {
            APIError::DataNotFound => Err(Status::NotFound),
            APIError::DBError => Err(Status::InternalServerError),
            _ => Err(Status::InternalServerError),
        },
    }
}
