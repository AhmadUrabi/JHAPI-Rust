use crate::ApiKey;

pub fn get_user_permissions(token: ApiKey<'_>, pool: &Pool) -> Result<Vec<String>> {
    let conn = pool.get()?;

    let mut stmt = conn.statement("SELECT PERMISSION FROM ODBC_JHC.JHC_USER_PERMISSIONS WHERE USER_ID = :user_id").build()?;
    let rows = stmt.query(&[&token.user_id])?;

    let mut permissions : Vec<String> = vec![];
    
    for row_result in rows {
        let row = row_result?;

        let permission = row.get("PERMISSION")?;

        permissions.push(permission);
    }

    Ok(permissions)
}

pub fn edit_user_permissions(token: ApiKey<'_>, pool: &Pool, permissions: Vec<String>) -> Result<()> {
    let conn = pool.get()?;

    let mut stmt = conn.statement("DELETE FROM ODBC_JHC.JHC_USER_PERMISSIONS WHERE USER_ID = :user_id").build()?;
    stmt.execute(&[&token.user_id])?;

    let mut stmt = conn.statement("INSERT INTO ODBC_JHC.JHC_USER_PERMISSIONS (USER_ID, PERMISSION) VALUES (:user_id, :permission)").build()?;

    for permission in permissions {
        stmt.execute(&[&token.user_id, &permission])?;
    }

    Ok(())
}