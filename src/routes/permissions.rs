use rocket::serde::json::Json;
use rocket::{post, State};

use oracle::pool::Pool;

use crate::permissions::structs::{PermissionEditParams, Permissions};

use crate::signing::decode_token_data;

use crate::ApiKey;

use crate::utils::permissions::{is_admin_perm, is_perm_perm};

#[get("/GetUserPermissions/<username>")]
pub async fn get_permissions(
    username: String,
    pool: &State<Pool>,
    key: ApiKey<'_>,
) -> Json<Permissions> {

    let mut userId: String = "".to_string();
    info!("GetUserPermissions Request: {:?}", username);
    match decode_token_data(key.0) {
        Some(data) => {
            userId = data.USER_ID.unwrap();
            info!("Token User Id: {:?}", userId);
        },
        None => info!("Token Data: None"),
    }

    let emptyPermissions = Permissions {
        users: None,
        permissions: None,
        query: None,
        images: None,
        cost: None,
        admin: None,
        stock: None,
        reports: None,
        stores: None,
    };

    if !is_perm_perm(&key, pool) && !is_admin_perm(&key, pool) && username.to_lowercase() != userId.to_lowercase() {
        return Json(emptyPermissions);
    }

    match crate::permissions::get_user_permissions(&username.to_lowercase(), pool) {
        Ok(permissions) => Json(permissions),
        Err(err) => {
            error!("Error: {}", err);
            Json(emptyPermissions)
        }
    }
}



#[post("/EditUserPermissions", data = "<params>")]
pub async fn edit_permissions(
    params: Json<PermissionEditParams>,
    pool: &State<Pool>,
    key: ApiKey<'_>,
) -> String {
    info!("GetUserPermissions Request: {:?}", params);
    match decode_token_data(key.0) {
        Some(data) => info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap()),
        None => info!("Token Data: None"),
    }
    if !is_perm_perm(&key, pool) && !is_admin_perm(&key, pool) {
        return "Permission Denied".to_string();
    }
    match crate::permissions::edit_user_permissions(key, (params.pUserName.clone()).to_lowercase(), pool, params.pPermissions.clone()) {
        Ok(permissions) => {
            info!("Permissions Edited");
            info!("New Permissions: {:?}", permissions);
            "Permissions Edited".to_string()
        }
        Err(err) => {
            error!("Error: {}", err);
            ("Error Editing Permissions").to_string()
        }
    }
}
