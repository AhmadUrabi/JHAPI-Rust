pub mod authentication;
pub mod files;
pub mod logs;
pub mod permissions;
pub mod products;
pub mod stores;
pub mod users;
pub mod versions;

#[get("/health_check")]
pub async fn health_check() -> &'static str {
    "Server is running"
}

#[cfg(test)]
mod test {
    use crate::utils::testing::*;
    use dotenv::dotenv;
    
    #[tokio::test]
    pub async fn test_health_check() {
        dotenv().ok();
        let client = get_client(routes![super::health_check]).await;
        let response = client.get("/api/health_check").dispatch().await;
        assert_eq!(response.status(), rocket::http::Status::Ok);
        assert_eq!(response.into_string().await.unwrap(), "Server is running");
    }
}
