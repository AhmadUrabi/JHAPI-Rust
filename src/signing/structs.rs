use rocket::serde::Deserialize;
use rocket::serde::Serialize;


#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub USER_ID: Option<String>,
    pub USER_NAME: Option<String>,
    pub USER_EMAIL: Option<String>,
    pub LOGIN_DURATION: Option<String>,
}

impl User {
    pub fn new() -> User {
        User {
            USER_ID: None,
            USER_NAME: None,
            USER_EMAIL: None,
            LOGIN_DURATION: None,
        }
    }
}

#[derive(serde::Deserialize, Debug, Serialize, Clone)]
pub struct LoginParams {
    pub p_username: Option<String>,
    pub p_password: Option<String>,
}
