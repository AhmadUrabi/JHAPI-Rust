use crate::server::JHApiServerState;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, State};

use crate::functions::authentication::signin;
use crate::functions::authentication::structs::LoginParams;

use crate::utils::structs::APIErrors;

#[post("/login", data = "<params>")]
pub async fn sign(
    params: Json<LoginParams>,
    state: &State<JHApiServerState>
) -> Result<String, Status> {
    info!("Sign Request: {:?}", params.0.p_username);
    let pool = &state.pool;
    let sql_manager = &state.sql_manager;
    match signin(params, &pool, &sql_manager).await {
        Ok(token) => {
            info!("Valid User Data, Token Sent");
            Ok(token.to_string())
        }
        Err(e) => {
            error!("Error authorizing, Token Not Sent");
            match e {
                APIErrors::InvalidData => Err(Status::Unauthorized),
                APIErrors::DBError => Err(Status::InternalServerError),
                APIErrors::UserNotFound => Err(Status::Unauthorized),
                APIErrors::InvalidCredentials => Err(Status::Unauthorized),
                _ => Err(Status::InternalServerError),
            }
        }
    }
}


#[cfg(test)]
mod test {
    use crate::utils::testing::*;
    use dotenv::dotenv;
    
    #[tokio::test]
    pub async fn test_login_valid() {
        dotenv().ok();
        assert!(get_valid_user_token().await.is_some());
    }

    #[tokio::test]
    pub async fn test_login_invalid() {
        dotenv().ok();
        let client = get_client(routes![super::sign]).await;
        let auth = (
            std::env::var("INVALID_USER_TEST").unwrap(),
            std::env::var("INVALID_PASS_TEST").unwrap(),
        );
        let response = client
            .post("/api/login")
            .header(rocket::http::Header::new(
                "Content-Type",
                "application/json",
            ))
            .body(format!(
                "{{\"p_username\":\"{}\",\"p_password\":\"{}\"}}",
                auth.0, auth.1
            ))
            .dispatch()
            .await;
        assert_eq!(response.status(), rocket::http::Status::Unauthorized);
    }


}