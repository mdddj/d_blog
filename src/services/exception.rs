use crate::app_writer::AppResult;
use crate::db::DB;
use crate::dtos::exception::{ExceptionAddRequest, ExceptionResponse};
use crate::entities::{exception, prelude::Exception};
use crate::utils::date_util::get_current_time_as_string;
use sea_orm::{EntityTrait, Set};

pub async fn add_exception(
    exception_add_request: ExceptionAddRequest,
) -> AppResult<ExceptionResponse> {
    let db = DB.get().ok_or(anyhow::anyhow!("连接数据库失败"))?;
    let date = get_current_time_as_string();
    let model = exception::ActiveModel {
        id: Default::default(),
        content: Set(exception_add_request.content),
        os: Set(exception_add_request.os),
        post_date: Set(date.clone()),
        create_date: Set(date),
    };
    let exception = Exception::insert(model).exec(db).await?;
    Ok(ExceptionResponse {
        id: exception.last_insert_id.to_string(),
    })
}
