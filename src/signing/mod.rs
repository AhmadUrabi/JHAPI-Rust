use rocket::serde::json::Json;
use std::time::{SystemTime, UNIX_EPOCH};

use oracle::pool::Pool;

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::signing::structs::LoginParams;
use crate::signing::structs::User;

pub mod structs;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: String,
    name: String,
    email: String,
    iat: usize,
    exp: usize,
}

impl Claims {
    pub fn new(id: String, name: String, email: String, duration: u64) -> Self {
        // normalize the timestamps by stripping of microseconds
        let iat = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let exp = iat + (duration * 3600);

        // Convert iat to usize
        let iat = iat as usize;
        let exp = exp as usize;

        Self {
            id,
            name,
            email,
            iat,
            exp,
        }
    }
}

const SECRET: &str = "SecretKey";

pub async fn signin(params: Json<LoginParams>, pool: &Pool) -> Option<Json<String>> {
    // Check for empty username and password
    info!("Login Attempt: {:?}", params.0.pUserName);

    if params.pUserName.is_none() || params.pPassword.is_none() {
        error!("Empty username or password");
        return None;
    }

    let mut mypUsername = "%";
    let mut mypPassword = "%";

    if let Some(pUserName) = &params.pUserName {
        mypUsername = pUserName;
    }

    if let Some(pPassword) = &params.pPassword {
        mypPassword = pPassword;
    }

    let user = fetch_user_data(mypUsername.to_string(), mypPassword.to_string(), pool);

    // If user doesn't exist, return None
    if user.is_none() {
        error!("User not found");
        println!("User not found");
        return None;
    }

    let token = generate_token(&user.unwrap());

    if token == "" {
        error!("Token generation failed");
        println!("Token generation failed");
        return None;
    }

    info!("Token generated successfully");
    return Some(Json(token));
}

fn fetch_user_data(username: String, password: String, pool: &Pool) -> Option<User> {
    let conn = pool.get().unwrap();
    let mut stmt = conn
        .statement("SELECT USERNAME, FULLNAME, EMAIL, LOGINDURATION FROM ODBC_JHC.AUTHENTICATION_JHC WHERE USERNAME = :1 AND PASSWORD = :2").build()
        .unwrap();
    let rows = stmt.query(&[&username, &password]).unwrap();

    let mut user = User {
        USER_ID: None,
        USER_NAME: None,
        USER_EMAIL: None,
        LOGIN_DURATION: None,
    };
    for row_result in rows {
        let row = row_result.unwrap();
        user.USER_ID = Some(row.get::<&str, String>("USERNAME").unwrap());
        user.USER_NAME = Some(row.get::<&str, String>("FULLNAME").unwrap());
        user.USER_EMAIL = Some(row.get::<&str, String>("EMAIL").unwrap());
        user.LOGIN_DURATION = Some(row.get::<&str, String>("LOGINDURATION").unwrap());
    }
    return Some(user);
}

fn generate_token(user: &User) -> String {
    if user.USER_ID.is_none()
        || user.USER_NAME.is_none()
        || user.USER_EMAIL.is_none()
        || user.LOGIN_DURATION.is_none()
    {
        return String::from("");
    }

    let claims = Claims::new(
        user.USER_ID.clone().unwrap(),
        user.USER_NAME.clone().unwrap(),
        user.USER_EMAIL.clone().unwrap(),
        user.LOGIN_DURATION.clone().unwrap().parse::<u64>().unwrap(),
    );

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET.to_string().as_ref()),
    );
    return token.unwrap();
}

pub fn validate_token(token: &str) -> bool {
    let DecodedToken = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &Validation::default(),
    );

    match DecodedToken {
        Ok(token) => {
            return token.claims.exp
                > SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as usize
        }
        Err(err) => {
            println!("Error decoding token: {}", err);
            return false;
        }
    }
}

pub fn decode_token_data(token: &str) -> Option<User> {
    let DecodedToken = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &Validation::default(),
    );
    let username;
    let name;
    let email;
    let duration;
    match DecodedToken {
        Ok(token) => {
            username = token.claims.id;
            name = token.claims.name;
            email = token.claims.email;
            duration = token.claims.exp - token.claims.iat;
        }
        Err(err) => {
            println!("Error decoding token: {}", err);
            return None;
        }
    }

    let user = User {
        USER_ID: Some(username),
        USER_NAME: Some(name),
        USER_EMAIL: Some(email),
        LOGIN_DURATION: Some(duration.to_string()),
    };

    return Some(user);
}
