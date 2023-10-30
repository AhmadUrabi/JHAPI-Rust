use oracle::pool::Pool;
use oracle::Result;


use self::structs::Permissions;

pub mod structs;

pub fn get_user_permissions(user_id: &str, pool: &Pool) -> Result<Permissions> {
    let conn = pool.get()?;
    let mut stmt = conn
        .statement("SELECT PERMISSION FROM ODBC_JHC.PERMISSIONS_JHC WHERE USERNAME = :user_id")
        .build()?;
    let rows = stmt.query(&[&user_id])?;

    let mut permission: Permissions = Permissions {
        users: Some(false),
        permissions: Some(false),
        query: Some(false),
        images: Some(false),
        cost: Some(false),
        admin: Some(false),
        stock: Some(false),
        reports: Some(false),
        stores: Some(false),
    };
    for row_result in rows {
        let row = row_result?;
        let perm: String = row.get(0)?;
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
) -> Result<String> {
    let conn = pool.get()?;
    let user_id = username.to_string();
    let mut stmt = conn
        .statement("DELETE FROM ODBC_JHC.PERMISSIONS_JHC WHERE USERNAME = :user_id")
        .build()?;
    stmt.execute(&[&user_id])?;

    let mut stmt = conn.statement("INSERT INTO ODBC_JHC.PERMISSIONS_JHC (USERNAME, PERMISSION) VALUES (:user_id, :permission)").build()?;

    if permissions.users.unwrap() {
        stmt.execute(&[&user_id, &"users".to_string()])?;
    }
    if permissions.permissions.unwrap() {
        stmt.execute(&[&user_id, &"permissions".to_string()])?;
    }
    if permissions.query.unwrap() {
        stmt.execute(&[&user_id, &"query".to_string()])?;
    }
    if permissions.images.unwrap() {
        stmt.execute(&[&user_id, &"images".to_string()])?;
    }
    if permissions.cost.unwrap() {
        stmt.execute(&[&user_id, &"cost".to_string()])?;
    }
    if permissions.admin.unwrap() {
        stmt.execute(&[&user_id, &"admin".to_string()])?;
    }
    if permissions.stock.unwrap() {
        stmt.execute(&[&user_id, &"stock".to_string()])?;
    }
    if permissions.reports.unwrap() {
        stmt.execute(&[&user_id, &"reports".to_string()])?;
    }
    if permissions.stores.unwrap() {
        stmt.execute(&[&user_id, &"stores".to_string()])?;
    }

    conn.commit()?;
    Ok("Permissions Updated".to_string())
}
