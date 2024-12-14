use oracle::pool::Pool;

use crate::utils::check_user_exists;

use crate::utils::sql::SQLManager;
use crate::utils::structs::APIError;

use self::structs::Permissions;

pub mod structs;

// TODO: try to optimize this function
pub async fn get_user_permissions(
    user_id: &str,
    sql_manager: &SQLManager,
    pool: &Pool,
) -> Result<Permissions, APIError> {
    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to DB");
        return Err(APIError::DBError);
    }

    // Check for user
    if !check_user_exists(user_id.to_string(), pool, &sql_manager)
        .await
        .unwrap_or(false)
    {
        error!("User does not exist");
        return Err(APIError::DataNotFound);
    }

    let conn = conn.unwrap();

    let stmt = conn
        .statement(sql_manager.get_sql("get_user_permissions")?.as_str())
        .build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(APIError::DBError);
    }
    let mut stmt = stmt.unwrap();

    let rows = stmt.query(&[&user_id]);
    if rows.is_err() {
        error!("Error executing query");
        return Err(APIError::DBError);
    }
    let rows = rows.unwrap();

    let mut permission: Permissions = Permissions::new();

    for row_result in rows {
        if row_result.is_err() {
            error!("Error fetching row");
            return Err(APIError::DBError);
        }
        let row = row_result.unwrap();

        // Unsafe Unwrap Here
        let perm: String = row.get(0).unwrap();
        match perm.as_str() {
            "users" => permission.users = true,
            "permissions" => permission.permissions = true,
            "query" => permission.query = true,
            "images" => permission.images = true,
            "cost" => permission.cost = true,
            "admin" => permission.admin = true,
            "stock" => permission.stock = true,
            "reports" => permission.reports = true,
            "stores" => permission.stores = true,
            _ => {}
        }
    }
    Ok(permission)
}

pub async fn edit_user_permissions(
    username: String,
    pool: &Pool,
    sql_manager: &SQLManager,
    permissions: Permissions,
) -> Result<String, APIError> {
    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to db");
        return Err(APIError::DBError);
    }
    let conn = conn.unwrap();

    // Check for user
    if !check_user_exists(username.to_string(), pool, &sql_manager)
        .await
        .unwrap_or(false)
    {
        error!("User does not exist");
        return Err(APIError::DataNotFound);
    }

    // Same Weird threads error as routes/users.rs
    let insert_stmt = sql_manager.get_sql("insert_user_permissions")?;
    let user_id = username.to_string();
    let stmt = conn
        .statement(sql_manager.get_sql("delete_user_permissions")?.as_str())
        .build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(APIError::DBError);
    }
    let mut stmt = stmt.unwrap();

    match stmt.execute(&[&user_id]) {
        Ok(_) => (),
        Err(err) => {
            error!("Error executing query: {}", err);
            return Err(APIError::DBError);
        }
    };

    let stmt = conn.statement(insert_stmt.as_str()).build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(APIError::DBError);
    }
    let mut stmt = stmt.unwrap();

    if permissions.users {
        stmt.execute(&[&user_id, &"users".to_string()]).unwrap();
    }
    if permissions.permissions {
        stmt.execute(&[&user_id, &"permissions".to_string()])
            .unwrap();
    }
    if permissions.query {
        stmt.execute(&[&user_id, &"query".to_string()]).unwrap();
    }
    if permissions.images {
        stmt.execute(&[&user_id, &"images".to_string()]).unwrap();
    }
    if permissions.cost {
        stmt.execute(&[&user_id, &"cost".to_string()]).unwrap();
    }
    if permissions.admin {
        stmt.execute(&[&user_id, &"admin".to_string()]).unwrap();
    }
    if permissions.stock {
        stmt.execute(&[&user_id, &"stock".to_string()]).unwrap();
    }
    if permissions.reports {
        stmt.execute(&[&user_id, &"reports".to_string()]).unwrap();
    }
    if permissions.stores {
        stmt.execute(&[&user_id, &"stores".to_string()]).unwrap();
    }

    match conn.commit() {
        Ok(_) => (),
        Err(err) => {
            error!("Error: {}", err);
            return Err(APIError::DBError);
        }
    }

    Ok("Permissions Updated".to_string())
}
