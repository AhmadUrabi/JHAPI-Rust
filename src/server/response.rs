pub struct ResponseStatus {
    pub code: u16,
    pub message: Option<String>,
}

pub struct ApiResponse {
    pub message: Option<String>,
    pub status: ResponseStatus,
    pub data: Option<serde_json::Value>,
}
