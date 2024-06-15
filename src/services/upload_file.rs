use salvo::{handler, Request, Response};
use salvo::http::StatusCode;

#[handler]
async fn upload_file_service(request: &mut Request,response: &mut Response) {
    let file = request.files("file").await;
    if let Some(file) = file {

    }else{
        response.status_code(StatusCode::BAD_REQUEST);
    }
}