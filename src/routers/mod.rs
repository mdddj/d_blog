use crate::middleware::{cors::cors_middleware, jwt::jwt_middleware};
use crate::routers::exception::post_add_exception;
use salvo::{
    prelude::{CatchPanic, Logger, OpenApi, SwaggerUi},
    Router,
};
use crate::routers::post::*;

use self::{
    demo::hello,
    user::{delete_user, get_users, post_add_user, post_login, put_update_user},
    category::*,
};

pub mod demo;
pub mod exception;
mod static_routers;
pub mod user;
pub mod post;
pub mod category;

pub fn router() -> Router {
    let exception_routers = Router::with_path("/api/exception").post(post_add_exception);

    let post_routers = Router::with_path("/api/post").get(get_post_all).post(post_add_post).push(Router::with_path("<id>").put(put_update_post).delete(delete_post)
        .get(get_post_by_id));

    let category_routers = Router::with_path("/api/category").get(get_category_all).post(post_add_category).push(Router::with_path("<id>").put(put_update_category).delete(delete_category));
    let mut no_auth_routers = vec![
        Router::with_path("/api/login").post(post_login),
        exception_routers,
        post_routers,
        category_routers,
    ];

    let _cors_handler = cors_middleware();

    let user_router = Router::with_path("/api/users")
        .get(get_users)
        .post(post_add_user)
        .push(
            Router::with_path("<id>")
                .put(put_update_user)
                .delete(delete_user),
        );
    let mut need_auth_routers = vec![user_router];

    let router = Router::new()
        //.hoop(_cors_handler)
        .hoop(Logger::new())
        .hoop(CatchPanic::new())
        .get(hello)
        .append(&mut no_auth_routers)
        .push(
            Router::new()
                .append(&mut need_auth_routers)
                .hoop(jwt_middleware()),
        );
    let doc = OpenApi::new("salvo web api", "0.0.1").merge_router(&router);
    router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"))
}
