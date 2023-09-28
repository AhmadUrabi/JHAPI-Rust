use oracle::pool::Pool;
use oracle::Result;

use crate::signing::decode_token_data;

use crate::ApiKey;

use self::structs::Permissions;

pub mod structs;

pub fn get_user_permissions(user_id: &str, pool: &Pool) -> Result<Permissions> {
    let conn = pool.get()?;
    let mut stmt = conn
        .statement("SELECT PERMISSION FROM ODBC_JHC.PERMISSIONS_JHC WHERE USERNAME = :user_id")
        .build()?;
    let rows = stmt.query(&[&user_id])?;

    let mut permission: Permissions = Permissions {
        users: false,
        permissions: false,
        query: false,
        images: false,
        cost: false,
        admin: false,
        stock: false,
        reports: false,
    };
    for row_result in rows {
        let row = row_result?;
        let perm: String = row.get(0)?;
        println!("Permission: {}", perm);
        match perm.as_str() {
            "users" => permission.users = true,
            "permissions" => permission.permissions = true,
            "query" => permission.query = true,
            "images" => permission.images = true,
            "cost" => permission.cost = true,
            "admin" => permission.admin = true,
            "stock" => permission.stock = true,
            "reports" => permission.reports = true,
            _ => {}
        }
        println!("Permission: {:?}", permission.permissions)
    }

    Ok(permission)
}

pub fn edit_user_permissions(
    token: ApiKey<'_>,
    pool: &Pool,
    permissions: Permissions,
) -> Result<String> {
    let conn = pool.get()?;
    let user_id = decode_token_data(token.0).unwrap().USER_ID.unwrap();
    let mut stmt = conn
        .statement("DELETE FROM ODBC_JHC.PERMISSIONS_JHC WHERE USERNAME = :user_id")
        .build()?;
    stmt.execute(&[&user_id])?;

    let mut stmt = conn.statement("INSERT INTO ODBC_JHC.PERMISSIONS_JHC (USERNAME, PERMISSION) VALUES (:user_id, :permission)").build()?;

    if permissions.users {
        stmt.execute(&[&user_id, &"users".to_string()])?;
    }
    if permissions.permissions {
        stmt.execute(&[&user_id, &"permissions".to_string()])?;
    }
    if permissions.query {
        stmt.execute(&[&user_id, &"query".to_string()])?;
    }
    if permissions.images {
        stmt.execute(&[&user_id, &"images".to_string()])?;
    }
    if permissions.cost {
        stmt.execute(&[&user_id, &"cost".to_string()])?;
    }
    if permissions.admin {
        stmt.execute(&[&user_id, &"admin".to_string()])?;
    }
    if permissions.stock {
        stmt.execute(&[&user_id, &"stock".to_string()])?;
    }
    if permissions.reports {
        stmt.execute(&[&user_id, &"reports".to_string()])?;
    }


    conn.commit()?;
    Ok("Permissions Updated".to_string())
}
