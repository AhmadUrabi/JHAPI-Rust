use oracle::pool::Pool;

use crate::ApiKey;

use crate::permissions::get_user_permissions;

use crate::signing::decode_token_data;

// Check for Admin Permissions
pub fn is_admin_perm(_key: &ApiKey<'_>, pool: &Pool) -> bool {
    match decode_token_data(_key.0) {
        Some(x) => {
            let user_id = x.USER_ID.unwrap();
            let permissions = get_user_permissions(&user_id, pool);
            if permissions.is_err() {
                return false;
            }
            let permissions = permissions.unwrap();
            match permissions.admin {
                Some(x) => return x,
                None => return false,
            }
        },
        None => return false,
    }

}

// Check for Permissions Permissions
pub fn is_perm_perm(_key: &ApiKey<'_>, pool: &Pool) -> bool {
    let user_id = decode_token_data(_key.0).unwrap().USER_ID.unwrap();
    let permissions = get_user_permissions(&user_id, pool);
            if permissions.is_err() {
                return false;
            }
            let permissions = permissions.unwrap();
    match permissions.permissions {
        Some(x) => return x,
        None => return false,
    }
}

// Check for Users Control Permissions
pub fn is_users_perm(_key: &ApiKey<'_>, pool: &Pool) -> bool {
    match decode_token_data(_key.0) {
        Some(x) => {
            let user_id = x.USER_ID.unwrap();
            let permissions = get_user_permissions(&user_id, pool);
            if permissions.is_err() {
                return false;
            }
            let permissions = permissions.unwrap();
            match permissions.users {
                Some(x) => return x,
                None => return false,
            }
        },
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
            match permissions.images {
                Some(x) => return x,
                None => return false,
            }
        },
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
            match permissions.cost {
                Some(x) => return x,
                None => return false,
            }
        },
        None => return false,
    }
}

// Check for Product Query Permissions
pub fn is_query_perm(_key: &ApiKey<'_>, pool: &Pool) -> bool {
    match decode_token_data(_key.0) {
        Some(x) => {
            let user_id = x.USER_ID.unwrap();
            let permissions = get_user_permissions(&user_id, pool);
            if permissions.is_err() {
                return false;
            }
            let permissions = permissions.unwrap();
            match permissions.query {
                Some(x) => return x,
                None => return false,
            }
        },
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
            match permissions.stock {
                Some(x) => return x,
                None => return false,
            }
        },
        None => return false,
    }
}

// Check for Reports Permissions
#[allow(dead_code)]
pub fn is_reports_perm(_key: &ApiKey<'_>, pool: &Pool) -> bool {
    match decode_token_data(_key.0) {
        Some(x) => {
            let user_id = x.USER_ID.unwrap();
            let permissions = get_user_permissions(&user_id, pool);
            if permissions.is_err() {
                return false;
            }
            let permissions = permissions.unwrap();
            match permissions.reports {
                Some(x) => return x,
                None => return false,
            }
        },
        None => return false,
    }
}

// Check for Stores Permissions
pub fn is_stores_perm(_key: &ApiKey<'_>, pool: &Pool) -> bool {
    match decode_token_data(_key.0) {
        Some(x) => {
            let user_id = x.USER_ID.unwrap();
            let permissions = get_user_permissions(&user_id, pool);
            if permissions.is_err() {
                return false;
            }
            let permissions = permissions.unwrap();
            match permissions.stores {
                Some(x) => return x,
                None => return false,
            }
        },
        None => return false,
    }
}
