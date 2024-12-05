use crate::server::JHApiServerState;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, Route, State};

use crate::controllers::auth::structs::LoginParams;
use crate::controllers::auth::{signin, validate_token};

use crate::utils::structs::APIError;

pub fn routes() -> Vec<Route> {
    routes![sign, authcheck, logout]
}

#[post("/login", data = "<params>")]
pub async fn sign(
    params: Json<LoginParams>,
    state: &State<JHApiServerState>,
    cookies: &rocket::http::CookieJar<'_>,
) -> Result<String, Status> {
    info!("Sign Request: {:?}", params.0.p_username);
    let pool = &state.pool;
    let sql_manager = &state.sql_manager;
    match signin(params, &pool, &sql_manager).await {
        Ok(token) => {
            info!("Valid User Data, Token Sent");
            let cookie = rocket::http::Cookie::build(("token", token.to_string()))
                .path("/")
                .same_site(rocket::http::SameSite::None)
                .secure(true);
            cookies.add(cookie);
            Ok(token.to_string())
        }
        Err(e) => {
            error!("Error authorizing, Token Not Sent");
            match e {
                APIError::InvalidData => Err(Status::Unauthorized),
                APIError::DBError => Err(Status::InternalServerError),
                APIError::UserNotFound => Err(Status::Unauthorized),
                APIError::InvalidCredentials => Err(Status::Unauthorized),
                _ => Err(Status::InternalServerError),
            }
        }
    }
}

#[post("/logout")]
pub async fn logout(cookies: &rocket::http::CookieJar<'_>) -> Result<&'static str, Status> {
    info!("Logout Request");
    cookies.remove(rocket::http::Cookie::from("token"));
    Ok("Logged Out")
}

#[get("/authcheck")]
pub async fn authcheck(cookies: &rocket::http::CookieJar<'_>) -> Result<&'static str, Status> {
    match cookies.get("token") {
        Some(value) => {
            info!("Token Found");
            if validate_token(value.value()) {
                info!("Valid Token Found");
                return Ok("Token Found");
            } else {
                error!("Invalid Token Found");
                Err(Status::Unauthorized)
            }
        }
        None => Err(Status::Unauthorized),
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
        let client = get_client().await;
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
