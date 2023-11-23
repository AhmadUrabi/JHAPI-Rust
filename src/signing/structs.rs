use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub USER_ID: Option<String>,
    pub USER_NAME: Option<String>,
    pub USER_EMAIL: Option<String>,
    pub LOGIN_DURATION: Option<String>,
}

#[derive(serde::Deserialize, Debug, Serialize, Clone)]
pub struct LoginParams {
    pub p_username: Option<String>,
    pub p_password: Option<String>,
}
