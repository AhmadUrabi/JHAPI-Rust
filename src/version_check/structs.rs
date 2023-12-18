
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Version {
    pub version: String,
    pub platform: String,
    pub url: String,
    pub release_date: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct VersionParams {
    pub p_current_version: String,
    pub p_platform: String,
}