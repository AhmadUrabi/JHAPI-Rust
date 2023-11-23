use crate::ApiKey;
use oracle::pool::Pool;
use oracle::Result;
use rocket::serde::json::Json;

use bcrypt::{hash, DEFAULT_COST};

use crate::utils::permissions::{is_admin_perm, is_users_perm};

pub mod structs;

use crate::user_control::structs::*;

pub async fn get_users(_key: ApiKey<'_>, pool: &Pool) -> Result<Vec<User>> {
    let mut users: Vec<User> = Vec::new();
    if is_admin_perm(&_key, pool) || is_users_perm(&_key, pool) {
        println!("Admin Permissions Found");
        let conn = pool.get().unwrap();

        let mut stmt = conn
            .statement(
                "SELECT USERNAME, FULLNAME, EMAIL, LOGINDURATION FROM ODBC_JHC.AUTHENTICATION_JHC",
            )
            .build()?;
        let rows = stmt.query(&[]).unwrap();
        for row_result in rows {
            let row = row_result.unwrap();
            let user = User {
                username: row.get::<&str, String>("USERNAME").unwrap(),
                fullname: row.get::<&str, String>("FULLNAME").unwrap(),
                email: row.get::<&str, String>("EMAIL").unwrap(),
                login_duration: row.get::<&str, i32>("LOGINDURATION").unwrap(),
            };
            users.push(user);
        }
    }
    Ok(users)
}

pub async fn get_user(user_id: &str, pool: &Pool) -> Result<User> {
    let conn = pool.get().unwrap();
    let mut stmt = conn
        .statement("SELECT USERNAME, FULLNAME, EMAIL, LOGINDURATION FROM ODBC_JHC.AUTHENTICATION_JHC WHERE USERNAME = :1")
        .build()?;
    let rows = stmt.query(&[&(user_id.to_lowercase())]).unwrap();
    let mut user = User {
        username: "".to_string(),
        fullname: "".to_string(),
        email: "".to_string(),
        login_duration: 0,
    };
    for row_result in rows {
        let row = row_result.unwrap();
        user = User {
            username: row.get::<&str, String>("USERNAME").unwrap(),
            fullname: row.get::<&str, String>("FULLNAME").unwrap(),
            email: row.get::<&str, String>("EMAIL").unwrap(),
            login_duration: row.get::<&str, i32>("LOGINDURATION").unwrap(),
        };
    }
    Ok(user)
}


pub async fn create_user(data: NewUser, pool: &Pool) -> Result<()> {
    let conn = pool.get().unwrap();
    let mut stmt = conn
        .statement("INSERT INTO ODBC_JHC.AUTHENTICATION_JHC (USERNAME, PASSWORD, FULLNAME, EMAIL, LOGINDURATION) VALUES (:1, :2, :3, :4, :5)")
        .build()?;
    stmt.execute(&[
        &(data.p_username).to_lowercase(),
        &(hash(data.p_password, DEFAULT_COST).unwrap()),
        &data.p_fullname,
        &data.p_email,
        &data.p_loginduration,
    ])
    .unwrap();
    conn.commit()?;
    Ok(())
}



pub async fn edit_user(params: Json<EditUserParams>, username:& String, pool: &Pool, is_admin: bool) -> Result<bool> {
    let params_unwrapped = params.into_inner();

    let original_user = match get_user(&username, pool).await {
        Ok(user) => user,
        Err(_) => return Ok(false),
    };

    if original_user.username == "" {
        return Ok(false);
    }

    let mut new_user = User {
        username: original_user.username,
        fullname: original_user.fullname,
        email: original_user.email,
        login_duration: original_user.login_duration,
    };

    if params_unwrapped.p_fullname.is_some() {
        new_user.fullname = params_unwrapped.p_fullname.unwrap();
    }

    if params_unwrapped.p_email.is_some() {
        new_user.email = params_unwrapped.p_email.unwrap();
    }

    if params_unwrapped.p_loginduration.is_some() {
        new_user.login_duration = params_unwrapped.p_loginduration.unwrap();
    }

    let conn = pool.get().unwrap();
    let mut stmt = conn
        .statement("UPDATE ODBC_JHC.AUTHENTICATION_JHC SET FULLNAME = :1, EMAIL = :2, LOGINDURATION = :3 WHERE USERNAME = :4")
        .build()?;
    stmt.execute(&[
        &new_user.fullname,
        &new_user.email,
        &new_user.login_duration,
        &new_user.username,
    ])
    .unwrap();
    conn.commit()?;

    if params_unwrapped.p_password.is_some() && is_admin {
        stmt = conn
            .statement("UPDATE ODBC_JHC.AUTHENTICATION_JHC SET PASSWORD = :1 WHERE USERNAME = :2")
            .build()?;
        stmt.execute(&[
            &hash(params_unwrapped.p_password.unwrap(), DEFAULT_COST).unwrap(),
            &new_user.username,
        ])
        .unwrap();
        conn.commit()?;
    }

    Ok(true)
}

pub async fn delete_user(user_id: &str, pool: &Pool) -> Result<()> {
    let conn = pool.get().unwrap();
    let mut stmt = conn
        .statement("DELETE FROM ODBC_JHC.AUTHENTICATION_JHC WHERE USERNAME = :1")
        .build()?;
    stmt.execute(&[&(user_id.to_lowercase())]).unwrap();
    conn.commit()?;
    Ok(())
}
