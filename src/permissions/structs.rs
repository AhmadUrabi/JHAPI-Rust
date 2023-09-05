use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PermissionEditParams {
    pub pUserName: String,
    pub pPermissions: Vec<String>,
}
