use oracle::pool::Pool;

use crate::server::request_guard::api_key::ApiKey;

use crate::functions::permissions::get_user_permissions;

use crate::functions::authentication::decode_token_data;

/// Check for Admin Permissions
pub fn has_admin_perm(_key: &ApiKey<'_>, pool: &Pool) -> bool {
    match decode_token_data(_key.0) {
        Some(x) => {
            let user_id = x.USER_ID.unwrap();
            let permissions = get_user_permissions(&user_id, pool);
            if permissions.is_err() {
                return false;
            }
            let permissions = permissions.unwrap();
            permissions.admin
        }
        None => return false,
    }
}

/// Check for Permission Management Permissions
pub fn has_permissions_perm(_key: &ApiKey<'_>, pool: &Pool) -> bool {
    match decode_token_data(_key.0) {
        Some(x) => {
            let user_id = x.USER_ID.unwrap();
            let permissions = get_user_permissions(&user_id, pool);
            if permissions.is_err() {
                return false;
            }
            let permissions = permissions.unwrap();
            permissions.permissions
        }
        None => return false,
    }
}

/// Check for Users Control Permissions
pub fn has_users_perm(_key: &ApiKey<'_>, pool: &Pool) -> bool {
    match decode_token_data(_key.0) {
        Some(x) => {
            let user_id = x.USER_ID.unwrap();
            let permissions = get_user_permissions(&user_id, pool);
            if permissions.is_err() {
                return false;
            }
            let permissions = permissions.unwrap();
            permissions.users
        }
        None => return false,
    }
}

// Check for Image Fetching Permissions
pub fn is_images_perm(_key: &ApiKey<'_>, pool: &Pool) -> bool {
    match decode_token_data(_key.0) {
        Some(x) => {
            let user_id = x.USER_ID.unwrap();
            let permissions = get_user_permissions(&user_id, pool);
            if permissions.is_err() {
                return false;
            }
            let permissions = permissions.unwrap();
            permissions.images
        }
        None => return false,
    }
}

// Check for Product Cost Permissions
pub fn is_cost_perm(_key: &ApiKey<'_>, pool: &Pool) -> bool {
    match decode_token_data(_key.0) {
        Some(x) => {
            let user_id = x.USER_ID.unwrap();
            let permissions = get_user_permissions(&user_id, pool);
            if permissions.is_err() {
                return false;
            }
            let permissions = permissions.unwrap();
            permissions.cost
        }
        None => return false,
    }
}

/// Check for Product Query Permissions
pub fn has_query_perm(_key: &ApiKey<'_>, pool: &Pool) -> bool {
    match decode_token_data(_key.0) {
        Some(x) => {
            let user_id = x.USER_ID.unwrap();
            let permissions = get_user_permissions(&user_id, pool);
            if permissions.is_err() {
                return false;
            }
            let permissions = permissions.unwrap();
            permissions.query
        }
        None => return false,
    }
}

// Check for Stock Permissions
#[allow(dead_code)]
pub fn is_stock_perm(_key: &ApiKey<'_>, pool: &Pool) -> bool {
    match decode_token_data(_key.0) {
        Some(x) => {
            let user_id = x.USER_ID.unwrap();
            let permissions = get_user_permissions(&user_id, pool);
            if permissions.is_err() {
                return false;
            }
            let permissions = permissions.unwrap();
            permissions.stock
        }
        None => return false,
    }
}


#[allow(dead_code)]
/// Check for Reports Permissions
pub fn has_reports_perm(_key: &ApiKey<'_>, pool: &Pool) -> bool {
    match decode_token_data(_key.0) {
        Some(x) => {
            let user_id = x.USER_ID.unwrap();
            let permissions = get_user_permissions(&user_id, pool);
            if permissions.is_err() {
                return false;
            }
            let permissions = permissions.unwrap();
            permissions.reports
        }
        None => return false,
    }
}

/// Check for Stores Permissions
pub fn has_stores_perm(_key: &ApiKey<'_>, pool: &Pool) -> bool {
    match decode_token_data(_key.0) {
        Some(x) => {
            let user_id = x.USER_ID.unwrap();
            let permissions = get_user_permissions(&user_id, pool);
            if permissions.is_err() {
                return false;
            }
            let permissions = permissions.unwrap();
            permissions.stores
        }
        None => return false,
    }
}
