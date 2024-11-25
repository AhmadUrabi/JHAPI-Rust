use crate::server::request_guard::api_key::ApiKey;
use crate::utils::permissions::has_admin_perm;
use crate::utils::permissions::has_query_perm;
use crate::utils::permissions::is_images_perm;

use crate::server::JHApiServerState;
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::{get, State};

use crate::functions::files::download_file;
use crate::functions::files::upload_file;

use crate::utils::structs::APIErrors;

use std::path::*;

pub fn routes() -> Vec<rocket::Route> {
    routes![get_image, upload]
}

#[get("/images/<file..>")]
pub async fn get_image(
    file: PathBuf,
    _key: ApiKey<'_>,
    state: &State<JHApiServerState>,
) -> Result<Option<NamedFile>, Status> {
    let pool = &state.pool;
    let sql_manager = &state.sql_manager;
    if !has_query_perm(&_key, pool, &sql_manager).await
        && !has_admin_perm(&_key, pool, &sql_manager).await
    {
        return Err(Status::Unauthorized);
    }
    info!("Image Request: {:?}", file);

    let filename = file.to_str().unwrap().to_string();

    if filename == "" {
        return Err(Status::NotFound);
    }

    match download_file(&filename).await {
        Ok(()) => info!("File Downloaded"),
        Err(e) => {
            info!("File Not Found");
            match e {
                APIErrors::SFTPError => return Err(Status::InternalServerError),
                APIErrors::FileNotFound => return Err(Status::NotFound),
                _ => return Err(Status::InternalServerError),
            }
        }
    };
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
    state: &State<JHApiServerState>,
) -> Result<String, Status> {
    let pool = &state.pool;
    let sql_manager = &state.sql_manager;
    if !is_images_perm(&_key, pool, &sql_manager).await
        && !has_admin_perm(&_key, pool, &sql_manager).await
    {
        return Err(Status::Unauthorized);
    }

    info!("Image Upload Request: {:?}", params.item_code);

    // Save file temporarily

    if params.file.name().is_none() || params.item_code == "" {
        return Err(Status::BadRequest);
    }
    let filename = "tmp/".to_string() + params.file.name().unwrap();
    params.file.persist_to(&filename).await.unwrap();

    // Upload file to server
    match upload_file(&params.item_code, &filename).await {
        Ok(()) => info!("File Uploaded"),
        Err(e) => {
            info!("File Not Uploaded");
            match e {
                APIErrors::SFTPError => return Err(Status::InternalServerError),
                APIErrors::FileNotFound => return Err(Status::NotFound),
                _ => return Err(Status::InternalServerError),
            }
        }
    }

    // Delete temporary file
    std::fs::remove_file(filename).unwrap();
    Ok("File Uploaded".to_string())
}
