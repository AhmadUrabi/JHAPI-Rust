#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub username: String,
    pub fullname: Option<String>,
    pub email: Option<String>,
    pub login_duration: Option<i32>,
}

impl User {
    pub fn new() -> User {
        User {
            username: "".to_string(),
            fullname: None,
            email: None,
            login_duration: None,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.username == ""
    }
}

impl Clone for User {
    fn clone(&self) -> User {
        User {
            username: self.username.clone(),
            fullname: self.fullname.clone(),
            email: self.email.clone(),
            login_duration: self.login_duration.clone(),
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

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct EditUserParams {
    pub p_password: Option<String>,
    pub p_fullname: Option<String>,
    pub p_email: Option<String>,
    pub p_loginduration: Option<i32>,
}
