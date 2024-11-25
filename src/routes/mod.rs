use rocket::Route;

pub mod auth;
pub mod files;
pub mod ldap;
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

// Hack: To handle Options request on firefox
#[options("/<_path..>")]
fn cors_preflight_handler(_path: std::path::PathBuf) -> rocket::http::Status {
    rocket::http::Status::Ok
}

pub fn get_all_routes() -> Vec<Route> {
    let routes_vec = vec![
        auth::routes(),
        files::routes(),
        ldap::routes(),
        logs::routes(),
        permissions::routes(),
        products::routes(),
        stores::routes(),
        users::routes(),
        versions::routes(),
        routes![health_check, cors_preflight_handler],
    ];
    routes_vec.into_iter().flatten().collect()
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
