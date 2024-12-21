use crate::server::JHApiServer;
use crate::server::response::{ApiResponse, ResponseData};

#[allow(dead_code)]
pub async fn get_server_with_route() -> rocket::Rocket<rocket::Build> {
    let server_wrapper = JHApiServer::init().await;
    server_wrapper.server
}

#[allow(dead_code)]
pub async fn get_client() -> rocket::local::asynchronous::Client {
    let server = get_server_with_route().await;
    rocket::local::asynchronous::Client::tracked(server)
        .await
        .expect("Failed to create client")
}

#[allow(dead_code)]
pub async fn get_valid_user_token() -> Option<String> {
    let client = get_client().await;
    let auth = (
        std::env::var("VALID_USER_TEST").unwrap(),
        std::env::var("VALID_PASS_TEST").unwrap(),
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
    assert_eq!(response.status(), rocket::http::Status::Ok);
    let response_body = response.into_json::<ApiResponse>().await.unwrap();
    match response_body.data {
        Some(data) => match data {
            ResponseData::Text(text) => Some(text),
            ResponseData::Json(value) => Some(value.to_string()),
        },
        None => None,
    }
}
