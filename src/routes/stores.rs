use crate::server::JHApiServerState;
use rocket::http::Status;
use rocket::log::private::info;
use rocket::serde::json::Json;
use rocket::{get, Route, State};

use crate::functions::stores::structs::*;

use crate::server::request_guard::api_key::ApiKey;
use crate::utils::structs::APIErrors;

use crate::functions::stores::get_stores;

use crate::functions::auth::decode_token_data;

use crate::utils::{check_user_exists, permissions::*};

use crate::functions::stores::structs::Store;

pub fn routes() -> Vec<Route> {
    routes![get_store_list, update_store_list, get_store_list_for_user]
}

#[get("/stores")]
pub async fn get_store_list(
    state: &State<JHApiServerState>,
    _key: ApiKey<'_>,
) -> Result<Json<Vec<Store>>, Status> {
    let pool = &state.pool;
    let sql_manager = &state.sql_manager;
    info!("Stores Get Request");
    let mut user_id: String = "".to_string();
    match decode_token_data(_key.0) {
        Some(data) => {
            info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap());
            user_id = data.USER_ID.unwrap();
        }
        None => info!("Token Data: None"),
    }

    if has_stores_perm(&_key, &pool, &sql_manager).await
        || has_admin_perm(&_key, &pool, &sql_manager).await
    {
        match get_stores(&pool, &sql_manager, "admin".to_string()).await {
            Ok(stores) => {
                return Ok(Json(stores));
            }
            Err(err) => match err {
                APIErrors::DBError => return Err(Status::InternalServerError),
                APIErrors::UserNotFound => return Err(Status::NotFound),
                _ => return Err(Status::InternalServerError),
            },
        }
    }
    match get_stores(&pool, &sql_manager, user_id).await {
        Ok(stores) => Ok(Json(stores)),
        Err(err) => match err {
            APIErrors::DBError => return Err(Status::InternalServerError),
            APIErrors::UserNotFound => return Err(Status::NotFound),
            _ => return Err(Status::InternalServerError),
        },
    }
}

// TODO: extract to function
#[post("/stores", data = "<params>")]
pub async fn update_store_list(
    state: &State<JHApiServerState>,
    _key: ApiKey<'_>,
    params: Json<StoreListUpdateParams>,
) -> Result<String, Status> {
    let pool = &state.pool;
    let sql_manager = &state.sql_manager;
    info!("stores Request: {:?}", params);
    if has_admin_perm(&_key, pool, &sql_manager).await
        || has_stores_perm(&_key, pool, &sql_manager).await
    {
        info!("User has permissions");
    } else {
        info!("User does not have permissions");
        return Err(Status::Unauthorized);
    }

    // TODO: Whole function should be separated from route function
    match check_user_exists(params.0.p_username.clone(), &pool, &sql_manager).await {
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
    state: &State<JHApiServerState>,
    _key: ApiKey<'_>,
    username: String,
) -> Result<Json<Vec<Store>>, Status> {
    let pool = &state.pool;
    let sql_manager = &state.sql_manager;
    info!("User stores Request");

    if !has_stores_perm(&_key, &pool, &sql_manager).await
        && !has_admin_perm(&_key, &pool, &sql_manager).await
    {
        info!("Token does not have permissions");
        return Err(Status::Unauthorized);
    }

    match get_stores(&pool, &sql_manager, username).await {
        Ok(stores) => Ok(Json(stores)),
        Err(err) => match err {
            APIErrors::DBError => return Err(Status::InternalServerError),
            APIErrors::UserNotFound => return Err(Status::NotFound),
            _ => return Err(Status::InternalServerError),
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::testing::*;
    use dotenv::dotenv;

    #[tokio::test]
    pub async fn test_get_store_list() {
        dotenv().ok();
        let token = get_valid_user_token().await;
        let client = get_client(routes![get_store_list]).await;
        let response = client
            .get("/api/stores")
            .header(rocket::http::Header::new(
                "Authorization",
                format!("{}", token.unwrap()),
            ))
            .dispatch()
            .await;
        println!("{:?}", response.body());
        assert_eq!(response.status(), rocket::http::Status::Ok);
    }

    #[tokio::test]
    pub async fn test_get_user_stores() {
        dotenv().ok();
        let token = get_valid_user_token().await;
        let client = get_client(routes![get_store_list_for_user]).await;
        let response = client
            .get(format!(
                "/api/stores/{}",
                std::env::var("VALID_USER_TEST").unwrap()
            ))
            .header(rocket::http::Header::new(
                "Authorization",
                format!("{}", token.unwrap()),
            ))
            .dispatch()
            .await;
        assert_eq!(response.status(), rocket::http::Status::Ok);
        let res = response
            .into_json::<Vec<crate::functions::stores::structs::Store>>()
            .await
            .unwrap();
        assert_eq!(res.len() > 0, true);

        // Check if the first store is the correct one
        assert_eq!(res[0].STORE_ID, Some("01".to_string()));
    }

    #[tokio::test]
    pub async fn test_post_stores() {
        dotenv().ok();
        let token = get_valid_user_token().await;
        let client = get_client(routes![update_store_list, get_store_list_for_user]).await;

        // Create an object of type EditStoresParams
        let params = crate::functions::stores::structs::StoreListUpdateParams {
            p_username: std::env::var("TESTING_USER").unwrap(),
            p_stores: Some(vec![1, 2, 5]),
            p_allstoresaccess: 0,
        };

        // Send the object as JSON in the request body
        let response = client
            .post("/api/stores")
            .header(rocket::http::Header::new(
                "Authorization",
                format!("{}", &token.clone().unwrap()),
            ))
            .header(rocket::http::Header::new(
                "Content-Type",
                "application/json",
            ))
            .body(serde_json::to_string(&params).unwrap())
            .dispatch()
            .await;

        assert_eq!(response.status(), rocket::http::Status::Ok);

        // Call the get stores route to check if the store was created correctly
        let response = client
            .get(format!(
                "/api/stores/{}",
                std::env::var("TESTING_USER").unwrap()
            ))
            .header(rocket::http::Header::new(
                "Authorization",
                format!("{}", &token.clone().unwrap()),
            ))
            .dispatch()
            .await;

        assert_eq!(response.status(), rocket::http::Status::Ok);
        let stores = response
            .into_json::<Vec<crate::functions::stores::structs::Store>>()
            .await
            .unwrap();

        // Check if the added stores are in the list
        assert_eq!(stores.len() > 0, true);
        assert_eq!(stores[0].STORE_ID, Some("01".to_string()));
        assert_eq!(stores[1].STORE_ID, Some("02".to_string()));
        assert_eq!(stores[2].STORE_ID, Some("05".to_string()));

        // Check if only 3 stores are in the list
        assert_eq!(stores.len(), 3);

        // Clean up the test by removing the stores
        let params = crate::functions::stores::structs::StoreListUpdateParams {
            p_username: std::env::var("TESTING_USER").unwrap(),
            p_stores: Some(vec![]),
            p_allstoresaccess: 0,
        };

        let response = client
            .post("/api/stores")
            .header(rocket::http::Header::new(
                "Authorization",
                format!("{}", &token.clone().unwrap()),
            ))
            .header(rocket::http::Header::new(
                "Content-Type",
                "application/json",
            ))
            .body(serde_json::to_string(&params).unwrap())
            .dispatch()
            .await;

        assert_eq!(response.status(), rocket::http::Status::Ok);

        // Call the get stores route to check if the store was removed correctly
        let response = client
            .get(format!(
                "/api/stores/{}",
                std::env::var("TESTING_USER").unwrap()
            ))
            .header(rocket::http::Header::new(
                "Authorization",
                format!("{}", &token.unwrap()),
            ))
            .dispatch()
            .await;

        assert_eq!(response.status(), rocket::http::Status::Ok);
        let stores = response
            .into_json::<Vec<crate::functions::stores::structs::Store>>()
            .await
            .unwrap();
        assert_eq!(stores.len(), 0);
    }
}
