use std::collections::{HashMap, HashSet};

use base64::Engine;
use ldap3::{Ldap, Mod, Scope, SearchEntry};
use models::{UserAccount, UserParams};

use crate::{
    server::{JHApiServer, JHApiServerState},
    utils::structs::APIErrors,
};

pub mod models;

pub async fn check_connection(state: &JHApiServerState) -> Result<(), String> {
    let mut ldap = state.ldap.lock().await;
    let username = std::env::var("LOGIN_USERNAME").unwrap();
    let password = std::env::var("LOGIN_PASSWORD").unwrap();

    match ldap.simple_bind(&username, &password).await {
        Ok(_) => Ok(()),
        Err(_) => {
            *ldap = JHApiServer::create_ldap_connection().await;
            Ok(())
        }
    }
}

impl UserAccount {
    pub fn default() -> Self {
        UserAccount {
            sAMAccountName: None,
            sn: None,
            badPasswordTime: None,
            uSNChanged: None,
            objectClass: None,
            logonCount: None,
            homeDirectory: None,
            accountExpires: None,
            lastLogonTimestamp: None,
            lastLogoff: None,
            distinguishedName: None,
            countryCode: None,
            objectCategory: None,
            cn: None,
            codePage: None,
            memberOf: None,
            instanceType: None,
            name: None,
            givenName: None,
            sAMAccountType: None,
            userPrincipalName: None,
            whenChanged: None,
            pwdLastSet: None,
            badPwdCount: None,
            lastLogon: None,
            whenCreated: None,
            displayName: None,
            homeDrive: None,
            userAccountControl: None,
            primaryGroupID: None,
            uSNCreated: None,
            dSCorePropagationData: None,
        }
    }

    pub async fn fetch_all_users(ldap: &mut Ldap) -> Vec<UserAccount> {
        let base_dn_string = std::env::var("BASE_DN").unwrap();
        let base_dn = base_dn_string.as_str();
        // Perform a search
        let (rs, _res) = ldap
            .search(
                base_dn,
                Scope::Subtree,
                "(objectClass=user)",
                vec!["*", "+"],
            )
            .await
            .unwrap()
            .success()
            .unwrap();

        let mut res = Vec::new();

        // Iterate through search results and print them
        for entry in rs {
            let entry = SearchEntry::construct(entry);
            let user: UserAccount = entry.attrs.into();
            res.push(user);
        }
        res
    }

    pub async fn create_new_user(
        ldap: &mut Ldap,
        user: UserParams,
    ) -> Result<UserAccount, APIErrors> {
        let binding = format!("CN={},{}", user.cn, std::env::var("BASE_DN").unwrap()).to_owned();
        let new_user_dn = binding.as_str();

        // Lookup the userPrincipalName to see if it already exists
        let user_exists = Self::get_dn_from_uname(ldap, user.userPrincipalName.as_str()).await;
        if user_exists.is_some() {
            return Err(APIErrors::UserExists);
        }

        let quoted_b64_password = format!("'{}'", user.password);

        let new_password_utf16: Vec<u16> = quoted_b64_password.encode_utf16().collect();
        let new_password_bytes: Vec<u8> = new_password_utf16
            .iter()
            .flat_map(|&c| c.to_le_bytes())
            .collect();
        let b64_password = base64::engine::general_purpose::STANDARD.encode(&new_password_bytes);

        let mut password_utf16: HashSet<&[u8]> = HashSet::new();
        password_utf16.insert(&b64_password.as_bytes());

        let new_user_attrs = vec![
            (
                "objectClass",
                ["top", "person", "organizationalPerson", "user"]
                    .iter()
                    .cloned()
                    .collect::<HashSet<_>>(),
            ), // Object Class
            (
                "cn",
                [user.cn.as_str()].iter().cloned().collect::<HashSet<_>>(),
            ), // Common Name
            (
                "givenName",
                [user.givenName.as_str()]
                    .iter()
                    .cloned()
                    .collect::<HashSet<_>>(),
            ), // First Name
            (
                "sn",
                [user.sn.as_str()].iter().cloned().collect::<HashSet<_>>(),
            ), // Surname
            (
                "displayName",
                [user.givenName.as_str()]
                    .iter()
                    .cloned()
                    .collect::<HashSet<_>>(),
            ), // Display Name
            (
                "userPrincipalName",
                [user.userPrincipalName.as_str()]
                    .iter()
                    .cloned()
                    .collect::<HashSet<_>>(),
            ), // User Logon Name
            (
                "sAMAccountName",
                [user.sAMAccountName.as_str()]
                    .iter()
                    .cloned()
                    .collect::<HashSet<_>>(),
            ), // User Logon Name
            (
                "mail",
                [user.mail.as_str()].iter().cloned().collect::<HashSet<_>>(),
            ), // Internal Mail
        ];
        let res = ldap.add(new_user_dn, new_user_attrs).await;
        if res.is_err() {
            return Err(APIErrors::InternalServerError);
        }

        let _ = Self::set_password(ldap, new_user_dn, &user.password).await;
        let _ = Self::update_user_account_control(ldap, new_user_dn, 66048).await;

        println!("{:?}", user.create_cpanel_account);

        if user.create_cpanel_account.unwrap_or(false) {
            let cpanel_user = user.userPrincipalName.split('@').next().unwrap();
            let res = create_cpanel_account(cpanel_user.to_string(), "jh.com.jo".to_string()).await;
            println!("Cpanel account created: {:?}", res);
        }

        match Self::fetch_user(ldap, new_user_dn).await {
            Some(user) => Ok(user),
            None => Err(APIErrors::UserNotFound),
        }
    }

    pub async fn fetch_user(ldap: &mut Ldap, dn: &str) -> Option<UserAccount> {
        let (rs, _res) = ldap
            .search(dn, ldap3::Scope::Base, "(objectClass=user)", vec!["*", "+"])
            .await
            .ok()?
            .success()
            .ok()?; // Get the search result

        let entry = rs.into_iter().next()?;

        let entry = ldap3::SearchEntry::construct(entry);

        Some(entry.attrs.into())
    }

    async fn set_password(
        conn: &mut ldap3::Ldap,
        user_dn: &str,
        new_password: &str,
    ) -> Result<(), ldap3::LdapError> {
        // Encode the password
        let attr_name = String::from("unicodePwd").into_bytes();
        let values: Vec<u8> = format!("\"{}\"", new_password)
            .encode_utf16()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        let mut passwd = HashSet::new();
        passwd.insert(values);
        let mods = vec![Mod::Replace(attr_name, passwd)];
        let result = conn.modify(&user_dn, mods).await.unwrap();
        println!("Set password result: {:?}", result);
        Ok(())
    }

    async fn update_user_account_control(
        conn: &mut ldap3::Ldap,
        user_dn: &str,
        flag: u32,
    ) -> Result<(), ldap3::LdapError> {
        // First, get the current userAccountControl value
        let res = conn
            .modify(
                user_dn,
                vec![ldap3::Mod::Replace(
                    "userAccountControl",
                    HashSet::from([flag.to_string().as_str()]),
                )],
            )
            .await;
        println!("Update user account control result: {:?}", res);
        Ok(())
    }

    pub async fn get_dn_from_uname(ldap: &mut Ldap, uname: &str) -> Option<String> {
        let filter = format!(
            "(&(objectCategory=person)(objectClass=user)(userPrincipalName={}))",
            uname
        );

        let base_dn_string = std::env::var("BASE_DN").unwrap();
        let base_dn = base_dn_string.as_str();
        // Perform a search
        let (rs, _res) = ldap
            .search(base_dn, Scope::Subtree, &filter, vec!["distinguishedName"])
            .await
            .unwrap()
            .success()
            .unwrap();

        if rs.is_empty() {
            return None;
        }

        let res = rs[0].clone();
        let entry = SearchEntry::construct(res);
        let dn = entry.attrs.get("distinguishedName").unwrap();
        Some(dn[0].clone())
    }
}

impl From<HashMap<String, Vec<String>>> for UserAccount {
    fn from(attrs: HashMap<String, Vec<String>>) -> Self {
        Self {
            sAMAccountName: attrs.get("sAMAccountName").cloned(),
            sn: attrs.get("sn").cloned(),
            badPasswordTime: attrs.get("badPasswordTime").cloned(),
            uSNChanged: attrs.get("uSNChanged").cloned(),
            objectClass: attrs.get("objectClass").cloned(),
            logonCount: attrs.get("logonCount").cloned(),
            homeDirectory: attrs.get("homeDirectory").cloned(),
            accountExpires: attrs.get("accountExpires").cloned(),
            lastLogonTimestamp: attrs.get("lastLogonTimestamp").cloned(),
            lastLogoff: attrs.get("lastLogoff").cloned(),
            distinguishedName: attrs.get("distinguishedName").cloned(),
            countryCode: attrs.get("countryCode").cloned(),
            objectCategory: attrs.get("objectCategory").cloned(),
            cn: attrs.get("cn").cloned(),
            codePage: attrs.get("codePage").cloned(),
            memberOf: attrs.get("memberOf").cloned(),
            instanceType: attrs.get("instanceType").cloned(),
            name: attrs.get("name").cloned(),
            givenName: attrs.get("givenName").cloned(),
            sAMAccountType: attrs.get("sAMAccountType").cloned(),
            userPrincipalName: attrs.get("userPrincipalName").cloned(),
            whenChanged: attrs.get("whenChanged").cloned(),
            pwdLastSet: attrs.get("pwdLastSet").cloned(),
            badPwdCount: attrs.get("badPwdCount").cloned(),
            lastLogon: attrs.get("lastLogon").cloned(),
            whenCreated: attrs.get("whenCreated").cloned(),
            displayName: attrs.get("displayName").cloned(),
            homeDrive: attrs.get("homeDrive").cloned(),
            userAccountControl: attrs.get("userAccountControl").cloned(),
            primaryGroupID: attrs.get("primaryGroupID").cloned(),
            uSNCreated: attrs.get("uSNCreated").cloned(),
            dSCorePropagationData: attrs.get("dSCorePropagationData").cloned(),
        }
    }
}

async fn create_cpanel_account(user: String, domain: String) -> Result<String, reqwest::Error> {
    let cpanel_url = std::env::var("CPANEL_URL").unwrap();
    let user_password = std::env::var("CPANEL_PASSWORD").unwrap();
    let access_token = std::env::var("CPANEL_ACCESS_TOKEN").unwrap();
    let client = reqwest::Client::new();
    let res = client
        .get(format!(
            "{}/execute/Email/add_pop?email={}&password={}&domain={}&quota=2048",
            cpanel_url, user, user_password, domain
        ))
        .header("Authorization", access_token)
        .send()
        .await?;
    let body = res.text().await?;
    let json = serde_json::from_str::<serde_json::Value>(&body).unwrap();
    Ok(json["data"].as_str().unwrap_or("").to_string())
}
