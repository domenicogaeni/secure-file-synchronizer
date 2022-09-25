use diesel::prelude::*;

#[derive(Queryable)]
pub struct UploadedFile {
    pub path: String,
    pub hash: String,
}
