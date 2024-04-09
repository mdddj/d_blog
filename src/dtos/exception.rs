use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Validate, ToSchema)]
pub struct ExceptionAddRequest {
    pub os: String,
    pub content: String,
}

#[derive(Serialize, Debug, ToSchema, Default)]
pub struct ExceptionResponse {
    pub id: String,
}
