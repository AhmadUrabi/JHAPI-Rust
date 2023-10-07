use oracle::pool::Pool;
use crate::permissions::structs::Permissions;

use crate::ApiKey;

use crate::permissions::get_user_permissions;

use crate::signing::decode_token_data;

// Check for Admin Permissions
pub fn is_admin_perm(_key: &ApiKey<'_>, pool: &Pool) -> bool {
    let user_id = decode_token_data(_key.0).unwrap().USER_ID.unwrap();
    let permissions: Permissions = get_user_permissions(&user_id, pool).unwrap();
    return permissions.admin.unwrap();
}

// Check for Permissions Permissions
pub fn is_perm_perm(_key: &ApiKey<'_>, pool: &Pool) -> bool {
    let user_id = decode_token_data(_key.0).unwrap().USER_ID.unwrap();
    let permissions: Permissions = get_user_permissions(&user_id, pool).unwrap();
    return permissions.permissions.unwrap();
}

// Check for Users Control Permissions
pub fn is_users_perm(_key: &ApiKey<'_>, pool: &Pool) -> bool {
    let user_id = decode_token_data(_key.0).unwrap().USER_ID.unwrap();
    let permissions: Permissions = get_user_permissions(&user_id, pool).unwrap();
    return permissions.users.unwrap();
}

// Check for Image Fetching Permissions
pub fn is_images_perm(_key: &ApiKey<'_>, pool: &Pool) -> bool {
    let user_id = decode_token_data(_key.0).unwrap().USER_ID.unwrap();
    let permissions: Permissions = get_user_permissions(&user_id, pool).unwrap();
    return permissions.images.unwrap();
}

// Check for Product Cost Permissions
pub fn is_cost_perm(_key: &ApiKey<'_>, pool: &Pool) -> bool {
    let user_id = decode_token_data(_key.0).unwrap().USER_ID.unwrap();
    let permissions: Permissions = get_user_permissions(&user_id, pool).unwrap();
    return permissions.cost.unwrap();
}

// Check for Product Query Permissions
pub fn is_query_perm(_key: &ApiKey<'_>, pool: &Pool) -> bool {
    let user_id = decode_token_data(_key.0).unwrap().USER_ID.unwrap();
    let permissions: Permissions = get_user_permissions(&user_id, pool).unwrap();
    return permissions.query.unwrap();
}
