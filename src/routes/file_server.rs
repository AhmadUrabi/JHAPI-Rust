use crate::LogCheck;
use crate::signing::decode_token_data;
use crate::utils::permissions::is_admin_perm;
use crate::utils::permissions::is_images_perm;
use crate::utils::permissions::is_query_perm;
use crate::ApiKey;
use oracle::pool::Pool;
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::{get, State};

use crate::file_server::download_file;
use crate::file_server::upload_file;

use std::net::IpAddr;

use crate::utils::logging::{getTimestamp, log_data};

use std::path::*;

#[get("/images/<file..>")]
pub async fn get_image(
    file: PathBuf,
    _key: ApiKey<'_>,
    pool: &State<Pool>,
    client_ip: Option<IpAddr>,
    log_check: LogCheck,
) -> Result<Option<NamedFile>, Status> {
    let mut userId: String = "".to_string();
    match decode_token_data(_key.0) {
        Some(data) => {
            info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap());
            userId = data.USER_ID.unwrap();
        }
        None => info!("Token Data: None"),
    }
    let userCopy = userId.clone();

    if !is_query_perm(&_key, pool) && !is_admin_perm(&_key, pool) {
        if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
        log_data(
            pool,
            userId,
            client_ip.unwrap().to_string(),
            ("/images/".to_owned() + file.to_str().unwrap()).to_string(),
            None,
            getTimestamp(),
            _key.0.to_string(),
            "Unauthorized".to_string(),
            "GET".to_string()
        );
    }
        return Err(Status::Unauthorized);
    }
    info!("Image Request: {:?}", file);

    let fileName = file.to_str().unwrap().to_string();

    if download_file(&fileName).await {
        info!("File Downloaded");
    } else {
        info!("File Not Found");
        if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
        log_data(
            pool,
            userId,
            client_ip.unwrap().to_string(),
            ("/images/".to_owned() + file.to_str().unwrap()).to_string(),
            None,
            getTimestamp(),
            _key.0.to_string(),
            "File Not Found".to_string(),
            "GET".to_string()
        );
    }
        Err(Status::NotFound)?;
    };
    if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
    log_data(
        pool,
        userCopy,
        client_ip.unwrap().to_string(),
        ("/images/".to_owned() + file.to_str().unwrap()).to_string(),
        None,
        getTimestamp(),
        _key.0.to_string(),
        "Success".to_string(),
        "GET".to_string()
    );
}
    Ok(NamedFile::open(Path::new("tmp/tmpdownload.jpg")).await.ok())
}

use rocket::form::Form;
use rocket::fs::TempFile;

#[derive(FromForm)]
pub struct ImageUpload<'f> {
    pub file: TempFile<'f>,
    pub item_code: String,
}

#[post("/upload", data = "<params>")]
pub async fn upload(
    mut params: Form<ImageUpload<'_>>,
    _key: ApiKey<'_>,
    pool: &State<Pool>,
    client_ip: Option<IpAddr>,
    log_check: LogCheck,
) -> Result<String, Status> {
    let mut userId: String = "".to_string();
    match decode_token_data(_key.0) {
        Some(data) => {
            info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap());
            userId = data.USER_ID.unwrap();
        }
        None => info!("Token Data: None"),
    }
    let userCopy = userId.clone();

    if !is_images_perm(&_key, pool) && !is_admin_perm(&_key, pool) {
        if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
        log_data(
            pool,
            userId,
            client_ip.unwrap().to_string(),
            "/upload".to_string(),
            Some(params.item_code.clone()),
            getTimestamp(),
            _key.0.to_string(),
            "Unauthorized".to_string(),
            "POST".to_string()
        );
    }
        return Err(Status::Unauthorized);
    }

    info!("Image Upload Request: {:?}", params.item_code);

    // Save file temporarily
    let fileName = "tmp/".to_string() + params.file.name().unwrap();
    params.file.persist_to(&fileName).await.unwrap();

    // Upload file to server
    if upload_file(&params.item_code, &fileName).await {
        info!("File Uploaded");
    } else {
        info!("File Not Uploaded");
        log_data(
            pool,
            userId,
            client_ip.unwrap().to_string(),
            "/upload".to_string(),
            Some(params.item_code.clone()),
            getTimestamp(),
            _key.0.to_string(),
            "File Not Uploaded".to_string(),
            "POST".to_string()
        );
        Err(Status::NotFound)?;
    }

    // Delete temporary file
    std::fs::remove_file(fileName).unwrap();

    log_data(
        pool,
        userCopy,
        client_ip.unwrap().to_string(),
        "/upload".to_string(),
        Some(params.item_code.clone()),
        getTimestamp(),
        _key.0.to_string(),
        "Success".to_string(),
        "POST".to_string()
    );

    Ok("File Uploaded".to_string())
}
