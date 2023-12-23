use crate::ApiKey;

use oracle::pool::Pool;

use rocket::serde::json::Json;

use bcrypt::{hash, DEFAULT_COST};

use crate::utils::permissions::{is_admin_perm, is_users_perm};

pub mod structs;

use crate::user_control::structs::*;

pub async fn get_users(_key: ApiKey<'_>, pool: &Pool) -> Result<Vec<User>, oracle::Error> {
    let mut users: Vec<User> = Vec::new();
    if is_admin_perm(&_key, pool) || is_users_perm(&_key, pool) {
        println!("Admin Permissions Found");
        let conn = pool.get();
        if conn.is_err() {
            error!("Error connecting to DB");
            return Err(conn.err().unwrap());
        }
        let conn = conn.unwrap();

        let stmt = conn
            .statement(
                "SELECT USERNAME, FULLNAME, EMAIL, LOGINDURATION FROM ODBC_JHC.AUTHENTICATION_JHC",
            )
            .build();
        if stmt.is_err() {
            error!("Error building statement");
            return Err(stmt.err().unwrap());
        }
        let mut stmt = stmt.unwrap();

        let rows = stmt.query(&[]);
        if rows.is_err() {
            error!("Error executing query");
            return Err(rows.err().unwrap());
        }
        let rows = rows.unwrap();

        for row_result in rows {
            let row = row_result;
            if row.is_err() {
                error!("Error fetching row");
                return Err(row.err().unwrap());
            }
            let row = row.unwrap();

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

pub async fn get_user(user_id: &str, pool: &Pool) -> Result<User, oracle::Error> {
    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to DB");
        return Err(conn.err().unwrap());
    }
    let conn = conn.unwrap();

    let stmt = conn
        .statement("SELECT USERNAME, FULLNAME, EMAIL, LOGINDURATION FROM ODBC_JHC.AUTHENTICATION_JHC WHERE USERNAME = :1")
        .build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(stmt.err().unwrap());
    }
    let mut stmt = stmt.unwrap();

    let rows = stmt.query(&[&(user_id.to_lowercase())]);
    if rows.is_err() {
        error!("Error executing query");
        return Err(rows.err().unwrap());
    }
    let rows = rows.unwrap();

    let mut user = User::new();

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


pub async fn create_user(data: NewUser, pool: &Pool) -> Result<(), String> {
    let conn = pool.get();
    if conn.is_err() {
        error!("Error Connecting to DB");
        return Err(conn.err().unwrap().to_string());
    }
    let conn = conn.unwrap();


    let stmt = conn
        .statement("INSERT INTO ODBC_JHC.AUTHENTICATION_JHC (USERNAME, PASSWORD, FULLNAME, EMAIL, LOGINDURATION) VALUES (:1, :2, :3, :4, :5)")
        .build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(stmt.err().unwrap().to_string());
    }
    let mut stmt = stmt.unwrap();

    match stmt.execute(&[
        &(data.p_username).to_lowercase(),
        &(hash(data.p_password, DEFAULT_COST).unwrap()),
        &data.p_fullname,
        &data.p_email,
        &data.p_loginduration,
    ]) {
        Ok(_) => (),
        Err(err) => {
            error!("Error executing query");
            return Err(err.to_string());
        }
    }
    
    match conn.commit() {
        Ok(_) => Ok(()),
        Err(err) => {
            error!("Error commiting");
            return Err(err.to_string());
        }
    }
}



pub async fn edit_user(params: Json<EditUserParams>, username:& String, pool: &Pool, is_admin: bool) -> Result<bool, String> {
    let params_unwrapped = params.into_inner();

    let original_user = match get_user(&username, pool).await {
        Ok(user) => user,
        Err(_) => return Ok(false),
    };

    if original_user.username == "" {
        return Ok(false);
    }

    let mut new_user = User::new();

    if params_unwrapped.p_fullname.is_some() {
        new_user.fullname = params_unwrapped.p_fullname.unwrap();
    }

    if params_unwrapped.p_email.is_some() {
        new_user.email = params_unwrapped.p_email.unwrap();
    }

    if params_unwrapped.p_loginduration.is_some() {
        new_user.login_duration = params_unwrapped.p_loginduration.unwrap();
    }

    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to DB");
        return Err(conn.err().unwrap().to_string());
    }
    let conn = conn.unwrap();

    let stmt = conn
        .statement("UPDATE ODBC_JHC.AUTHENTICATION_JHC SET FULLNAME = :1, EMAIL = :2, LOGINDURATION = :3 WHERE USERNAME = :4")
        .build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(stmt.err().unwrap().to_string());
    }
    let mut stmt = stmt.unwrap();

    match stmt.execute(&[
        &new_user.fullname,
        &new_user.email,
        &new_user.login_duration,
        &new_user.username,
    ]) {
        Ok(_) => (),
        Err(err) => {
            error!("Error executing query: {}", err);
            return Err(err.to_string());
        }
    }


    match conn.commit() {
        Ok(_) => (),
        Err(err) => {
            error!("Error commiting to DB: {}", err);
            return Err(err.to_string());
        }
    }

    if params_unwrapped.p_password.is_some() && is_admin {
        match conn
            .statement("UPDATE ODBC_JHC.AUTHENTICATION_JHC SET PASSWORD = :1 WHERE USERNAME = :2")
            .build() {
                    Ok(data) => stmt = data,
                    Err(err) => {
                        error!("Error building statement: {}", err);
                        return Err(err.to_string());
                    }
                }
        

        match stmt.execute(&[
            &hash(params_unwrapped.p_password.unwrap(), DEFAULT_COST).unwrap(),
            &new_user.username,
        ]) {
            Ok(_) => (),
            Err(err) => {
                error!("Error executing query: {}", err);
                return Err(err.to_string());
            }
        }

        match conn.commit() {
            Ok(_) => (),
            Err(err) => {
                error!("Error commiting to DB: {}", err);
                return Err(err.to_string());
            }
        }
    }

    Ok(true)
}

pub async fn delete_user(user_id: &str, pool: &Pool) -> Result<(), String> {
    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to DB");
        return Err(conn.err().unwrap().to_string());
    }
    let conn = conn.unwrap();

    let stmt = conn
        .statement("DELETE FROM ODBC_JHC.AUTHENTICATION_JHC WHERE USERNAME = :1")
        .build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(stmt.err().unwrap().to_string());
    }
    let mut stmt = stmt.unwrap();

    match stmt.execute(&[&(user_id.to_lowercase())]) {
        Ok(_) => (),
        Err(err) => {
            error!("Error executing query: {}", err);
            return Err(err.to_string());
        }
    }

    match conn.commit() {
        Ok(_) => (),
        Err(err) => {
            error!("Error commiting to DB: {}", err);
            return Err(err.to_string());
        }
    }
    Ok(())
}
