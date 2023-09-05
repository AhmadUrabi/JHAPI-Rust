use crate::user_control::*;
use crate::ApiKey;

use oracle::pool::Pool;

use rocket::State;
use rocket::serde::json::Json;

use crate::utils::permissions::{is_admin_perm, is_users_perm};


// Get User List
#[get("/UserList")]
pub async fn get_user_list(pool: &State<Pool>, _key: ApiKey<'_>) -> Json<Vec<User>> {
    if !is_admin_perm(&_key, pool) || !is_users_perm(&_key, pool) {
        return Json(Vec::new());
    }
    Json(get_users(_key, pool).await.unwrap())
}

#[get("/User/<user_id>")]
pub async fn get_user_by_id(pool: &State<Pool>, _key: ApiKey<'_>, user_id: String) -> Json<User> {
    if !is_admin_perm(&_key, pool) || !is_users_perm(&_key, pool) {
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
pub async fn create_user_route(params: Json<NewUser>, pool: &State<Pool>, _key: ApiKey<'_>) -> String {
    println!("Create User Request: {:?}", params.0);
    if !is_admin_perm(&_key, pool) || !is_users_perm(&_key, pool){
        return "Permission Denied".to_string();
    }
    create_user(params.0, pool).await.unwrap();
    "User Created".to_string()
}



/*
// Edit User
#[post("/EditUser", data = "<params>")]
pub async fn edit_user(params: Json<crate::user_control::EditUserParams>, pool: &State<Pool>) {
    crate::user_control::edit_user(params, pool).await.unwrap();
}
*/