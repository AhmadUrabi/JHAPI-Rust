use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PermissionEditParams {
    pub p_permissions: Permissions,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Permissions {
    pub users: Option<bool>,
    pub permissions: Option<bool>,
    pub query: Option<bool>,
    pub images: Option<bool>,
    pub cost: Option<bool>,
    pub admin: Option<bool>,
    pub stock: Option<bool>,
    pub reports: Option<bool>,
    pub stores: Option<bool>,
}

impl Permissions {
    pub fn new() -> Permissions {
        Permissions {
            users: None,
            permissions: None,
            query: None,
            images: None,
            cost: None,
            admin: None,
            stock: None,
            reports: None,
            stores: None,
        }
    }
}

pub enum PermissionsErrors {
    UserNotFound,
    DBError,
}