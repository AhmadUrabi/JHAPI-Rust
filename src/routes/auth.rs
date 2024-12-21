use crate::respond;
use crate::server::JHApiServerState;
use crate::utils::structs::APIError;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, Route, State};

use crate::controllers::auth::*;

use crate::server::response::ApiResponse;

pub fn routes() -> Vec<Route> {
    routes![sign, authcheck, logout]
}

#[post("/login", data = "<params>")]
pub async fn sign(
    params: Json<LoginParams>,
    state: &State<JHApiServerState>,
    cookies: &rocket::http::CookieJar<'_>,
) -> ApiResponse {
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
            respond!(200, "Authenticated", token)
        }
        Err(e) => {
            error!("Error authorizing, Token Not Sent");
            e.into()
        }
    }
}

#[post("/logout")]
pub async fn logout(cookies: &rocket::http::CookieJar<'_>) -> ApiResponse {
    // TODO: Extend functionality
    info!("Logout Request");
    cookies.remove(rocket::http::Cookie::from("token"));
    respond!(200, "Logged Out")
}

#[get("/authcheck")]
pub async fn authcheck(cookies: &rocket::http::CookieJar<'_>) -> ApiResponse {
    match cookies.get("token") {
        Some(value) => {
            info!("Token Found");
            if validate_token(value.value()) {
                info!("Valid Token Found");
                respond!(200, "Token Found")
            } else {
                error!("Invalid Token Found");
                APIError::InvalidToken.into()
            }
        }
        None => APIError::InvalidCredentials.into(),
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
