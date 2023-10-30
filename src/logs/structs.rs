#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LogData {
    pub id: i32,
    pub username: Option<String>,
    pub route: String,
    pub parameters: Option<String>,
    pub result: String,
    pub timestamp: String,
    pub token_used: String,
    pub ip_address: Option<String>,
}