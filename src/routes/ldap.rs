use crate::{
    controllers::ldap::{
        check_connection,
        UserAccount, UserParams,
    },
    respond,
    server::{response::ApiResponse, JHApiServerState},
};
use ldap3::{Scope, SearchEntry};
use rocket::{serde::json::Json, Route, State};
use tokio::time::timeout;

const TIMEOUT: core::time::Duration = core::time::Duration::from_secs(10);

pub fn routes() -> Vec<Route> {
    routes![get_all_users, get_all_computers, create_user, delete_user,]
}

// TODO: Clean all functionality

async fn refresh_connection(state: &State<JHApiServerState>) {
    let mut retries = 5;
    while retries > 0 {
        if let Ok(Ok(_)) = timeout(TIMEOUT, check_connection(&state)).await {
            println!("Connection established!");
            break;
        } else {
            retries -= 1;
            println!("Retrying... Remaining attempts: {}", retries);
        }
    }
}

#[get("/ldap/users")]
pub async fn get_all_users(state: &State<JHApiServerState>) -> ApiResponse {
    refresh_connection(&state).await;
    let mut ldap = state.ldap.lock().await;
    let users = UserAccount::fetch_all_users(&mut ldap).await;
    respond!(200, "Users Found", users)
}

#[get("/ldap/computers")]
pub async fn get_all_computers(state: &State<JHApiServerState>) -> ApiResponse {
    refresh_connection(&state).await;
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

    // TODO: Clean up and error handling

    let mut res: Vec<UserAccount> = vec![];

    // Iterate through search results and print them
    for entry in rs {
        let entry = SearchEntry::construct(entry);
        let user: UserAccount = entry.attrs.into();
        res.push(user);
    }

    respond!(200, "Users Found", res)
}

#[post("/ldap/users", format = "json", data = "<user>")]
pub async fn create_user(user: Json<UserParams>, state: &State<JHApiServerState>) -> ApiResponse {
    refresh_connection(&state).await;
    let mut ldap = state.ldap.lock().await;
    let user_data = user.into_inner();
    let new_user = UserAccount::create_new_user(&mut ldap, user_data).await;
    match new_user {
        Ok(user) => respond!(201, "User Created", user),
        // TODO: Add handling for errors
        Err(_) => respond!(422, "Couldn't Create User"),
    }
}

#[delete("/ldap/users/<uname>")]
pub async fn delete_user(uname: String, state: &State<JHApiServerState>) -> ApiResponse {
    refresh_connection(&state).await;
    let mut ldap = state.ldap.lock().await;
    let user_dn = UserAccount::get_dn_from_uname(&mut ldap, uname.as_str()).await;
    if user_dn.is_none() {
        return respond!(404, "User Not Found");
    }

    let user_dn = user_dn.unwrap();

    let res = ldap.delete(user_dn.as_str()).await;
    if res.is_err() {
        return respond!(500, "Error Deleting User");
    }
    // TODO: Error Handling
    match res.unwrap().success() {
        Ok(_) => respond!(200, "User Deleted"),
        Err(_) => respond!(500, "Error Deleting User"),
    }
}
