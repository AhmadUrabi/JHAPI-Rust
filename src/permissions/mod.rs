use oracle::pool::Pool;

use crate::utils::structs::APIErrors;
use crate::utils::check_user_exists;

use self::structs::Permissions;

pub mod structs;


// TODO: try to optimize this function
pub fn get_user_permissions(user_id: &str, pool: &Pool) -> Result<Permissions, APIErrors> {
    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to DB");
        return Err(APIErrors::DBError);
    }

    // Check for user
    if !check_user_exists(user_id.to_string(), pool).unwrap_or(false) {
        error!("User does not exist");
        return Err(APIErrors::UserNotFound);
    }

    let conn = conn.unwrap();

    let stmt = conn
        .statement("SELECT PERMISSION FROM ODBC_JHC.PERMISSIONS_JHC WHERE USERNAME = :user_id")
        .build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(APIErrors::DBError);
    }
    let mut stmt = stmt.unwrap();

    let rows = stmt.query(&[&user_id]);
    if rows.is_err() {
        error!("Error executing query");
        return Err(APIErrors::DBError);
    }
    let rows = rows.unwrap();

    let mut permission: Permissions = Permissions::new();

    for row_result in rows {
        if row_result.is_err() {
            error!("Error fetching row");
            return Err(APIErrors::DBError);
        }
        let row = row_result.unwrap();

        // Unsafe Unwrap Here
        let perm: String = row.get(0).unwrap();
        match perm.as_str() {
            "users" => permission.users = Some(true),
            "permissions" => permission.permissions = Some(true),
            "query" => permission.query = Some(true),
            "images" => permission.images = Some(true),
            "cost" => permission.cost = Some(true),
            "admin" => permission.admin = Some(true),
            "stock" => permission.stock = Some(true),
            "reports" => permission.reports = Some(true),
            "stores" => permission.stores = Some(true),
            _ => {}
        }
    }

    Ok(permission)
}

pub fn edit_user_permissions(
    username: String,
    pool: &Pool,
    permissions: Permissions,
) -> Result<String, APIErrors> {
    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to db");
        return Err(APIErrors::DBError);
    }
    let conn = conn.unwrap();

    // Check for user
    if !check_user_exists(username.to_string(), pool).unwrap_or(false) {
        error!("User does not exist");
        return Err(APIErrors::UserNotFound);
    }


    let user_id = username.to_string();
    let stmt = conn
        .statement("DELETE FROM ODBC_JHC.PERMISSIONS_JHC WHERE USERNAME = :user_id")
        .build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(APIErrors::DBError);
    }
    let mut stmt = stmt.unwrap();


    match stmt.execute(&[&user_id]) {
        Ok(_) => (),
        Err(err) => {
            error!("Error executing query: {}", err);
            return Err(APIErrors::DBError);
        }
    };

    let stmt = conn.statement("INSERT INTO ODBC_JHC.PERMISSIONS_JHC (USERNAME, PERMISSION) VALUES (:user_id, :permission)").build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(APIErrors::DBError);
    }
    let mut stmt = stmt.unwrap();


    
    if permissions.users.unwrap_or(false) {
        stmt.execute(&[&user_id, &"users".to_string()]).unwrap();
    }
    if permissions.permissions.unwrap_or(false) {
        stmt.execute(&[&user_id, &"permissions".to_string()]).unwrap();
    }
    if permissions.query.unwrap_or(false) {
        stmt.execute(&[&user_id, &"query".to_string()]).unwrap();
    }
    if permissions.images.unwrap_or(false) {
        stmt.execute(&[&user_id, &"images".to_string()]).unwrap();
    }
    if permissions.cost.unwrap_or(false) {
        stmt.execute(&[&user_id, &"cost".to_string()]).unwrap();
    }
    if permissions.admin.unwrap_or(false) {
        stmt.execute(&[&user_id, &"admin".to_string()]).unwrap();
    }
    if permissions.stock.unwrap_or(false) {
        stmt.execute(&[&user_id, &"stock".to_string()]).unwrap();
    }
    if permissions.reports.unwrap_or(false) {
        stmt.execute(&[&user_id, &"reports".to_string()]).unwrap();
    }
    if permissions.stores.unwrap_or(false) {
        stmt.execute(&[&user_id, &"stores".to_string()]).unwrap();
    }

    match conn.commit() {
        Ok(_) => (),
        Err(err) => {
            error!("Error: {}", err);
            return Err(APIErrors::DBError);
        }
    }

    Ok("Permissions Updated".to_string())
}
