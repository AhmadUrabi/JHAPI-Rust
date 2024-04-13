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