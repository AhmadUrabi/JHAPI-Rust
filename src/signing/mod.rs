use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use rocket::time::{Date, Duration};
use sha2::Sha256;
use std::{collections::BTreeMap, time::{SystemTime, UNIX_EPOCH}};
use chrono::prelude::*;

use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

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
        let iat = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let exp = iat + (duration * 3600);

        // Convert iat to usize
        let iat = iat as usize;
        let exp = exp as usize;

        Self { id, name, email, iat, exp }
    }
}

use crate::apistructs::User;

pub async fn signin(){
    let user = User {
        USER_ID: Some("1".to_string()),
        USER_NAME: Some("test".to_string()),
        USER_EMAIL: Some("ahmad.u@live.com".to_string()),
        LOGIN_DURATION: Some("1".to_string())
    };
    let token = generate_token(&user);
    println!("Token: {}", token);
}

fn generate_token(user: &User) -> String {
    let claims = Claims::new(
        user.USER_ID.clone().unwrap(),
        user.USER_NAME.clone().unwrap(),
        user.USER_EMAIL.clone().unwrap(),
        user.LOGIN_DURATION.clone().unwrap().parse::<u64>().unwrap(),
    );


    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("S".as_ref()),
    );
    return token.unwrap();
}
