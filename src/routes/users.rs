use crate::controllers::auth::decode_token_data;
use crate::controllers::users::structs::*;
use crate::controllers::users::*;
use crate::respond;
use crate::server::request_guard::api_key::ApiKey;
use crate::server::response::ApiResponse;
use crate::utils::structs::APIError;

use crate::server::JHApiServerState;

use rocket::serde::json::Json;
use rocket::State;
use rocket::{http::Status, Route};

use crate::utils::permissions::{has_admin_perm, has_users_perm};

pub fn routes() -> Vec<Route> {
    routes![
        get_user_list,
        get_user_by_id,
        create_user_route,
        edit_user_route,
        delete_user_route
    ]
}

// Get User List
#[get("/users")]
pub async fn get_user_list(state: &State<JHApiServerState>, _key: ApiKey<'_>) -> ApiResponse {
    let pool = &state.pool;
    let sql_manager = &state.sql_manager;
    if !has_admin_perm(&_key, pool, &sql_manager).await
        && !has_users_perm(&_key, pool, &sql_manager).await
    {
        return respond!(401, "User is Unauthorized");
    }
    match get_users(&_key, &sql_manager, &pool).await {
        Ok(users) => respond!(200, "User list Found", users),
        Err(error) => error.into(),
    }
}

#[get("/user/<user_id>")]
pub async fn get_user_by_id(
    state: &State<JHApiServerState>,
    _key: ApiKey<'_>,
    user_id: String,
) -> ApiResponse {
    let mut my_user_id: String = "".to_string();
    let pool = &state.pool;
    let sql_manager = &state.sql_manager;
    match decode_token_data(_key.0) {
        Some(data) => {
            my_user_id = data.USER_ID.unwrap();
        }
        None => info!("Token Data: None"),
    }

    if !has_admin_perm(&_key, pool, &sql_manager).await
        && !has_users_perm(&_key, pool, &sql_manager).await
        && my_user_id.to_lowercase() != user_id.to_lowercase()
    {
        return respond!(401, "User is Unauthorized");
    }

    match get_user(&user_id, &sql_manager, pool).await {
        Ok(user) => respond!(200, "User Found", user),
        Err(error) => error.into(),
    }
}

#[post("/user", data = "<params>")]
pub async fn create_user_route(
    params: Json<NewUser>,
    state: &State<JHApiServerState>,
    _key: ApiKey<'_>,
) -> ApiResponse {
    println!(
        "Create User Request: {:?}, {:?}",
        params.0.p_username, params.0.p_fullname
    );
    let pool = &state.pool;
    let sql_manager = &state.sql_manager;
    if !has_admin_perm(&_key, &pool, &sql_manager).await
        && !has_users_perm(&_key, &pool, &sql_manager).await
    {
        return respond!(401, "User is Unauthorized");
    }
    match create_user(params.0, &sql_manager, &pool).await {
        Ok(_) => respond!(201, "User Crated"),
        Err(error) => error.into(),
    }
}

#[put("/user/<username>", data = "<params>")]
pub async fn edit_user_route(
    username: &str,
    params: Json<EditUserParams>,
    state: &State<JHApiServerState>,
    _key: ApiKey<'_>,
) -> ApiResponse {
    let pool = &state.pool;
    let sql_manager = &state.sql_manager;
    // Check deserialization
    if params.0.p_password.is_none()
        && params.0.p_fullname.is_none()
        && params.0.p_email.is_none()
        && params.0.p_loginduration.is_none()
    {
        respond!(422, "Invalid Request Body");
    }

    println!("Edit User Request: {:?}", username);
    if !has_admin_perm(&_key, &pool, &sql_manager).await
        && !has_users_perm(&_key, &pool, &sql_manager).await
    {
        return respond!(401, "User is Unauthorized");
    }
    let perm = has_admin_perm(&_key, &pool, &sql_manager).await.clone();

    match edit_user(params.0.clone(), username, &pool, &sql_manager, perm).await {
        Ok(_) => respond!(200, "User Edited Sucessfully"),
        Err(error) => error.into(),
    }
}

#[delete("/user/<user_id>")]
pub async fn delete_user_route(
    state: &State<JHApiServerState>,
    _key: ApiKey<'_>,
    user_id: String,
) -> ApiResponse {
    let pool = &state.pool;
    let sql_manager = &state.sql_manager;
    if !has_admin_perm(&_key, &pool, &sql_manager).await
        && !has_users_perm(&_key, &pool, &sql_manager).await
    {
        return respond!(401, "Unauthorized");
    }
    match delete_user(&user_id, &sql_manager, &pool).await {
        Ok(_) => respond!(200, "User Deleted Successfully"),
        Err(error) => error.into(),
    }
}

/*
// Edit User
#[post("/EditUser", data = "<params>")]
pub async fn edit_user(params: Json<crate::user_control::EditUserParams>, state: &State<JHApiServerState>) {
    crate::user_control::edit_user(params, pool).await.unwrap();
}
*/
