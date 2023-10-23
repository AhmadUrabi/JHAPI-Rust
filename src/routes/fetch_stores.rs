use oracle::pool::Pool;
use rocket::log::private::info;
use rocket::serde::json::Json;
use rocket::{get, State};

use crate::ApiKey;

use crate::fetch_stores::fetch_store_list;
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

    Json(fetch_store_list(pool, userId).await)
}

#[derive(serde::Deserialize, Debug)]
pub struct StoreListUpdateParams {
    pUsername: String,
    pStores: Option<Vec<i8>>,
    pAllStoresAccess: i8,
}

#[post("/StoreListEdit", data = "<params>")]
pub async fn UpdateStoreList(pool: &State<Pool>, _key: ApiKey<'_>, params: Json<StoreListUpdateParams>){
    info!("StoreListEdit Request: {:?}", params);

    let mut userId: String = "".to_string();

    match decode_token_data(_key.0) {
        Some(data) => { 
            info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap());
            userId = data.USER_ID.unwrap();
        },
        None => info!("Token Data: None"),
    }

    if is_admin_perm(&_key, pool) || is_perm_perm(&_key, pool) {
        info!("User has permissions");
    } else {
        info!("User does not have permissions");
        return;
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

}
