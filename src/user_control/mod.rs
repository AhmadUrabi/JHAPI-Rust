use crate::ApiKey;

use crate::utils::check_user_exists;
use crate::utils::structs::APIErrors;

use oracle::pool::Pool;

use rocket::serde::json::Json;

use bcrypt::{hash, DEFAULT_COST};

use crate::utils::permissions::{is_admin_perm, is_users_perm};

pub mod structs;
mod tests;
use crate::user_control::structs::*;
// TODO Split into multiple files
pub async fn get_users(_key: &ApiKey<'_>, pool: &Pool) -> Result<Vec<User>, APIErrors> {
    let mut users: Vec<User> = Vec::new();
    if is_admin_perm(_key, pool) || is_users_perm(_key, pool) {
        println!("Admin Permissions Found");
        let conn = pool.get();
        if conn.is_err() {
            error!("Error connecting to DB");
            return Err(APIErrors::DBError);
        }
        let conn = conn.unwrap();

        let stmt = conn
            .statement(
                "SELECT USERNAME, FULLNAME, EMAIL, LOGINDURATION FROM ODBC_JHC.AUTHENTICATION_JHC",
            )
            .build();
        if stmt.is_err() {
            error!("Error building statement");
            return Err(APIErrors::DBError);
        }
        let mut stmt = stmt.unwrap();

        let rows = stmt.query(&[]);
        if rows.is_err() {
            error!("Error executing query");
            return Err(APIErrors::DBError);
        }
        let rows = rows.unwrap();

        for row_result in rows {
            let row = row_result;
            if row.is_err() {
                error!("Error fetching row");
                return Err(APIErrors::DBError);
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

pub async fn get_user(user_id: &str, pool: &Pool) -> Result<User, APIErrors> {
    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to DB");
        return Err(APIErrors::DBError);
    }
    let conn = conn.unwrap();

    let stmt = conn
        .statement("SELECT USERNAME, FULLNAME, EMAIL, LOGINDURATION FROM ODBC_JHC.AUTHENTICATION_JHC WHERE USERNAME = :1")
        .build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(APIErrors::DBError);
    }
    let mut stmt = stmt.unwrap();

    let rows = stmt.query(&[&(user_id.to_lowercase())]);
    if rows.is_err() {
        error!("Error executing query");
        return Err(APIErrors::DBError);
    }
    let rows = rows.unwrap();

    let mut user = User::new();

    for row_result in rows {
        let row = row_result;
        if row.is_err() {
            error!("Error fetching row");
            return Err(APIErrors::DBError);
        }
        let row = row.unwrap();
        user = User {
            username: row.get::<&str, String>("USERNAME").unwrap(),
            fullname: row.get::<&str, String>("FULLNAME").unwrap(),
            email: row.get::<&str, String>("EMAIL").unwrap(),
            login_duration: row.get::<&str, i32>("LOGINDURATION").unwrap(),
        };
    }
    if user.is_empty() {
        error!("User not found");
        return Err(APIErrors::UserNotFound);
    } else {
        Ok(user)
    }
}



pub async fn create_user(data: NewUser, pool: &Pool) -> Result<(), APIErrors> {
    let conn = pool.get();
    if conn.is_err() {
        error!("Error Connecting to DB");
        return Err(APIErrors::DBError);
    }
    let conn = conn.unwrap();

    // Check if user exists with same username
    match check_user_exists(data.p_username.clone(), &pool) {
        Ok(exists) => {
            if exists {
                error!("User already exists");
                return Err(APIErrors::UserExists);
            }
        }
        Err(_err) => {
            error!("Error checking for duplicate user");
            return Err(APIErrors::DBError);
        }
    }

    let stmt = conn
        .statement("INSERT INTO ODBC_JHC.AUTHENTICATION_JHC (USERNAME, PASSWORD, FULLNAME, EMAIL, LOGINDURATION) VALUES (:1, :2, :3, :4, :5)")
        .build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(APIErrors::DBError);
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
        Err(_err) => {
            error!("Error executing query");
            return Err(APIErrors::DBError);
        }
    }
    
    match conn.commit() {
        Ok(_) => Ok(()),
        Err(_err) => {
            error!("Error commiting");
            Err(APIErrors::DBError)
        }
    }
}



pub async fn edit_user(params: Json<EditUserParams>, username: &str, pool: &Pool, is_admin: bool) -> Result<(), APIErrors> {
    let params_unwrapped = params.into_inner();

    let original_user = match get_user(&username, pool).await {
        Ok(user) => user,
        Err(_) => {
            error!("Error getting user");
            return Err(APIErrors::DBError);
        },
    };

    if original_user.username == "" {
        error!("User not found");
        return Err(APIErrors::UserNotFound);
    }

    let mut new_user = original_user.clone();

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
        return Err(APIErrors::DBError);
    }
    let conn = conn.unwrap();

    let stmt = conn
        .statement("UPDATE ODBC_JHC.AUTHENTICATION_JHC SET FULLNAME = :1, EMAIL = :2, LOGINDURATION = :3 WHERE USERNAME = :4")
        .build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(APIErrors::DBError);
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
            return Err(APIErrors::DBError);
        }
    }


    match conn.commit() {
        Ok(_) => (),
        Err(err) => {
            error!("Error commiting to DB: {}", err);
            return Err(APIErrors::DBError);
        }
    }

    if params_unwrapped.p_password.is_some() && is_admin {
        match conn
            .statement("UPDATE ODBC_JHC.AUTHENTICATION_JHC SET PASSWORD = :1 WHERE USERNAME = :2")
            .build() {
                    Ok(data) => stmt = data,
                    Err(err) => {
                        error!("Error building statement: {}", err);
                        return Err(APIErrors::DBError);
                    }
                }
        

        match stmt.execute(&[
            &hash(params_unwrapped.p_password.unwrap(), DEFAULT_COST).unwrap(),
            &new_user.username,
        ]) {
            Ok(_) => (),
            Err(err) => {
                error!("Error executing query: {}", err);
                return Err(APIErrors::DBError);
            }
        }

        match conn.commit() {
            Ok(_) => (),
            Err(err) => {
                error!("Error commiting to DB: {}", err);
                return Err(APIErrors::DBError);
            }
        }
    }

    Ok(())
}

pub async fn delete_user(user_id: &str, pool: &Pool) -> Result<(), APIErrors> {

    match check_user_exists(user_id.to_string(), pool) {
        Ok(exists) => {
            if !exists {
                error!("User does not exist");
                return Err(APIErrors::UserNotFound);
            }
        }
        Err(_err) => {
            error!("Error checking for duplicate user");
            return Err(APIErrors::DBError);
        }
    }

    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to DB");
        return Err(APIErrors::DBError);
    }
    let conn = conn.unwrap();

    let stmt = conn
        .statement("DELETE FROM ODBC_JHC.AUTHENTICATION_JHC WHERE USERNAME = :1")
        .build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(APIErrors::DBError);
    }
    let mut stmt = stmt.unwrap();

    match stmt.execute(&[&(user_id.to_lowercase())]) {
        Ok(_) => (),
        Err(err) => {
            error!("Error executing query: {}", err);
            return Err(APIErrors::DBError);
        }
    }

    match conn.commit() {
        Ok(_) => (),
        Err(err) => {
            error!("Error commiting to DB: {}", err);
            return Err(APIErrors::DBError);
        }
    }
    Ok(())
}
