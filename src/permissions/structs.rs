use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PermissionEditParams {
    pub p_permissions: Permissions,
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
    pub stores: bool,
}

impl Permissions {
    pub fn new() -> Permissions {
        Permissions {
            users: false,
            permissions: false,
            query: false,
            images: false,
            cost: false,
            admin: false,
            stock: false,
            reports: false,
            stores: false,
        }
    }
}