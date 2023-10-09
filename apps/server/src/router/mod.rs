use crate::controller;

use axum::{
    http::{HeaderValue, Method, StatusCode, self},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;

pub fn init_router() -> Router {
    return Router::new()
        .route("/", get(controller::html::view_handler))
        .route("/root/code", get(controller::mail::get_root_code))
        .route(
            "/verification/code",
            post(controller::mail::send_verification_code),
        )
        .route("/faucet", post(controller::faucet::faucet))
        .fallback(handler_404)
        .layer(
            // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
            // for more details
            //
            // pay attention that for some request types like posting content-type: application/json
            // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
            // or see this issue https://github.com/tokio-rs/axum/issues/849
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_headers([http::header::CONTENT_TYPE])
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                    Method::OPTIONS,
                    Method::PATCH,
                ]),
        );
}
async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
