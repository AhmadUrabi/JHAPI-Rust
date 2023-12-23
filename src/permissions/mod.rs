use oracle::pool::Pool;

use self::structs::Permissions;

pub mod structs;

pub fn get_user_permissions(user_id: &str, pool: &Pool) -> Result<Permissions, oracle::Error> {
    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to DB");
        return Err(conn.err().unwrap());
    }
    let conn = conn.unwrap();

    let stmt = conn
        .statement("SELECT PERMISSION FROM ODBC_JHC.PERMISSIONS_JHC WHERE USERNAME = :user_id")
        .build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(stmt.err().unwrap());
    }
    let mut stmt = stmt.unwrap();

    let rows = stmt.query(&[&user_id]);
    if rows.is_err() {
        error!("Error executing query");
        return Err(rows.err().unwrap());
    }
    let rows = rows.unwrap();

    let mut permission: Permissions = Permissions::new();

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
) -> Result<String, oracle::Error> {
    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to db");
        return Err(conn.err().unwrap());
    }
    let conn = conn.unwrap();

    let user_id = username.to_string();
    let stmt = conn
        .statement("DELETE FROM ODBC_JHC.PERMISSIONS_JHC WHERE USERNAME = :user_id")
        .build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(stmt.err().unwrap());
    }
    let mut stmt = stmt.unwrap();


    match stmt.execute(&[&user_id]) {
        Ok(_) => (),
        Err(err) => {
            error!("Error executing query: {}", err);
            return Err(err);
        }
    };

    let stmt = conn.statement("INSERT INTO ODBC_JHC.PERMISSIONS_JHC (USERNAME, PERMISSION) VALUES (:user_id, :permission)").build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(stmt.err().unwrap());
    }
    let mut stmt = stmt.unwrap();

    if permissions.users.unwrap_or(false) {
        stmt.execute(&[&user_id, &"users".to_string()])?;
    }
    if permissions.permissions.unwrap_or(false) {
        stmt.execute(&[&user_id, &"permissions".to_string()])?;
    }
    if permissions.query.unwrap_or(false) {
        stmt.execute(&[&user_id, &"query".to_string()])?;
    }
    if permissions.images.unwrap_or(false) {
        stmt.execute(&[&user_id, &"images".to_string()])?;
    }
    if permissions.cost.unwrap_or(false) {
        stmt.execute(&[&user_id, &"cost".to_string()])?;
    }
    if permissions.admin.unwrap_or(false) {
        stmt.execute(&[&user_id, &"admin".to_string()])?;
    }
    if permissions.stock.unwrap_or(false) {
        stmt.execute(&[&user_id, &"stock".to_string()])?;
    }
    if permissions.reports.unwrap_or(false) {
        stmt.execute(&[&user_id, &"reports".to_string()])?;
    }
    if permissions.stores.unwrap_or(false) {
        stmt.execute(&[&user_id, &"stores".to_string()])?;
    }

    match conn.commit() {
        Ok(_) => (),
        Err(err) => {
            error!("Error: {}", err);
            return Err(err);
        }
    }

    Ok("Permissions Updated".to_string())
}
