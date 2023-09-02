// TODO: Fix everything

use rocket::serde::json::Json;
use rocket::{State, post};
use oracle::pool::Pool;

use crate::signing::decode_token_data;
use crate::signing::
use crate::ApiKey;

struct FetchParams {
    pub pUserName: String,
}

#[post("/get_user_permissions", data = "<params>")]
pub async fn get_user_permissions(params: Json<FetchParams>, pool: &State<Pool>, key: ApiKey<'_>) -> Json<Vec<Permission>> {
    
    info!("GetUserPermissions Request: {:?}", params);
    match decode_token_data(key.0) {
        Some(data) => info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap()),
        None => info!("Token Data: None"),
    }

    // No error handling as the function will always return a result
    //Json(get_product(params, pool, key).unwrap())
    match get_user_permissions(params, pool, key) {
        Ok(permissions) => Json(permissions),
        Err(err) => {
            error!("Error: {}", err);
            Json(vec![])
        },
    }

}