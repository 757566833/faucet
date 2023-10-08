use axum::response::Html;

pub async fn view_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
