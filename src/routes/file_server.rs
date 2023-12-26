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

use crate::utils::structs::APIErrors;

use std::net::IpAddr;

use crate::utils::logging::{get_timestamp, log_data};

use std::path::*;


// TODO: Rework temporary file storage
#[get("/images/<file..>")]
pub async fn get_image(
    file: PathBuf,
    _key: ApiKey<'_>,
    pool: &State<Pool>,
    client_ip: Option<IpAddr>,
    log_check: LogCheck,
) -> Result<Option<NamedFile>, Status> {
    // TODO: Fix these temp user_id strings
    let mut user_id: String = "".to_string();
    match decode_token_data(_key.0) {
        Some(data) => {
            info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap());
            user_id = data.USER_ID.unwrap();
        }
        None => info!("Token Data: None"),
    }
    let user_copy = user_id.clone();

    if !is_query_perm(&_key, pool) && !is_admin_perm(&_key, pool) {
        if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
        log_data(
            pool,
            user_id,
            client_ip.unwrap().to_string(),
            ("/images/".to_owned() + file.to_str().unwrap()).to_string(),
            None,
            get_timestamp(),
            _key.0.to_string(),
            "Unauthorized".to_string(),
            "GET".to_string()
        );
    }
        return Err(Status::Unauthorized);
    }
    info!("Image Request: {:?}", file);

    let filename = file.to_str().unwrap().to_string();

    if filename == "" {
        if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
            log_data(
                pool,
                user_id,
                client_ip.unwrap().to_string(),
                ("/images/".to_owned() + file.to_str().unwrap()).to_string(),
                None,
                get_timestamp(),
                _key.0.to_string(),
                "File Not Found".to_string(),
                "GET".to_string()
            );
        }
        return Err(Status::NotFound);
    }

    match download_file(&filename).await {
        Ok (()) => info!("File Downloaded"),
        Err(e) => {
        info!("File Not Found");
                if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
                log_data(
                    pool,
                    user_id,
                    client_ip.unwrap().to_string(),
                    ("/images/".to_owned() + file.to_str().unwrap()).to_string(),
                    None,
                    get_timestamp(),
                    _key.0.to_string(),
                    match e {
                        APIErrors::SFTPError => "SFTP Error".to_string(),
                        APIErrors::FileNotFound => "File Not Found".to_string(),
                        _ => "Error".to_string(),
                    },
                    "GET".to_string()
                );
            }
            match e {
                APIErrors::SFTPError => return Err(Status::InternalServerError),
                APIErrors::FileNotFound => return Err(Status::NotFound),
                _ => return Err(Status::InternalServerError),
            }
        }
    };
    if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
    log_data(
        pool,
        user_copy,
        client_ip.unwrap().to_string(),
        ("/images/".to_owned() + file.to_str().unwrap()).to_string(),
        None,
        get_timestamp(),
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
// TODO: Add Error Handling
#[post("/upload", data = "<params>")]
pub async fn upload(
    mut params: Form<ImageUpload<'_>>,
    #[allow(non_snake_case)] // Keeps giving warnings about _key not being snake_case
    _key: ApiKey<'_>,
    pool: &State<Pool>,
    client_ip: Option<IpAddr>,
    log_check: LogCheck,
) -> Result<String, Status> {
    let mut user_id: String = "".to_string();
    match decode_token_data(_key.0) {
        Some(data) => {
            info!("Token User Id: {:?}", data.USER_ID.as_ref().unwrap());
            user_id = data.USER_ID.unwrap();
        }
        None => info!("Token Data: None"),
    }
    let user_copy = user_id.clone();

    if !is_images_perm(&_key, pool) && !is_admin_perm(&_key, pool) {
        if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
        log_data(
            pool,
            user_id,
            client_ip.unwrap().to_string(),
            "/upload".to_string(),
            Some(params.item_code.clone()),
            get_timestamp(),
            _key.0.to_string(),
            "Unauthorized".to_string(),
            "POST".to_string()
        );
    }
        return Err(Status::Unauthorized);
    }

    info!("Image Upload Request: {:?}", params.item_code);

    // Save file temporarily
    
    if params.file.name().is_none() || params.item_code == "" {
        if log_check.0 || (!log_check.0 && !is_admin_perm(&_key, pool)){
        log_data(
            pool,
            user_id,
            client_ip.unwrap().to_string(),
            "/upload".to_string(),
            Some(params.item_code.clone()),
            get_timestamp(),
            _key.0.to_string(),
            "No File Uploaded".to_string(),
            "POST".to_string()
        );
    }
        return Err(Status::BadRequest);
    }
    let filename = "tmp/".to_string() + params.file.name().unwrap();
    params.file.persist_to(&filename).await.unwrap();

    // Upload file to server
    match upload_file(&params.item_code, &filename).await {
        Ok(()) => info!("File Uploaded"),
        Err(e) => {
        info!("File Not Uploaded");
        log_data(
            pool,
            user_id,
            client_ip.unwrap().to_string(),
            "/upload".to_string(),
            Some(params.item_code.clone()),
            get_timestamp(),
            _key.0.to_string(),
            match e {
                APIErrors::SFTPError => "SFTP Error".to_string(),
                APIErrors::FileNotFound => "File Not Found".to_string(),
                _ => "Error".to_string(),
            },
            "POST".to_string()
        );
            match e {
                APIErrors::SFTPError => return Err(Status::InternalServerError),
                APIErrors::FileNotFound => return Err(Status::NotFound),
                _ => return Err(Status::InternalServerError),
            }
    }
    }

    // Delete temporary file
    std::fs::remove_file(filename).unwrap();

    log_data(
        pool,
        user_copy,
        client_ip.unwrap().to_string(),
        "/upload".to_string(),
        Some(params.item_code.clone()),
        get_timestamp(),
        _key.0.to_string(),
        "Success".to_string(),
        "POST".to_string()
    );

    Ok("File Uploaded".to_string())
}
