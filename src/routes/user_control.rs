use crate::signing::decode_token_data;
use crate::user_control::*;
use crate::ApiKey;

use oracle::pool::Pool;

use rocket::serde::json::Json;
use rocket::State;

use crate::utils::permissions::{is_admin_perm, is_users_perm};

// Get User List
#[get("/UserList")]
pub async fn get_user_list(pool: &State<Pool>, _key: ApiKey<'_>) -> Json<Vec<User>> {
    if !is_admin_perm(&_key, pool) && !is_users_perm(&_key, pool) {
        return Json(Vec::new());
    }
    Json(get_users(_key, pool).await.unwrap())
}

#[get("/User/<user_id>")]
pub async fn get_user_by_id(pool: &State<Pool>, _key: ApiKey<'_>, user_id: String) -> Json<User> {

    let mut userId: String = "".to_string();

    match decode_token_data(_key.0) {
        Some(data) => {
            userId = data.USER_ID.unwrap();
        },
        None => info!("Token Data: None"),
    }

    if !is_admin_perm(&_key, pool) && !is_users_perm(&_key, pool) && user_id.to_lowercase() != userId.to_lowercase() {
        return Json(User {
            username: "".to_string(),
            fullname: "".to_string(),
            email: "".to_string(),
            login_duration: 0,
        });
    }
    Json(get_user(&user_id, pool).await.unwrap())
}

#[post("/CreateUser", data = "<params>")]
pub async fn create_user_route(
    params: Json<NewUser>,
    pool: &State<Pool>,
    _key: ApiKey<'_>,
) -> String {
    println!("Create User Request: {:?}, {:?}", params.0.username, params.0.fullname);
    if !is_admin_perm(&_key, pool) && !is_users_perm(&_key, pool) {
        return "Permission Denied".to_string();
    }
    create_user(params.0, pool).await.unwrap();
    "User Created".to_string()
}

#[put("/EditUser", data = "<params>")]
pub async fn edit_user_route(
    params: Json<EditUserParams>,
    pool: &State<Pool>,
    _key: ApiKey<'_>,
) -> String {
    println!("Edit User Request: {:?}", params.0.username);
    if !is_admin_perm(&_key, pool) && !is_users_perm(&_key, pool) {
        return "Permission Denied".to_string();
    }
    let res = edit_user(params, pool, is_admin_perm(&_key, pool)).await.unwrap();
    if res == false {
        return "User Not Found".to_string();
    }
    "User Edited".to_string()
}

#[delete("/DeleteUser/<user_id>")]
pub async fn delete_user_route(pool: &State<Pool>, _key: ApiKey<'_>, user_id: String) -> String {
    if !is_admin_perm(&_key, pool) && !is_users_perm(&_key, pool) {
        return "Permission Denied".to_string();
    }
    delete_user(&user_id, pool).await.unwrap();
    "User Deleted".to_string()
}

/*
// Edit User
#[post("/EditUser", data = "<params>")]
pub async fn edit_user(params: Json<crate::user_control::EditUserParams>, pool: &State<Pool>) {
    crate::user_control::edit_user(params, pool).await.unwrap();
}
*/
