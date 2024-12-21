use std::io::Cursor;

use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::serde_json::{json, Value, to_string};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde", untagged)]
pub enum ResponseData {
    Json(Value),
    Text(String),
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub message: Option<String>,
    pub status: u16,
    pub data: Option<ResponseData>,
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for ApiResponse {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .header(ContentType::JSON)
            .sized_body(
                to_string(&self).unwrap().len(),
                Cursor::new(to_string(&self).unwrap()),
            )
            .status(Status::from_code(self.status).unwrap())
            .ok()
    }
}

impl ApiResponse {
    pub fn new(status: u16, message: Option<String>, data: Option<ResponseData>) -> Self {
        Self {
            message,
            status,
            data,
        }
    }
}

#[macro_export]
macro_rules! respond {
    ($code:expr, $message:expr, $data:expr) => {
        ApiResponse::new(
            $code,
            Some($message.to_string()),
            Some(crate::server::response::ResponseData::from_serializable(
                $data,
            )),
        )
    };
    ($code:expr, $message:expr) => {
        ApiResponse::new($code, Some($message.to_string()), None)
    };
    ($code:expr) => {
        ApiResponse::new($code, None, None)
    };
}

impl ResponseData {
    pub fn from_serializable<T>(value: T) -> Self
    where
        T: Serialize,
    {
        ResponseData::Json(json!(value))
    }
}
