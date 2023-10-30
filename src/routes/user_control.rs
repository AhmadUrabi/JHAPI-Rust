use crate::LogCheck;
use crate::signing::decode_token_data;
use crate::user_control::*;
use crate::ApiKey;

use oracle::pool::Pool;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use std::net::IpAddr;

use crate::utils::logging::{getTimestamp, log_data};
use crate::utils::permissions::{is_admin_perm, is_users_perm};

// Get User List
#[get("/UserList")]
pub async fn get_user_list(
    pool: &State<Pool>,
    _key: ApiKey<'_>,
    client_ip: Option<IpAddr>,
    log_check: LogCheck,
) -> Result<Json<Vec<User>>, Status> {
    let mut userId = "".to_string();

    match decode_token_data(_key.0) {
        Some(data) => {
            userId = data.USER_ID.unwrap();
        }
        None => info!("Token Data: None"),
    }

    if !is_admin_perm(&_key, pool) && !is_users_perm(&_key, pool) {
        if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
        log_data(
            pool,
            userId,
            client_ip.unwrap().to_string(),
            "/UserList".to_string(),
            None,
            getTimestamp(),
            _key.0.to_string(),
            "Unauthorized".to_string(),
        );
    }
        return Ok(Json(Vec::new()));
    }
    if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
    log_data(
        pool,
        userId,
        client_ip.unwrap().to_string(),
        "/UserList".to_string(),
        None,
        getTimestamp(),
        _key.0.to_string(),
        "Success".to_string(),
    );
}
    Ok(Json(get_users(_key, pool).await.unwrap()))
}

#[get("/User/<user_id>")]
pub async fn get_user_by_id(
    pool: &State<Pool>,
    _key: ApiKey<'_>,
    user_id: String,
    client_ip: Option<IpAddr>,
    log_check: LogCheck,
) -> Result<Json<User>, Status> {
    let mut userId: String = "".to_string();

    match decode_token_data(_key.0) {
        Some(data) => {
            userId = data.USER_ID.unwrap();
        }
        None => info!("Token Data: None"),
    }

    if !is_admin_perm(&_key, pool)
        && !is_users_perm(&_key, pool)
        && user_id.to_lowercase() != userId.to_lowercase()
    {
        if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
        log_data(
            pool,
            userId,
            client_ip.unwrap().to_string(),
            ("/User/".to_owned() + &user_id).to_string(),
            None,
            getTimestamp(),
            _key.0.to_string(),
            "Unauthorized".to_string(),
        );
    }
        return Err(Status::Unauthorized);
    }

    match get_user(&user_id, pool).await {
        Ok(user) => {
            if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
            log_data(
                pool,
                userId,
                client_ip.unwrap().to_string(),
                ("/User/".to_owned() + &user_id).to_string(),
                None,
                getTimestamp(),
                _key.0.to_string(),
                "Success".to_string(),
            );
        }
            Ok(Json(user))
        }
        Err(error) => {
            if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
            log_data(
                pool,
                userId,
                client_ip.unwrap().to_string(),
                ("/User/".to_owned() + &user_id).to_string(),
                None,
                getTimestamp(),
                _key.0.to_string(),
                error.to_string(),
            );
        }
            Err(Status::NotFound)
        }
    }
}

#[post("/CreateUser", data = "<params>")]
pub async fn create_user_route(
    params: Json<NewUser>,
    pool: &State<Pool>,
    _key: ApiKey<'_>,
    client_ip: Option<IpAddr>,
    log_check: LogCheck,
) -> Result<String, Status> {
    let mut userId: String = "".to_string();
    match decode_token_data(_key.0) {
        Some(data) => {
            userId = data.USER_ID.unwrap();
        }
        None => info!("Token Data: None"),
    }

    println!(
        "Create User Request: {:?}, {:?}",
        params.0.username, params.0.fullname
    );
    if !is_admin_perm(&_key, pool) && !is_users_perm(&_key, pool) {
        if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
        log_data(
            pool,
            userId,
            client_ip.unwrap().to_string(),
            "/CreateUser".to_string(),
            None,
            getTimestamp(),
            _key.0.to_string(),
            "Unauthorized".to_string(),
        );
    }
        return Err(Status::Unauthorized);
    }
    match create_user(params.0, pool).await {
        Ok(_) => {
            if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
            log_data(
                pool,
                userId,
                client_ip.unwrap().to_string(),
                "/CreateUser".to_string(),
                None,
                getTimestamp(),
                _key.0.to_string(),
                "Success".to_string(),
            );
        }
            Ok("User Created".to_string())
        }
        Err(error) => {
            if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
            log_data(
                pool,
                userId,
                client_ip.unwrap().to_string(),
                "/CreateUser".to_string(),
                None,
                getTimestamp(),
                _key.0.to_string(),
                error.to_string(),
            );
        }
            Err(Status::InternalServerError)
        }
    }
}

#[put("/EditUser", data = "<params>")]
pub async fn edit_user_route(
    params: Json<EditUserParams>,
    pool: &State<Pool>,
    _key: ApiKey<'_>,
    client_ip: Option<IpAddr>,
    log_check: LogCheck,
) -> Result<String, Status> {
    let mut userId: String = "".to_string();
    match decode_token_data(_key.0) {
        Some(data) => {
            userId = data.USER_ID.unwrap();
        }
        None => info!("Token Data: None"),
    }

    println!("Edit User Request: {:?}", params.0.username);
    if !is_admin_perm(&_key, pool) && !is_users_perm(&_key, pool) {
        if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
        log_data(
            pool,
            userId,
            client_ip.unwrap().to_string(),
            "/EditUser".to_string(),
            None,
            getTimestamp(),
            _key.0.to_string(),
            "Unauthorized".to_string(),
        );
    }
        return Err(Status::Unauthorized);
    }
    let res = edit_user(params, pool, is_admin_perm(&_key, pool))
        .await
        .unwrap();
    if res == false {
        if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
        log_data(
            pool,
            userId,
            client_ip.unwrap().to_string(),
            "/EditUser".to_string(),
            None,
            getTimestamp(),
            _key.0.to_string(),
            "User Not Found".to_string(),
        );
    }
        return Err(Status::NotFound);
    }
    if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
    log_data(
        pool,
        userId,
        client_ip.unwrap().to_string(),
        "/EditUser".to_string(),
        None,
        getTimestamp(),
        _key.0.to_string(),
        "Success".to_string(),
    );
}
    Ok("User Edited".to_string())
}

#[delete("/DeleteUser/<user_id>")]
pub async fn delete_user_route(
    pool: &State<Pool>,
    _key: ApiKey<'_>,
    user_id: String,
    client_ip: Option<IpAddr>,
    log_check: LogCheck,
) -> Result<String, Status> {
    let mut userId: String = "".to_string();
    match decode_token_data(_key.0) {
        Some(data) => {
            userId = data.USER_ID.unwrap();
        }
        None => info!("Token Data: None"),
    }

    if !is_admin_perm(&_key, pool) && !is_users_perm(&_key, pool) {
        if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
        log_data(
            pool,
            userId,
            client_ip.unwrap().to_string(),
            ("/DeleteUser/".to_owned() + &user_id).to_string(),
            None,
            getTimestamp(),
            _key.0.to_string(),
            "Unauthorized".to_string(),
        );
    }
        return Err(Status::Unauthorized);
    }
    match delete_user(&user_id, pool).await {
        Ok(_) => {
            if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
            log_data(
                pool,
                userId,
                client_ip.unwrap().to_string(),
                ("/DeleteUser/".to_owned() + &user_id).to_string(),
                None,
                getTimestamp(),
                _key.0.to_string(),
                "Success".to_string(),
            );
        }
            Ok("User Deleted".to_string())
        }
        Err(error) => {
            if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
            log_data(
                pool,
                userId,
                client_ip.unwrap().to_string(),
                ("/DeleteUser/".to_owned() + &user_id).to_string(),
                None,
                getTimestamp(),
                _key.0.to_string(),
                error.to_string(),
            );
        }
            Err(Status::InternalServerError)
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
