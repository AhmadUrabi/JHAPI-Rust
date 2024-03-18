use oracle::pool::Pool;
use rocket::http::Status;
use rocket::log::private::info;
use rocket::serde::json::Json;
use rocket::{get, State};

use crate::fetch_stores::structs::*;

use crate::utils::structs::APIErrors;
use crate::ApiKey;

use crate::fetch_stores::get_stores;

use crate::signing::decode_token_data;

use crate::utils::{check_user_exists, permissions::*};

use crate::fetch_stores::structs::Store;


#[get("/stores")]
pub async fn get_store_list(
    pool: &State<Pool>,
    _key: ApiKey<'_>,
) -> Result<Json<Vec<Store>>, Status> {
    info!("Stores Get Request");
    let mut user_id: String = "".to_string();
    match decode_token_data(_key.0) {
        Some(data) => {
            info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap());
            user_id = data.USER_ID.unwrap();
        }
        None => info!("Token Data: None"),
    }

    if is_stores_perm(&_key, &pool) || is_admin_perm(&_key, &pool) {
        match get_stores(pool, "admin".to_string()) {
            Ok(stores) => {
                return Ok(Json(stores));
            }
            Err(err) => {
                match err {
                    APIErrors::DBError => return Err(Status::InternalServerError),
                    APIErrors::UserNotFound => return Err(Status::NotFound),
                    _ => return Err(Status::InternalServerError),
                }
            }
        }
    }
    match get_stores(pool, user_id) {
        Ok(stores) => {
            Ok(Json(stores))
        }
        Err(err) => {
            match err {
                APIErrors::DBError => return Err(Status::InternalServerError),
                APIErrors::UserNotFound => return Err(Status::NotFound),
                _ => return Err(Status::InternalServerError),
            }
        }
    }
}

#[post("/stores", data = "<params>")]
pub async fn update_store_list(
    pool: &State<Pool>,
    _key: ApiKey<'_>,
    params: Json<StoreListUpdateParams>,
) -> Result<String, Status> {
    info!("stores Request: {:?}", params);
    if is_admin_perm(&_key, pool) || is_stores_perm(&_key, pool) {
        info!("User has permissions");
    } else {
        info!("User does not have permissions");
        return Err(Status::Unauthorized);
    }

    // TODO: Whole function should be separated from route function
    match check_user_exists(params.0.p_username.clone(), pool) {
        Ok(x) => {
            if !x {
                return Err(Status::NotFound);
            } else {
                println!("User exists");
            }
        }
        Err(_err) => {
            return Err(Status::InternalServerError);
        }
    }

    let conn = pool.get().unwrap();
    // Delete previous values, if all access stores is set to one, just add a single row, else, add a row for each store
    if !params.p_stores.is_none() || params.p_allstoresaccess == 0 {
        let mut stmt = conn
            .statement(
                "
            DELETE FROM ODBC_JHC.USER_STORES_JHC
            WHERE USERNAME = :username",
            )
            .build()
            .unwrap();

        stmt.execute(&[&params.p_username]).unwrap();
        conn.commit().unwrap();
    }

    if params.p_allstoresaccess == 1 {
        let mut stmt = conn
            .statement(
                "
                INSERT INTO ODBC_JHC.USER_STORES_JHC (USERNAME, ALL_STORES_ACCESS)
                VALUES (:username, 1)",
            )
            .build()
            .unwrap();

        stmt.execute(&[&params.p_username]).unwrap();
        conn.commit().unwrap();
    } else {
        for store in params.p_stores.as_ref().unwrap().iter() {
            let mut stmt = conn
                .statement(
                    "
                    INSERT INTO ODBC_JHC.USER_STORES_JHC (USERNAME, STORE_ID)
                    VALUES (:username, :store_id)",
                )
                .build()
                .unwrap();

            stmt.execute(&[&params.p_username, store]).unwrap();
            conn.commit().unwrap();
        }
    }

    return Ok("Success".to_string());
}

#[get("/stores/<username>")]
pub async fn get_store_list_for_user(
    pool: &State<Pool>,
    _key: ApiKey<'_>,
    username: String,
) -> Result<Json<Vec<Store>>, Status> {
    info!("User stores Request");

    if !is_stores_perm(&_key, &pool) && !is_admin_perm(&_key, &pool) {
        info!("Token does not have permissions");
        return Err(Status::Unauthorized);
    }

    match get_stores(pool, username) {
        Ok(stores) => {
            Ok(Json(stores))
        }
        Err(err) => {
            match err {
                APIErrors::DBError => return Err(Status::InternalServerError),
                APIErrors::UserNotFound => return Err(Status::NotFound),
                _ => return Err(Status::InternalServerError),
            }
        }
    }
}
