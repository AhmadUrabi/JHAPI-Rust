use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub USER_ID: Option<String>,
    pub USER_NAME: Option<String>,
    pub USER_EMAIL: Option<String>,
    pub LOGIN_DURATION: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct LoginParams {
    pub pUserName: Option<String>,
    pub pPassword: Option<String>,
}
