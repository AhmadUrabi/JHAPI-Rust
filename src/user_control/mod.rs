use crate::ApiKey;
use crate::signing::decode_token_data;
use oracle::pool::Pool;
use oracle::Result;
use rocket::serde::json::Json;

use crate::permissions::get_user_permissions;


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub username: String,
    pub fullname: String,
    pub email: String,
    pub login_duration: i32,
}

pub async fn get_users(_key: ApiKey<'_>, pool: &Pool) -> Result<Vec<User>> {
    let user_id = decode_token_data(_key.0).unwrap().USER_ID.unwrap();
    let permissions: Vec<String> = get_user_permissions(&user_id, pool).unwrap();
    println!("Permissions: {:?}", permissions);
    let mut users: Vec<User> = Vec::new();
    if permissions.contains(&"admin".to_string()){
        println!("Admin Permissions Found");
        let conn = pool.get().unwrap();
        
        let mut stmt = conn
            .statement("SELECT USERNAME, FULLNAME, EMAIL, LOGINDURATION FROM ODBC_JHC.AUTHENTICATION_JHC")
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
    let rows = stmt.query(&[&user_id]).unwrap();
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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub fullname: String,
    pub email: String,
    pub login_duration: i32,
}

pub async fn create_user(data: NewUser, pool: &Pool) -> Result<()> {
    let conn = pool.get().unwrap();
    let mut stmt = conn
        .statement("INSERT INTO ODBC_JHC.AUTHENTICATION_JHC (USERNAME, PASSWORD, FULLNAME, EMAIL, LOGINDURATION) VALUES (:1, :2, :3, :4, :5)")
        .build()?;
    stmt.execute(&[&data.username, &data.password, &data.fullname, &data.email, &data.login_duration])
        .unwrap();
    conn.commit()?;
    Ok(())
}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EditUserParams {
    pub username: Option<String>,
    pub password: Option<String>,
    pub fullname: Option<String>,
    pub email: Option<String>,
    pub loginduration: Option<i32>,
}

pub async fn edit_user(params: Json<EditUserParams>, pool: &Pool) -> Result<bool> {

    let paramsUnwrapped = params.into_inner();

    
    let original_user = get_user(&paramsUnwrapped.username.unwrap(), pool).await.unwrap();

    if original_user.username == "" {
        return Ok(false);
    }

    let mut new_user = User {
        username: original_user.username,
        fullname: original_user.fullname,
        email: original_user.email,
        login_duration: original_user.login_duration,
    };

    if paramsUnwrapped.fullname.is_some() {
        new_user.fullname = paramsUnwrapped.fullname.unwrap();
    }

    if paramsUnwrapped.email.is_some() {
        new_user.email = paramsUnwrapped.email.unwrap();
    }

    if paramsUnwrapped.loginduration.is_some() {
        new_user.login_duration = paramsUnwrapped.loginduration.unwrap();
    }

    let conn = pool.get().unwrap();
    let mut stmt = conn
        .statement("UPDATE ODBC_JHC.AUTHENTICATION_JHC SET FULLNAME = :1, EMAIL = :2, LOGINDURATION = :3 WHERE USERNAME = :4")
        .build()?;
    stmt.execute(&[&new_user.fullname, &new_user.email, &new_user.login_duration, &new_user.username])
        .unwrap();
    conn.commit()?;
    Ok(true)
}


pub async fn delete_user(user_id: &str, pool: &Pool) -> Result<()> {
    let conn = pool.get().unwrap();
    let mut stmt = conn
        .statement("DELETE FROM ODBC_JHC.AUTHENTICATION_JHC WHERE USERNAME = :1")
        .build()?;
    stmt.execute(&[&user_id])
        .unwrap();
    conn.commit()?;
    Ok(())
}