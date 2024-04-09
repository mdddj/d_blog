use crate::app_writer::AppResult;
use salvo::oapi::endpoint;
#[endpoint]
pub async fn hello() -> AppResult<&'static str> {
    Ok("Hello World from salvo")
}

#[allow(unused_imports)]
mod tests {
    use crate::config::CFG;
    use salvo::test::{ResponseExt, TestClient};
    use salvo::Service;

    #[tokio::test]
    async fn test_hello_world() {
        let service = Service::new(crate::routers::router());

        let content = TestClient::get(format!(
            "http://{}",
            &CFG.server.address.replace("0.0.0.0", "127.0.0.1")
        ))
        .send(&service)
        .await
        .take_string()
        .await
        .unwrap();
        assert_eq!(content, "Hello World from salvo");
    }
}
