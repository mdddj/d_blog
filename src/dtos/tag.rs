use salvo::prelude::{Extractible, ToSchema};
use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
pub struct TagAddRequest {
    pub name: String,
}

#[derive(Debug, Deserialize, Extractible, ToSchema, Default)]
#[salvo(extract(default_source(from = "body", parse = "json")))]
pub struct TagUpdateRequest {
    #[salvo(extract(source(from = "param")))]
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct TagResponse {
    pub id: i32,
}
