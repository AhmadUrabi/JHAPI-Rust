use oracle::pool::Pool;
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::{get, State};
use crate::utils::permissions::is_images_perm;
use crate::utils::permissions::is_query_perm;
use crate::utils::permissions::is_admin_perm;
use crate::ApiKey;

use crate::file_server::download_file;
use crate::file_server::upload_file;


use std::path::*;

#[get("/images/<file..>")]
pub async fn get_image(
    file: PathBuf,
    _key: ApiKey<'_>,
    pool: &State<Pool>,
) -> Result<Option<NamedFile>, Status> {
    if !is_query_perm(&_key, pool) && !is_admin_perm(&_key, pool) {
        return Err(Status::Unauthorized);
    }
    info!("Image Request: {:?}", file);

    let fileName = file.to_str().unwrap().to_string();

    if download_file(&fileName).await {
        info!("File Downloaded");
    } else {
        info!("File Not Found");
        Err(Status::NotFound)?;
    };

    
    Ok(NamedFile::open(Path::new("tmp/tmpdownload.jpg")).await.ok())
}

use rocket::fs::TempFile;
use rocket::form::Form;

#[derive(FromForm)]
pub struct ImageUpload<'f> {
    pub file: TempFile<'f>,
    pub item_code: String,
}

#[post("/upload", data = "<params>")]
pub async fn upload(mut params: Form<ImageUpload<'_>>,_key: ApiKey<'_>, pool:&State<Pool>) -> Result<String, Status> {
    
    if !is_images_perm(&_key, pool) && !is_admin_perm(&_key, pool) {
        return Err(Status::Unauthorized);
    }

    info!("Image Upload Request: {:?}", params.item_code);

    // Save file temporarily
    params.file.persist_to("tmp/test.jpg").await.unwrap();

    // Upload file to server
    if upload_file(&params.item_code).await {
        info!("File Uploaded");
    } else {
        info!("File Not Uploaded");
        Err(Status::NotFound)?;
    }

    // Delete temporary file
    std::fs::remove_file("tmp/test.jpg").unwrap();

    Ok("File Uploaded".to_string())
}
