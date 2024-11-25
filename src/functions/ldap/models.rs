#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserParams {
    pub cn: String,
    pub givenName: String,
    pub sn: String,
    pub displayName: String,
    pub userPrincipalName: String,
    pub sAMAccountName: String,
    pub mail: String,
    pub password: String,
    pub create_cpanel_account: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAccount {
    pub sAMAccountName: Option<Vec<String>>,
    pub sn: Option<Vec<String>>,
    pub badPasswordTime: Option<Vec<String>>,
    pub uSNChanged: Option<Vec<String>>,
    pub objectClass: Option<Vec<String>>,
    pub logonCount: Option<Vec<String>>,
    pub homeDirectory: Option<Vec<String>>,
    pub accountExpires: Option<Vec<String>>,
    pub lastLogonTimestamp: Option<Vec<String>>,
    pub lastLogoff: Option<Vec<String>>,
    pub distinguishedName: Option<Vec<String>>,
    pub countryCode: Option<Vec<String>>,
    pub objectCategory: Option<Vec<String>>,
    pub cn: Option<Vec<String>>,
    pub codePage: Option<Vec<String>>,
    pub memberOf: Option<Vec<String>>,
    pub instanceType: Option<Vec<String>>,
    pub name: Option<Vec<String>>,
    pub givenName: Option<Vec<String>>,
    pub sAMAccountType: Option<Vec<String>>,
    pub userPrincipalName: Option<Vec<String>>,
    pub whenChanged: Option<Vec<String>>,
    pub pwdLastSet: Option<Vec<String>>,
    pub badPwdCount: Option<Vec<String>>,
    pub lastLogon: Option<Vec<String>>,
    pub whenCreated: Option<Vec<String>>,
    pub displayName: Option<Vec<String>>,
    pub homeDrive: Option<Vec<String>>,
    pub userAccountControl: Option<Vec<String>>,
    pub primaryGroupID: Option<Vec<String>>,
    pub uSNCreated: Option<Vec<String>>,
    pub dSCorePropagationData: Option<Vec<String>>,
}
