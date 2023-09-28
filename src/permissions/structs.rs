use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PermissionEditParams {
    pub pUserName: String,
    pub pPermissions: Permissions,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Permissions {
    pub users: bool,
    pub permissions: bool,
    pub query: bool,
    pub images: bool,
    pub cost: bool,
    pub admin: bool,
    pub stock: bool,
    pub reports: bool,
}


