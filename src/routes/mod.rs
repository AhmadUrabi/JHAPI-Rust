pub mod stores;
pub mod files;
pub mod logs;
pub mod permissions;
pub mod products;
pub mod authentication;
pub mod users;
pub mod versions;

#[get("/health_check")]
pub async fn health_check() -> &'static str {
    "Server is running"
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::server::JHApiServer;
    use dotenv::dotenv;
    async fn get_server_with_route(routes: Vec<rocket::Route>) -> rocket::Rocket<rocket::Build> {
        let server_wrapper = JHApiServer::init(routes).await;
        server_wrapper.server
    }

    async fn get_client(routes: Vec<rocket::Route>) -> rocket::local::asynchronous::Client {
        let server = get_server_with_route(routes).await;
        rocket::local::asynchronous::Client::tracked(server).await.expect("Failed to create client")
    }

    async fn get_valid_user_token() -> String {
        let client = get_client(routes![authentication::sign]).await;
        let auth = (std::env::var("VALID_USER_TEST").unwrap(), std::env::var("VALID_PASS_TEST").unwrap());
        let response = client
            .post("/api/login")
            .header(rocket::http::Header::new("Content-Type", "application/json"))
            .body(format!("{{\"p_username\":\"{}\",\"p_password\":\"{}\"}}", auth.0, auth.1))
            .dispatch()
            .await;
        let response_body = response.into_string().await.unwrap();
        response_body
    }

    #[tokio::test]
    pub async fn test_health_check() {
        dotenv().ok();
        let client = get_client(routes![health_check]).await;
        let response = client.get("/api/health_check").dispatch().await;
        assert_eq!(response.status(), rocket::http::Status::Ok);
        assert_eq!(response.into_string().await.unwrap(), "Server is running");
    }

    #[tokio::test]
    pub async fn test_get_store_list() {
        dotenv().ok();
        let token = get_valid_user_token().await;
        let client = get_client(routes![stores::get_store_list]).await;
        let response = client.get("/api/stores")
            .header(rocket::http::Header::new("Authorization", format!("{}", token)))
            .dispatch().await;
        println!("{:?}", response.body());
        assert_eq!(response.status(), rocket::http::Status::Ok);
    }

    #[tokio::test]
    pub async fn test_get_user_stores() {
        dotenv().ok();
        let token = get_valid_user_token().await;
        let client = get_client(routes![stores::get_store_list_for_user]).await;
        let response = client.get(format!("/api/stores/{}",std::env::var("VALID_USER_TEST").unwrap()))
            .header(rocket::http::Header::new("Authorization", format!("{}", token)))
            .dispatch().await;
        assert_eq!(response.status(), rocket::http::Status::Ok);
        let res = response.into_json::<Vec<crate::functions::stores::structs::Store>>().await.unwrap();
        assert_eq!(res.len()>0, true);

        // Check if the first store is the correct one
        assert_eq!(res[0].STORE_ID, Some("01".to_string()));
    }

}