use oracle::pool::Pool;
use crate::ApiKey;
use crate::permissions::get_user_permissions;
use crate::signing::decode_token_data;


pub fn is_admin_perm(_key: &ApiKey<'_>, pool: &Pool) -> bool {
    let user_id = decode_token_data(_key.0).unwrap().USER_ID.unwrap();
    let permissions: Vec<String> = get_user_permissions(&user_id, pool).unwrap();
    if permissions.contains(&"admin".to_string()){
        println!("Admin Permissions Found");
        return true;
    }
    false
}