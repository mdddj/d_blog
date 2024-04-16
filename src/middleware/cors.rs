use crate::config::CFG;
use salvo::cors::{AllowCredentials, AllowHeaders, AllowMethods, AllowOrigin, Cors, CorsHandler};
use tracing::info;

pub fn cors_middleware() -> CorsHandler {
    let cors_handler = Cors::new()
        .allow_origin(AllowOrigin::any())
        .allow_methods(AllowMethods::any())
        .allow_headers(AllowHeaders::any())
        .into_handler();
    cors_handler
}
