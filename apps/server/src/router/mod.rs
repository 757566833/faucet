use crate::controller;

use axum::{http::StatusCode, response::IntoResponse, routing::{get, post}, Router};

pub fn init_router() -> Router {
    return Router::new()
        .route("/", get(controller::html::view_handler))
        .route("/root/code", get(controller::mail::get_root_code))
        .route("/verification/code", post(controller::mail::send_verification_code))
        .fallback(handler_404);
}
async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
