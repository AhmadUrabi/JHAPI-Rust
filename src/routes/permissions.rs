use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, State};

use oracle::pool::Pool;

use crate::functions::permissions::structs::{PermissionEditParams, Permissions};

use crate::functions::authentication::decode_token_data;

use crate::server::request_guard::api_key::ApiKey;

use crate::utils::permissions::{has_admin_perm, has_permissions_perm};
use crate::utils::structs::APIErrors;


#[get("/permissions/<username>")]
pub async fn get_permissions(
    username: String,
    pool: &State<Pool>,
    _key: ApiKey<'_>,
) -> Result<Json<Permissions>, Status> {
    let mut user_id: String = "".to_string();
    info!("/permissions/<username> Get Request: {:?}", username);
    match decode_token_data(_key.0) {
        Some(data) => {
            user_id = data.USER_ID.unwrap();
            info!("Token User Id: {:?}", user_id);
        }
        None => info!("Token Data: None"),
    }

    if !has_permissions_perm(&_key, pool).await
        && !has_admin_perm(&_key, pool).await
        && username.to_lowercase() != user_id.to_lowercase()
    {
        return Err(Status::Unauthorized);
    }

    match crate::functions::permissions::get_user_permissions(&username.to_lowercase(), pool).await {
        Ok(permissions) => {
            Ok(Json(permissions))
        }
        Err(err) => {
            match err {
                APIErrors::UserNotFound => Err(Status::NotFound),
                APIErrors::DBError => Err(Status::InternalServerError),
                _ => Err(Status::InternalServerError),
            }
        }
    }
}

#[post("/permissions/<username>", data = "<params>")]
pub async fn edit_permissions(
    username: String,
    params: Json<PermissionEditParams>,
    pool: &State<Pool>,
    _key: ApiKey<'_>,
) -> Result<String, Status> {
    info!("/permissions/{:?} Request: {:?}", username.clone(), params);

    if !has_permissions_perm(&_key, pool).await && !has_admin_perm(&_key, pool).await {
        return Err(Status::Unauthorized);
    }
    match crate::functions::permissions::edit_user_permissions(
        (username.clone()).to_lowercase(),
        &pool,
        params.p_permissions.clone(),
    ).await {
        Ok(permissions) => {
            info!("Permissions Edited");
            info!("New Permissions: {:?}", permissions);
            Ok("Permissions Edited".to_string())
        }
        Err(err) => {
            match err {
                APIErrors::UserNotFound => Err(Status::NotFound),
                APIErrors::DBError => Err(Status::InternalServerError),
                _ => Err(Status::InternalServerError),
            }
        }
    }
}
