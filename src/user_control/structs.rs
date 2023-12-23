#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub username: String,
    pub fullname: String,
    pub email: String,
    pub login_duration: i32,
}

impl User {
    pub fn new() -> User {
        User {
            username: "".to_string(),
            fullname: "".to_string(),
            email: "".to_string(),
            login_duration: 0,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct NewUser {
    pub p_username: String,
    pub p_password: String,
    pub p_fullname: String,
    pub p_email: String,
    pub p_loginduration: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EditUserParams {
    pub p_password: Option<String>,
    pub p_fullname: Option<String>,
    pub p_email: Option<String>,
    pub p_loginduration: Option<i32>,
}