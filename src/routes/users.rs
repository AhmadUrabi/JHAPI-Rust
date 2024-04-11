use crate::functions::authentication::decode_token_data;
use crate::functions::users::structs::*;
use crate::functions::users::*;
use crate::utils::structs::APIErrors;
use crate::server::request_guard::api_key::ApiKey;

use oracle::pool::Pool;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::utils::permissions::{has_admin_perm, has_users_perm};

// Get User List
#[get("/users")]
pub async fn get_user_list(
    pool: &State<Pool>,
    _key: ApiKey<'_>,
) -> Result<Json<Vec<User>>, Status> {

    if !has_admin_perm(&_key, pool) && !has_users_perm(&_key, pool) {
        return Err(Status::Unauthorized);
    }
    match get_users(&_key, &pool).await {
        Ok(users) => Ok(Json(users)),
        Err(_error) => {
            Err(Status::InternalServerError)
        }
    }
}

#[get("/user/<user_id>")]
pub async fn get_user_by_id(
    pool: &State<Pool>,
    _key: ApiKey<'_>,
    user_id: String,
) -> Result<Json<User>, Status> {
    let mut my_user_id: String = "".to_string();

    match decode_token_data(_key.0) {
        Some(data) => {
            my_user_id = data.USER_ID.unwrap();
        }
        None => info!("Token Data: None"),
    }

    if !has_admin_perm(&_key, pool)
        && !has_users_perm(&_key, pool)
        && my_user_id.to_lowercase() != user_id.to_lowercase()
    {
        return Err(Status::Unauthorized);
    }

    match get_user(&user_id, pool).await {
        Ok(user) => {
            Ok(Json(user))
        }
        Err(_error) => {
            Err(Status::NotFound)
        }
    }
}

#[post("/user", data = "<params>")]
pub async fn create_user_route(
    params: Json<NewUser>,
    pool: &State<Pool>,
    _key: ApiKey<'_>,
) -> Result<String, Status> {
    println!(
        "Create User Request: {:?}, {:?}",
        params.0.p_username, params.0.p_fullname
    );
    if !has_admin_perm(&_key, pool) && !has_users_perm(&_key, pool) {
        return Err(Status::Unauthorized);
    }
    match create_user(params.0, pool).await {
        Ok(_) => {
            Ok("User Created".to_string())
        }
        Err(error) => {
            match error {
                APIErrors::UserExists => Err(Status::Conflict),
                APIErrors::DBError => Err(Status::InternalServerError),
                _ => Err(Status::InternalServerError),
            }
        }
    }
}

#[put("/user/<username>", data = "<params>")]
pub async fn edit_user_route(
    username: &str,
    params: Json<EditUserParams>,
    pool: &State<Pool>,
    _key: ApiKey<'_>,
) -> Result<String, Status> {
    // Check deserialization
    if params.0.p_password.is_none()
        && params.0.p_fullname.is_none()
        && params.0.p_email.is_none()
        && params.0.p_loginduration.is_none()
    {
        return Err(Status::BadRequest);
    }

    println!("Edit User Request: {:?}", username);
    if !has_admin_perm(&_key, pool) && !has_users_perm(&_key, pool) {
        return Err(Status::Unauthorized);
    }
    let res = edit_user(params, username, pool, has_admin_perm(&_key, pool)).await;

    if res.is_err() {
        let error = res.err().unwrap();
        match error {
            APIErrors::UserNotFound => return Err(Status::NotFound),
            APIErrors::DBError => return Err(Status::InternalServerError),
            _ => return Err(Status::InternalServerError),
        }
    }
    Ok("User Edited".to_string())
}

#[delete("/user/<user_id>")]
pub async fn delete_user_route(
    pool: &State<Pool>,
    _key: ApiKey<'_>,
    user_id: String,
) -> Result<String, Status> {
    if !has_admin_perm(&_key, pool) && !has_users_perm(&_key, pool) {
        return Err(Status::Unauthorized);
    }
    match delete_user(&user_id, pool).await {
        Ok(_) => {
            Ok("User Deleted".to_string())
        }
        Err(error) => {
            match error {
                APIErrors::UserNotFound => Err(Status::NotFound),
                APIErrors::DBError => Err(Status::InternalServerError),
                _ => Err(Status::InternalServerError),
            }
        }
    }
}

/*
// Edit User
#[post("/EditUser", data = "<params>")]
pub async fn edit_user(params: Json<crate::user_control::EditUserParams>, pool: &State<Pool>) {
    crate::user_control::edit_user(params, pool).await.unwrap();
}
*/
