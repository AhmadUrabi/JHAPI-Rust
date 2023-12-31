use rocket::serde::json::Json;

use std::time::{SystemTime, UNIX_EPOCH};

use oracle::pool::Pool;

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::signing::structs::LoginParams;
use crate::signing::structs::User;
use crate::utils::structs::APIErrors;

use bcrypt::verify;

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



pub async fn signin(params: Json<LoginParams>, pool: &Pool) -> Result<String, APIErrors> {
    // Check for empty username and password
    info!("Login Attempt: {:?}", params.0.p_username);

    if params.p_username == "" {
        error!("Empty username");
        return Err(APIErrors::InvalidData);
    }

    let user = fetch_user_data(params.p_username.to_lowercase(), params.p_password.to_string(), pool);
    if user.is_err() {
        error!("Error fetching user data");
        return Err(user.err().unwrap());
    }
    let user = user.unwrap();

    let token = generate_token(&user);
    if token.is_err() {
        error!("Error generating token");
        return Err(token.err().unwrap());
    }
    let token = token.unwrap();

    info!("Token generated successfully");
    Ok(token)
}

fn fetch_user_data(username: String, password: String, pool: &Pool) -> Result<User, APIErrors> {
    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to DB");
        return Err(APIErrors::DBError);
    }
    let conn = conn.unwrap();

    let stmt = conn.statement("SELECT USERNAME, PASSWORD, FULLNAME, EMAIL, LOGINDURATION FROM ODBC_JHC.AUTHENTICATION_JHC WHERE USERNAME = :1").fetch_array_size(1).build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(APIErrors::DBError);
    }
    let mut stmt = stmt.unwrap();
    let rows = stmt.query_row(&[&username]);
    if rows.is_err() {
        error!("Error executing query");
        return Err(APIErrors::UserNotFound);
    }
    let row = rows.unwrap();
    

    let mut user = User::new();
    if !verify(&password, &row.get::<&str, String>("PASSWORD").unwrap()).unwrap() {
        error!("Invalid password");
        return Err(APIErrors::InvalidCredentials);
    }
    user.USER_ID = Some(row.get::<&str, String>("USERNAME").unwrap());
    user.USER_NAME = Some(row.get::<&str, String>("FULLNAME").unwrap());
    user.USER_EMAIL = Some(row.get::<&str, String>("EMAIL").unwrap());
    user.LOGIN_DURATION = Some(row.get::<&str, String>("LOGINDURATION").unwrap());
    Ok(user)
}

fn generate_token(user: &User) -> Result<String,APIErrors> {
    let secret: String = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set.");
    if user.USER_ID.is_none()
        || user.USER_NAME.is_none()
        || user.USER_EMAIL.is_none()
        || user.LOGIN_DURATION.is_none()
    {
        error!("User data incomplete");
        return Err(APIErrors::InvalidData);
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
        &EncodingKey::from_secret(secret.to_string().as_ref()),
    );

    match token {
        Ok(token) => Ok(token),
        Err(err) => {
            error!("Error generating token: {}", err);
            Err(APIErrors::InternalServerError)
        }
    }
}

pub fn validate_token(token: &str) -> bool {
    let secret: String = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set.");
    let decoded_token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    );

    match decoded_token {
        Ok(token) => {
            return token.claims.exp
                > SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap() // Safe Unwap, earlier is const, always less than current time
                    .as_secs() as usize
        }
        Err(err) => {
            println!("Error decoding token: {}", err);
            return false;
        }
    }
}

pub fn decode_token_data(token: &str) -> Option<User> {
    let secret: String = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set.");
    let decoded_token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    );
    let username;
    let name;
    let email;
    let duration;
    match decoded_token {
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
