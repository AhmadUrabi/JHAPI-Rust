use oracle::pool::Pool;
use oracle::Result;

use crate::signing::decode_token_data;

use crate::ApiKey;

pub mod structs;

pub fn get_user_permissions(user_id: &str, pool: &Pool) -> Result<Vec<String>> {
    let conn = pool.get()?;
    let mut stmt = conn
        .statement("SELECT PERMISSION FROM ODBC_JHC.PERMISSIONS_JHC WHERE USERNAME = :user_id")
        .build()?;
    let rows = stmt.query(&[&user_id])?;

    let mut permissions: Vec<String> = vec![];
    for row_result in rows {
        let row = row_result?;

        let permission = row.get("PERMISSION")?;

        permissions.push(permission);
    }

    Ok(permissions)
}

pub fn edit_user_permissions(
    token: ApiKey<'_>,
    pool: &Pool,
    permissions: Vec<String>,
) -> Result<String> {
    let conn = pool.get()?;
    let user_id = decode_token_data(token.0).unwrap().USER_ID.unwrap();
    let mut stmt = conn
        .statement("DELETE FROM ODBC_JHC.PERMISSIONS_JHC WHERE USERNAME = :user_id")
        .build()?;
    stmt.execute(&[&user_id])?;

    let mut stmt = conn.statement("INSERT INTO ODBC_JHC.PERMISSIONS_JHC (USERNAME, PERMISSION) VALUES (:user_id, :permission)").build()?;

    for permission in permissions {
        stmt.execute(&[&user_id, &permission])?;
    }
    conn.commit()?;
    Ok("Permissions Updated".to_string())
}
