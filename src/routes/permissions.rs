use rocket::serde::json::Json;
use rocket::{State, post};

use oracle::pool::Pool;

use crate::permissions::structs::PermissionEditParams;

use crate::signing::decode_token_data;

use crate::ApiKey;

use crate::utils::{is_perm_perm, is_admin_perm};


#[get("/GetUserPermissions/<username>")]
pub async fn get_permissions(username: String, pool: &State<Pool>, key: ApiKey<'_>) -> Json<Vec<String>> {
    info!("GetUserPermissions Request: {:?}", username);
    match decode_token_data(key.0) {
        Some(data) => info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap()),
        None => info!("Token Data: None"),
    }

    if !is_perm_perm(&key, pool) || !is_admin_perm(&key, pool) {
        return Json(vec![]);
    }

    match crate::permissions::get_user_permissions(&username, pool) {
        Ok(permissions) => Json(permissions),
        Err(err) => {
            error!("Error: {}", err);
            Json(vec![])
        },
    }
}



#[post("/EditUserPermissions", data = "<params>")]
pub async fn edit_permissions(params: Json<PermissionEditParams>, pool: &State<Pool>, key: ApiKey<'_>) -> String {
    
    info!("GetUserPermissions Request: {:?}", params);
    match decode_token_data(key.0) {
        Some(data) => info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap()),
        None => info!("Token Data: None"),
    }
    if !is_perm_perm(&key, pool) || !is_admin_perm(&key, pool) {
        return "Permission Denied".to_string();
    }
    match crate::permissions::edit_user_permissions(key, pool, params.pPermissions.clone()) {
        Ok(permissions) => {
            info!("Permissions Edited");
            info!("New Permissions: {:?}", permissions);
            "Permissions Edited".to_string()
        },
        Err(err) => {
            error!("Error: {}", err);
            ("Error Editing Permissions").to_string()
        },
    }
}