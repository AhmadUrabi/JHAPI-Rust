use rocket::{fairing::{Fairing, Info, Kind}, Request, Response};

use crate::{functions::authentication::decode_token_data, utils::logging::{get_timestamp, log_data}};

pub struct Logger;

#[rocket::async_trait]
impl Fairing for Logger {
    fn info(&self) -> Info {
        Info {
            name: "Request Logging",
            kind: Kind::Response,
        }
    }
    
    async fn on_response<'r>(&self, req: &'r Request<'_>, response: &mut Response<'r>) {
        let pool = req.guard::<&rocket::State<oracle::pool::Pool>>().await.unwrap();
        let token = req.headers().get_one("Authorization");
        let username;
        let client_ip = req.client_ip().unwrap();
        let method = req.method().as_str().to_string();
        let route: String = req.uri().path().to_string();
        let current_time = get_timestamp();
        let result = response.status().code.to_string() + " " + response.status().reason().unwrap_or("No Reason");
        match token {
            Some(token) => {
                let token_data = decode_token_data(token);
                match token_data {
                    Some(user_data) => {
                        username = user_data.USER_ID .unwrap_or("Unknown".to_string());
                    }
                    None => {
                        username = "Unknown".to_string();
                    }
                }
            },
            None => {
                username = "No User".to_string();
            }
        }
        log_data(
            &pool,
            username,
            client_ip.to_string(),
            route,
            None,
            current_time,
            token.unwrap_or("No Token").to_string(),
            result,
            method,
        );
        
    }
}