use rocket::fs::NamedFile;
use rocket::{State, get};
use rocket::http::Status;
use oracle::pool::Pool;

use crate::ApiKey;
use crate::signing::get_image_permissions;

use std::path::*;


#[get("/images/<file..>")]
pub async fn files(file: PathBuf, _key: ApiKey<'_>, pool: &State<Pool>) -> Result<Option<NamedFile>, Status> {
    if get_image_permissions(_key.0, &pool) {
        info!("Image Request: {:?}", file);
        Ok(NamedFile::open(Path::new("static/").join(file)).await.ok())
    } else {
        error!("Image Request: {:?} - Unauthorized", file);
        Err(Status::Unauthorized)
    }
}