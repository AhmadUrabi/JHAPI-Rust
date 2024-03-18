use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, State};

use oracle::pool::Pool;

use crate::permissions::structs::{PermissionEditParams, Permissions};

use crate::signing::decode_token_data;

use crate::ApiKey;

use crate::utils::permissions::{is_admin_perm, is_perm_perm};
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

    if !is_perm_perm(&_key, pool)
        && !is_admin_perm(&_key, pool)
        && username.to_lowercase() != user_id.to_lowercase()
    {
        return Err(Status::Unauthorized);
    }

    match crate::permissions::get_user_permissions(&username.to_lowercase(), pool) {
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

    if !is_perm_perm(&_key, pool) && !is_admin_perm(&_key, pool) {
        return Err(Status::Unauthorized);
    }
    match crate::permissions::edit_user_permissions(
        (username.clone()).to_lowercase(),
        pool,
        params.p_permissions.clone(),
    ) {
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
