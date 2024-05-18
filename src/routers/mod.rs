use crate::middleware::{cors::cors_middleware, jwt::jwt_middleware};
use crate::routers::exception::post_add_exception;
use crate::routers::post::*;
use crate::routers::product::{
    delete_product, get_product_all, post_add_product, put_update_product,
};
use crate::routers::product_category::*;
use salvo::{
    prelude::{CatchPanic, Logger, OpenApi, SwaggerUi},
    Router,
};
use crate::routers::permission::{delete_permission, get_permission_all, post_add_permission, put_update_permission};
use crate::routers::permission_role::{get_all_roles, get_role_by_id, post_add_permission_role};
use crate::routers::user::{get_user_by_token_api, set_current_user_hook};

use self::{
    category::*,
    user::{delete_user, get_users, post_add_user, post_login, put_update_user},
};

pub mod category;
pub mod exception;
pub mod post;
pub mod product;
pub mod product_category;
mod static_routers;
pub mod user;
pub mod permission;
pub mod role;
pub mod permission_role;

pub fn router() -> Router {
    let exception_routers = Router::with_path("/api/exception").post(post_add_exception);

    let permission_routers = Router::with_path("/api/permission")
        .hoop(set_current_user_hook)
        .get(get_permission_all).post(post_add_permission).push(Router::with_path("<id>").put(put_update_permission).delete(delete_permission));


    let role_add_routers = Router::with_path("/api/permission_role").get(get_all_roles).post(post_add_permission_role)
        .push(Router::with_path("<id>").get(get_role_by_id));

    let post_routers = Router::with_path("/api/post").get(get_post_all)
        .post(post_add_post)
        .delete(post_add_post)
        .push(
            Router::with_path("<id>")
                .put(put_update_post)
                .delete(delete_post)
                .get(get_post_by_id),
        );
    let product_router = Router::with_path("/api/product").get(get_product_all)
        .post(post_add_product)
        .push(
            Router::with_path("<id>")
                .put(put_update_product)
                .delete(delete_product),
        );

    let product_category_router = Router::with_path("/api/product_category")
        .get(get_product_category_all)
        .post(post_add_product_category)
        .push(
            Router::with_path("<id>")
                .put(put_update_product_category)
                .delete(delete_product_category),
        );

    let category_routers = Router::with_path("/api/category")
        .get(get_category_all)
        .post(post_add_category)
        .push(
            Router::with_path("<id>")
                .put(put_update_category)
                .delete(delete_category),
        );
    let login_router = Router::with_path("/api/login").post(post_login);

    let mut no_auth_routers = vec![
        login_router,
        exception_routers,
        post_routers,
        category_routers,
        product_router,
        product_category_router,
    ];

    let _cors_handler = cors_middleware();

    let user_router = Router::with_path("/api/users")
        .get(get_users)
        .post(post_add_user)
        .push(
            Router::with_path("<id>")
                .put(put_update_user)
                .delete(delete_user),
        ).push(
        Router::with_path("/curr")
            .get(get_user_by_token_api)
    );


    let mut need_auth_routers = vec![user_router, permission_routers, role_add_routers];


    let router = Router::new()
        //.hoop(_cors_handler)
        .hoop(Logger::new())
        .hoop(CatchPanic::new())
        .append(&mut no_auth_routers)
        .push(
            Router::new()
                .append(&mut need_auth_routers)
                .hoop(jwt_middleware()).hoop(set_current_user_hook),
        );
    let doc = OpenApi::new("salvo web api", "0.0.1").merge_router(&router);
    router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"))
}
