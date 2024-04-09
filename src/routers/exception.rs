use crate::app_writer::AppWriter;
use crate::dtos::exception::ExceptionResponse;
use crate::{dtos::exception::ExceptionAddRequest, services::exception::add_exception};
use salvo::oapi::endpoint;
use salvo::oapi::extract::JsonBody;
use salvo::Writer;

#[endpoint(tags("exception"))]
pub async fn post_add_exception(
    new_exception: JsonBody<ExceptionAddRequest>,
) -> AppWriter<ExceptionResponse> {
    let response = add_exception(new_exception.0).await;
    AppWriter(response)
}
