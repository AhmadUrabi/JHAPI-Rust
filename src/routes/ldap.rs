use crate::{
    controllers::ldap::{
        check_connection,
        models::{UserAccount, UserParams},
    },
    server::JHApiServerState,
    utils::structs::APIErrors,
};
use ldap3::{Scope, SearchEntry};
use rocket::{serde::json::Json, Route, State};

pub fn routes() -> Vec<Route> {
    routes![get_all_users, get_all_computers, create_user, delete_user,]
}

#[get("/ldap/users")]
pub async fn get_all_users(
    state: &State<JHApiServerState>,
) -> Result<Json<Vec<UserAccount>>, String> {
    // if let Err(_) = check_connection(&state).await {
    //     loop {
    //         if let Ok(_) = check_connection(&state).await {
    //             break;
    //         }
    //     }
    // }
    let mut ldap = state.ldap.lock().await;
    let users = UserAccount::fetch_all_users(&mut ldap).await;
    // ApiResponse::new("Success".to_string(), rocket::http::Status::Ok, Some(users))
    Ok(Json(users))
}

#[get("/ldap/computers")]
pub async fn get_all_computers(
    state: &State<JHApiServerState>,
) -> Result<Json<Vec<String>>, String> {
    if let Err(_) = check_connection(&state).await {
        loop {
            if let Ok(_) = check_connection(&state).await {
                break;
            }
        }
    }
    let mut ldap = state.ldap.lock().await;
    // let users = UserAccount::fetch_all_users(&mut ldap).await;

    let base_dn_string = std::env::var("BASE_DN").unwrap();
    let base_dn = base_dn_string.as_str();
    // Perform a search
    let (rs, _res) = ldap
        .search(
            base_dn,
            Scope::Subtree,
            "(objectClass=computer)",
            vec!["*", "+"],
        )
        .await
        .unwrap()
        .success()
        .unwrap();

    println!("{:?}", rs);

    // Iterate through search results and print them
    for entry in rs {
        let entry = SearchEntry::construct(entry);
        println!("{:?}", entry);
        // let user: UserAccount = entry.attrs.into();
        // res.push(user);
    }

    Ok(Json(vec![]))
}

#[post("/ldap/users", format = "json", data = "<user>")]
pub async fn create_user(
    user: Json<UserParams>,
    state: &State<JHApiServerState>,
) -> Result<Json<UserAccount>, String> {
    let mut ldap = state.ldap.lock().await;
    let user_data = user.into_inner();
    let new_user = UserAccount::create_new_user(&mut ldap, user_data).await;
    match new_user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err("Error Creating User".to_string()),
    }
    // match new_user {
    //     Ok(user) => Ok(user),
    //     Err(e) => Err(e),
    //     // errors::APIErrors::EntryExists => ApiResponse::new(
    //     //     "User Already Exists".to_string(),
    //     //     rocket::http::Status::Conflict,
    //     //     None,
    //     // ),
    //     // _ => ApiResponse::new(
    //     //     "Error Creating User".to_string(),
    //     //     rocket::http::Status::InternalServerError,
    //     //     None,
    //     // ),
    // }
}

#[delete("/ldap/users/<uname>")]
pub async fn delete_user(uname: String, state: &State<JHApiServerState>) -> Result<(), String> {
    let mut ldap = state.ldap.lock().await;
    let user_dn = UserAccount::get_dn_from_uname(&mut ldap, uname.as_str()).await;
    if user_dn.is_none() {
        return Err("User Not Found".to_string());
    }

    let user_dn = user_dn.unwrap();
    println!("Deleting user: {}", user_dn);

    let res = ldap.delete(user_dn.as_str()).await;
    if res.is_err() {
        return Err("Error Deleting User".to_string());
    }
    match res.unwrap().success() {
        Ok(_) => Ok(()),
        Err(_) => Err("Error Deleting User".to_string()),
    }
}
