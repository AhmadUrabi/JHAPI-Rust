// TODO: Fix everything

use rocket::serde::json::Json;
use rocket::{State, post};
use oracle::pool::Pool;

use crate::signing::decode_token_data;

use crate::ApiKey;


use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct FetchParams {
    pub pUserName: String,
}

#[post("/get_permissions", data = "<params>")]
pub async fn get_permissions(params: Json<FetchParams>, pool: &State<Pool>, key: ApiKey<'_>) -> Json<Vec<String>> {
    info!("GetUserPermissions Request: {:?}", params);
    match decode_token_data(key.0) {
        Some(data) => info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap()),
        None => info!("Token Data: None"),
    }

    match crate::permissions::get_user_permissions(&params.pUserName, pool) {
        Ok(permissions) => Json(permissions),
        Err(err) => {
            error!("Error: {}", err);
            Json(vec![])
        },
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PermissionEditParams {
    pub pUserName: String,
    pub pPermissions: Vec<String>,
}


#[post("/edit_permissions", data = "<params>")]
pub async fn edit_permissions(params: Json<PermissionEditParams>, pool: &State<Pool>, key: ApiKey<'_>) -> Json<Vec<String>> {
    
    info!("GetUserPermissions Request: {:?}", params);
    match decode_token_data(key.0) {
        Some(data) => info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap()),
        None => info!("Token Data: None"),
    }

    match crate::permissions::edit_user_permissions(key, pool, params.pPermissions.clone()) {
        Ok(permissions) => Json(vec![permissions]),
        Err(err) => {
            error!("Error: {}", err);
            Json(vec![])
        },
    }

}