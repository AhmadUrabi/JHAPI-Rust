#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LogData {
    pub LOG_ID: i32,
    pub USERNAME: Option<String>,
    pub ROUTE: String,
    pub PARAMETERS: Option<String>,
    pub RESULT: String,
    pub TIMESTAMP: String,
    pub TOKEN_USED: String,
    pub IP_ADDRESS: Option<String>,
    pub METHOD: Option<String>,
}