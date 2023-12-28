fn get_pool() -> oracle::pool::Pool {
    dotenv::dotenv().ok();
    let username = std::env::var("LOGIN_USERNAME").expect("LOGIN_USERNAME must be set.");
    let password = std::env::var("LOGIN_PASSWORD").expect("LOGIN_PASSWORD must be set.");
    let database = std::env::var("DB_CONNECTION").expect("DB_CONNECTION must be set.");

    let pool = oracle::pool::PoolBuilder::new(username, password, database)
        .min_connections(8) // Min == Max always
        .max_connections(8) 
        .build();

    pool.unwrap()
}

#[tokio::test]
async fn sign_in_valid_params() {
    let valid_params: (String, String) = crate::utils::get_valid_user_cred();
    let login_params = crate::signing::structs::LoginParams {
        p_username: valid_params.0,
        p_password: valid_params.1,
    };

    match crate::signing::signin(rocket::serde::json::Json(login_params), &get_pool()).await {
        Ok(token) => assert!(token.len() > 0),
        Err(_err) => {
            match _err {
                crate::utils::structs::APIErrors::InvalidCredentials => {
                    print!("Invalid Credentials");
                    assert!(false);
                },
                crate::utils::structs::APIErrors::InvalidData => {
                    print!("Invalid Data");
                    assert!(false);
                },
                crate::utils::structs::APIErrors::InternalServerError => {
                    println!("Internal Server Error");
                    assert!(false);
                },
                crate::utils::structs::APIErrors::DBError => {
                    println!("DB Error");
                    assert!(false);
                },
                _ => {
                    println!("Unknown Error");
                    assert!(false);
                },
            }
        }
    }
}

#[tokio::test]
async fn sign_in_invalid_params() {
    let invalid_params: (String, String) = crate::utils::get_invalid_user_cred();
    let login_params = crate::signing::structs::LoginParams {
        p_username: invalid_params.0,
        p_password: invalid_params.1,
    };

    match crate::signing::signin(rocket::serde::json::Json(login_params), &get_pool()).await {
        Ok(_token) => assert!(false),
        Err(_err) => {
            match _err {
                crate::utils::structs::APIErrors::InvalidCredentials => {
                    print!("Invalid Data Error");
                    assert!(true); // This is the expected result
                },
                _ => {
                    assert!(false);
                },
            }
        }
    }

}