use crate::server::request_guard::api_key::ApiKey;

use crate::utils::check_user_exists;

use crate::utils::sql::SQLManager;
use crate::utils::structs::APIErrors;

use oracle::pool::Pool;

use bcrypt::{hash, DEFAULT_COST};

use crate::utils::permissions::{has_admin_perm, has_users_perm};

pub mod structs;

use crate::functions::users::structs::*;

pub async fn get_users(_key: &ApiKey<'_>, sql_manager: &SQLManager, pool: &Pool) -> Result<Vec<User>, APIErrors> {
    let mut users: Vec<User> = Vec::new();
    if has_admin_perm(_key, pool, &sql_manager).await || has_users_perm(_key, pool, &sql_manager).await {
        println!("Admin Permissions Found");
        let conn = pool.get();
        if conn.is_err() {
            error!("Error connecting to DB");
            return Err(APIErrors::DBError);
        }
        let conn = conn.unwrap();

        let stmt = conn
            .statement(sql_manager.get_sql("get_users")?.as_str())
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
                username: row.get("USERNAME").unwrap(),
                fullname: row.get("FULLNAME").unwrap(),
                email: row.get("EMAIL").unwrap(),
                login_duration: row.get("LOGINDURATION").unwrap(),
            };
            users.push(user);
        }
    }
    Ok(users)
}

pub async fn get_user(user_id: &str, sql_manager: &SQLManager, pool: &Pool) -> Result<User, APIErrors> {
    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to DB");
        return Err(APIErrors::DBError);
    }
    let conn = conn.unwrap();

    let stmt = conn
        .statement(sql_manager.get_sql("get_user_by_id")?.as_str())
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
            username: row.get("USERNAME").unwrap(),
            fullname: row.get("FULLNAME").unwrap(),
            email: row.get("EMAIL").unwrap(),
            login_duration: row.get("LOGINDURATION").unwrap(),
        };
    }
    if user.is_empty() {
        error!("User not found");
        return Err(APIErrors::UserNotFound);
    } else {
        Ok(user)
    }
}

pub async fn create_user(data: NewUser, sql_manager: &SQLManager, pool: &Pool) -> Result<(), APIErrors> {
    let conn = pool.get();
    if conn.is_err() {
        error!("Error Connecting to DB");
        return Err(APIErrors::DBError);
    }
    let conn = conn.unwrap();

    // Check if user exists with same username
    match check_user_exists(data.p_username.clone(), &pool, &sql_manager).await {
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
        .statement(sql_manager.get_sql("create_user")?.as_str())
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

pub async fn edit_user(
    params: EditUserParams,
    username: &str,
    pool: &Pool,
    sql_manager: &SQLManager,
    is_admin: bool,
) -> Result<(), APIErrors> {
    let params_unwrapped = params;

    let original_user = match get_user(&username, &sql_manager, pool).await {
        Ok(user) => user,
        Err(_) => {
            error!("Error getting user");
            return Err(APIErrors::DBError);
        }
    };
    
    // Weird bug where threads errors are shown when using sql_manager.get_sql, the line position also seems to matter
    let pass_stmt = sql_manager.get_sql("update_user_password");

    if original_user.username == "" {
        error!("User not found");
        return Err(APIErrors::UserNotFound);
    }

    let mut new_user = original_user.clone();

    if params_unwrapped.p_fullname.is_some() {
        new_user.fullname = params_unwrapped.p_fullname;
    }

    if params_unwrapped.p_email.is_some() {
        new_user.email = params_unwrapped.p_email;
    }

    if params_unwrapped.p_loginduration.is_some() {
        new_user.login_duration = params_unwrapped.p_loginduration;
    }

    let conn = pool.get();
    if conn.is_err() {
        error!("Error connecting to DB");
        return Err(APIErrors::DBError);
    }
    let conn = conn.unwrap();

    let stmt = conn
        .statement(sql_manager.get_sql("update_user")?.as_str())
        .build();
    if stmt.is_err() {
        error!("Error building statement");
        return Err(APIErrors::DBError);
    }
    let mut stmt = stmt.unwrap();

    let _ = stmt.execute(&[
        &new_user.fullname,
        &new_user.email,
        &new_user.login_duration,
        &new_user.username,
    ]).unwrap(); 

    match conn.commit() {
        Ok(_) => (),
        Err(err) => {
            error!("Error commiting to DB: {}", err);
            return Err(APIErrors::DBError);
        }
    }

    if params_unwrapped.p_password.is_some() && is_admin {
        match conn
            .statement(pass_stmt.unwrap().as_str())
            .build()
        {
            Ok(data) => stmt = data,
            Err(err) => {
                error!("Error building statement: {}", err);
                return Err(APIErrors::DBError);
            }
        }

        let _ = stmt.execute(&[
            &hash(params_unwrapped.p_password.unwrap(), DEFAULT_COST).unwrap(),
            &new_user.username,
        ]).unwrap();

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

pub async fn delete_user(user_id: &str, sql_manager: &SQLManager, pool: &Pool) -> Result<(), APIErrors> {
    match check_user_exists(user_id.to_string(), &pool, &sql_manager).await {
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

    let delete_stmt = conn.statement(sql_manager.get_sql("delete_user_permissions")?.as_str()).build();
    if delete_stmt.is_err() {
        error!("Error building statement");
        return Err(APIErrors::DBError);
    }
    let mut delete_stmt = delete_stmt.unwrap();

    match delete_stmt.execute(&[&(user_id.to_lowercase())]) {
        Ok(_) => println!("Deleted user permissions"),
        Err(err) => {
            error!("Error executing delete: {}", err);
            return Err(APIErrors::DBError);
        }
    }

    let delete_stmt = conn.statement(sql_manager.get_sql("delete_user_stores")?.as_str()).build();
    if delete_stmt.is_err() {
        error!("Error building statement");
        return Err(APIErrors::DBError);
    }
    let mut delete_stmt = delete_stmt.unwrap();

    match delete_stmt.execute(&[&(user_id.to_lowercase())]) {
        Ok(_) => println!("Deleted user stores"),
        Err(err) => {
            error!("Error executing delete: {}", err);
            return Err(APIErrors::DBError);
        }
    }

    let stmt = conn
        .statement(sql_manager.get_sql("delete_user")?.as_str())
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
