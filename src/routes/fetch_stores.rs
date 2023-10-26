use oracle::pool::Pool;
use rocket::http::Status;
use rocket::log::private::info;
use rocket::serde::json::Json;
use rocket::{get, State};
use serde::Serialize;

use crate::ApiKey;
use std::net::IpAddr;

use crate::fetch_stores::get_stores;

use crate::signing::decode_token_data;

use crate::utils::permissions::*;

use crate::fetch_stores::structs::Store;

use crate::utils::logging::{getTimestamp, log_data};

#[get("/StoreList")]
pub async fn get_store_list(
    pool: &State<Pool>,
    _key: ApiKey<'_>,
    client_ip: Option<IpAddr>,
) -> Result<Json<Vec<Store>>, Status> {
    info!("StoreList Request");

    let mut userId: String = "".to_string();
    let usernameClone = userId.clone();

    match decode_token_data(_key.0) {
        Some(data) => {
            info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap());
            userId = data.USER_ID.unwrap();
        }
        None => info!("Token Data: None"),
    }

    if is_stores_perm(&_key, &pool) || is_admin_perm(&_key, &pool) {
        match get_stores(pool, "admin".to_string()) {
            Ok(stores) => {
                log_data(
                    pool,
                    userId,
                    client_ip.unwrap().to_string(),
                    "/StoreList".to_string(),
                    None,
                    getTimestamp(),
                    _key.0.to_string(),
                    "Success as Admin".to_string(),
                );
                return Ok(Json(stores));
            }
            Err(err) => {
                println!("Error: {}", err.to_string());
                return Err(Status::InternalServerError);
            }
        }
    }
    match get_stores(pool, userId) {
        Ok(stores) => {
            log_data(
                pool,
                usernameClone,
                client_ip.unwrap().to_string(),
                "/StoreList".to_string(),
                None,
                getTimestamp(),
                _key.0.to_string(),
                "Success".to_string(),
            );
            Ok(Json(stores))
        }
        Err(err) => {
            log_data(
                pool,
                usernameClone,
                client_ip.unwrap().to_string(),
                "/StoreList".to_string(),
                None,
                getTimestamp(),
                _key.0.to_string(),
                "Error Fetching".to_string(),
            );
            println!("Error: {}", err.to_string());
            Err(Status::InternalServerError)
        }
    }
}

#[derive(serde::Deserialize, Debug, Serialize, Clone)]
pub struct StoreListUpdateParams {
    pUsername: String,
    pStores: Option<Vec<i8>>,
    pAllStoresAccess: i8,
}

#[post("/StoreListEdit", data = "<params>")]
pub async fn UpdateStoreList(
    pool: &State<Pool>,
    _key: ApiKey<'_>,
    params: Json<StoreListUpdateParams>,
    client_ip: Option<IpAddr>,
) -> Result<String, Status> {
    info!("StoreListEdit Request: {:?}", params);

    let mut userId: String = "".to_string();

    let params_clone = params.clone();

    match decode_token_data(_key.0) {
        Some(data) => {
            info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap());
            userId = data.USER_ID.unwrap();
        }
        None => info!("Token Data: None"),
    }

    if is_admin_perm(&_key, pool) || is_stores_perm(&_key, pool) {
        info!("User has permissions");
    } else {
        info!("User does not have permissions");
        log_data(
            pool,
            userId,
            client_ip.unwrap().to_string(),
            "/StoreListEdit".to_string(),
            Some(serde_json::to_string(&params_clone.0).unwrap()),
            getTimestamp(),
            _key.0.to_string(),
            "Not Authorized".to_string(),
        );
        return Err(Status::Unauthorized);
    }

    let conn = pool.get().unwrap();
    // Delete previous values, if all access stores is set to one, just add a single row, else, add a row for each store
    if !params.pStores.is_none() || params.pAllStoresAccess == 0 {
        let mut stmt = conn
            .statement(
                "
            DELETE FROM ODBC_JHC.USER_STORES_JHC
            WHERE USERNAME = :username",
            )
            .build()
            .unwrap();

        stmt.execute(&[&params.pUsername]).unwrap();
        conn.commit().unwrap();
    }

    if params.pAllStoresAccess == 1 {
        let mut stmt = conn
            .statement(
                "
                INSERT INTO ODBC_JHC.USER_STORES_JHC (USERNAME, ALL_STORES_ACCESS)
                VALUES (:username, 1)",
            )
            .build()
            .unwrap();

        stmt.execute(&[&params.pUsername]).unwrap();
        conn.commit().unwrap();
    } else {
        for store in params.pStores.as_ref().unwrap().iter() {
            let mut stmt = conn
                .statement(
                    "
                    INSERT INTO ODBC_JHC.USER_STORES_JHC (USERNAME, STORE_ID)
                    VALUES (:username, :store_id)",
                )
                .build()
                .unwrap();

            stmt.execute(&[&params.pUsername, store]).unwrap();
            conn.commit().unwrap();
        }
    }

    return Ok("Success".to_string());
}

#[get("/StoreList/<username>")]
pub async fn get_store_list_for_user(
    pool: &State<Pool>,
    _key: ApiKey<'_>,
    username: String,
    client_ip: Option<IpAddr>,
) -> Result<Json<Vec<Store>>, Status> {
    info!("StoreList Request");

    let mut userId: String = "".to_string();
    let myUsername = username.to_lowercase();

    match decode_token_data(_key.0) {
        Some(data) => {
            info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap());
            userId = data.USER_ID.unwrap();
        }
        None => info!("Token Data: None"),
    }

    if !is_stores_perm(&_key, &pool) || !is_admin_perm(&_key, &pool) {
        log_data(
            pool,
            userId,
            client_ip.unwrap().to_string(),
            ("/StoreList/".to_owned() + &myUsername).to_string(),
            None,
            getTimestamp(),
            _key.0.to_string(),
            "Not Authorized".to_string(),
        );
        info!("Token does not have permissions");
        return Err(Status::Unauthorized);
    }

    match get_stores(pool, username) {
        Ok(stores) => {
            log_data(
                pool,
                userId,
                client_ip.unwrap().to_string(),
                ("/StoreList/".to_owned() + &myUsername).to_string(),
                None,
                getTimestamp(),
                _key.0.to_string(),
                "Success".to_string(),
            );
            Ok(Json(stores))
        }
        Err(err) => {
            log_data(
                pool,
                userId,
                client_ip.unwrap().to_string(),
                ("/StoreList/".to_owned() + &myUsername).to_string(),
                None,
                getTimestamp(),
                _key.0.to_string(),
                "Error Fetching".to_string(),
            );
            println!("Error: {}", err.to_string());
            Err(Status::InternalServerError)
        }
    }
}
