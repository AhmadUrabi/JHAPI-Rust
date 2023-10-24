use oracle::pool::Pool;
use rocket::log::private::info;
use rocket::serde::json::Json;
use rocket::{get, State};

use crate::ApiKey;

use crate::fetch_stores::get_stores;

use crate::signing::decode_token_data;

use crate::utils::permissions::*;

use crate::fetch_stores::structs::Store;

#[get("/StoreList")]
pub async fn get_store_list(pool: &State<Pool>, _key: ApiKey<'_>) -> Json<Vec<Store>> {
    info!("StoreList Request");

    let mut userId: String = "".to_string();

    match decode_token_data(_key.0) {
        Some(data) => { 
            info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap());
            userId = data.USER_ID.unwrap();
        },
        None => info!("Token Data: None"),
    }
    if is_stores_perm(&_key, &pool) || is_admin_perm(&_key, &pool){
        match get_stores(pool, "admin".to_string()) {
            Ok(stores) => return Json(stores),
            Err(err) => {
                println!("Error: {}", err.to_string());
                return Json(Vec::new());
            }
        }
    }
    match get_stores(pool, userId) {
        Ok(stores) => Json(stores),
        Err(err) => {
            println!("Error: {}", err.to_string());
            Json(Vec::new())
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct StoreListUpdateParams {
    pUsername: String,
    pStores: Option<Vec<i8>>,
    pAllStoresAccess: i8,
}

#[post("/StoreListEdit", data = "<params>")]
pub async fn UpdateStoreList(pool: &State<Pool>, _key: ApiKey<'_>, params: Json<StoreListUpdateParams>) -> String{
    info!("StoreListEdit Request: {:?}", params);

    let mut userId: String = "".to_string();

    match decode_token_data(_key.0) {
        Some(data) => { 
            info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap());
            userId = data.USER_ID.unwrap();
        },
        None => info!("Token Data: None"),
    }

    if is_admin_perm(&_key, pool) || is_stores_perm(&_key, pool) {
        info!("User has permissions");
    } else {
        info!("User does not have permissions");
        return "User does not have permissions".to_string();
    }

    let conn = pool.get().unwrap();
    // Delete previous values, if all access stores is set to one, just add a single row, else, add a row for each store
    if !params.pStores.is_none() && params.pAllStoresAccess == 0 {
    let mut stmt = conn
        .statement("
            DELETE FROM ODBC_JHC.USER_STORES_JHC
            WHERE USERNAME = :username")
        .build()
        .unwrap();

        stmt.execute(&[&params.pUsername]).unwrap();
        conn.commit().unwrap();
    }

    


    if params.pAllStoresAccess == 1 {
        let mut stmt = conn
            .statement("
                INSERT INTO ODBC_JHC.USER_STORES_JHC (USERNAME, ALL_STORES_ACCESS)
                VALUES (:username, 1)")
            .build()
            .unwrap();

        stmt.execute(&[&params.pUsername]).unwrap();
        conn.commit().unwrap();
    } else {
        for store in params.pStores.as_ref().unwrap().iter() {
            let mut stmt = conn
                .statement("
                    INSERT INTO ODBC_JHC.USER_STORES_JHC (USERNAME, STORE_ID)
                    VALUES (:username, :store_id)")
                .build()
                .unwrap();

            stmt.execute(&[&params.pUsername, store]).unwrap();
            conn.commit().unwrap();
        }
    }

    return "Success".to_string();

}


#[get("/StoreList/<username>")]
pub async fn get_store_list_for_user(pool: &State<Pool>, _key: ApiKey<'_>, username: String) -> Json<Vec<Store>> {
    info!("StoreList Request");

    let mut userId: String = "".to_string();

    match decode_token_data(_key.0) {
        Some(data) => { 
            info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap());
            userId = data.USER_ID.unwrap();
        },
        None => info!("Token Data: None"),
    }

    if !is_stores_perm(&_key, &pool) || !is_admin_perm(&_key, &pool) { 
            info!("Token does not have permissions");
            return Json(vec![]);
    }

    match get_stores(pool, username) {
        Ok(stores) => Json(stores),
        Err(err) => {
            println!("Error: {}", err.to_string());
            Json(Vec::new())
        }
    }
}