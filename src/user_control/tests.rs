#![allow(dead_code)] // To Silence Rust Analyzer
#![allow(unused_imports)]
use crate::user_control::structs::NewUser;

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

async fn get_token() -> String {
    let valid_params: (String, String) = crate::utils::get_valid_user_cred();
    let login_params = crate::signing::structs::LoginParams {
        p_username: valid_params.0,
        p_password: valid_params.1,
    };

    let token_string = crate::signing::signin(rocket::serde::json::Json(login_params), &get_pool()).await.unwrap();
    token_string.clone()
}

#[tokio::test]
async fn get_all_users() {
    let token = get_token().await;

    let users = crate::user_control::get_users(&crate::ApiKey(&token), &get_pool()).await.unwrap();
    assert!(users.len() > 0); // TODO: Check for specific users
}

#[tokio::test]
async fn get_specific_username() {
    match crate::user_control::get_user("test", &get_pool()).await {
        Ok(user) => {
            assert!(user.username == "test");
            assert!(user.fullname == "Test User");
            assert!(user.email == "test@test.com");
            assert!(user.login_duration == 1);
        },
        Err(e) => {
            println!("{:?}", e);
            assert!(false);
        },
    }    
}

#[tokio::test]
async fn create_test_user() {
    let new_user_data: NewUser = NewUser {
        p_username: "test_dummy".to_string(),
        p_password: "test2".to_string(),
        p_fullname: "TESTS_DUMMY_USER".to_string(),
        p_email: "test@test.com".to_string(),
        p_loginduration: 0,
    };
    
    
    match crate::user_control::create_user(new_user_data, &get_pool()).await {
        Ok(_) => {
            let user = crate::user_control::get_user("test_dummy", &get_pool()).await.unwrap();
            assert!(user.username == "test_dummy");
            // This is to clean up the test, later we test the delete function, could be considered redundant
            crate::user_control::delete_user("test_dummy", &get_pool()).await.unwrap(); 
        },
        Err(e) => {
            println!("{:?}", e);
            assert!(false);
        },
    }
}


#[tokio::test]
async fn edit_test_user() {

    // We cant test different passwords, due to the password hashing

   let edit_params = crate::user_control::structs::EditUserParams {
        p_fullname: Some("TESTS_DUMMY_USER_EDITED".to_string()),
        p_password: None,
        p_email: Some("testing@test.com".to_string()),
        p_loginduration: Some(12),
    };

    match crate::user_control::edit_user(rocket::serde::json::Json(edit_params), "test", &get_pool(), false).await {
        Ok(_) => {
            let user = crate::user_control::get_user("test", &get_pool()).await.unwrap();
            assert!(user.fullname == "TESTS_DUMMY_USER_EDITED");
            assert!(user.email == "testing@test.com");
            assert!(user.login_duration == 12);
        },
        Err(e) => {
            println!("{:?}", e);
            assert!(false);
        },
    }
    // Revert data for other tests, unwrap should be safe in practice since we just used it
    let edit_params = crate::user_control::structs::EditUserParams {
        p_fullname: Some("Test User".to_string()),
        p_password: None,
        p_email: Some("test@test.com".to_string()),
        p_loginduration: Some(1),
    };
    crate::user_control::edit_user(rocket::serde::json::Json(edit_params), "test", &get_pool(), false).await.unwrap();
}


#[tokio::test]
async fn delete_test_user() {
    let new_user_data: NewUser = NewUser {
        p_username: "test_dummy".to_string(),
        p_password: "test2".to_string(),
        p_fullname: "TESTS_DUMMY_USER".to_string(),
        p_email: "test_user".to_string(),
        p_loginduration: 0,
    };

    match crate::user_control::create_user(new_user_data, &get_pool()).await {
        Ok(_) => {
            let user = crate::user_control::get_user("test_dummy", &get_pool()).await.unwrap();
            assert!(user.username == "test_dummy");
        },
        Err(e) => {
            println!("{:?}", e);
            assert!(false);
        },
    }

    match crate::user_control::delete_user("test_dummy", &get_pool()).await {
        Ok(_) => {
            assert!(true);
        },
        Err(e) => {
            println!("{:?}", e);
            assert!(false);
        },
    }
    // Should return UserDoesntExist
    match crate::user_control::delete_user("test_dummy", &get_pool()).await {
        Ok(_) => {
            assert!(false);
        },
        Err(e) => {
            assert!(e == crate::user_control::APIErrors::UserNotFound);
        },
    }

}