use oracle::pool::Pool;
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::{get, State};

use crate::utils::permissions::is_images_perm;
use crate::ApiKey;

use std::path::*;

#[get("/images/<file..>")]
pub async fn files(
    file: PathBuf,
    _key: ApiKey<'_>,
    pool: &State<Pool>,
) -> Result<Option<NamedFile>, Status> {
    if !is_images_perm(&_key, pool) && !is_admin_perm(&_key, pool) {
        return Err(Status::Unauthorized);
    }
    info!("Image Request: {:?}", file);
    Ok(NamedFile::open(Path::new("static/").join(file)).await.ok())
}
