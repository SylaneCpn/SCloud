use axum::response::Html;
use tokio;

pub async fn root() -> Html<String> {
    let contents = tokio::fs::read_to_string("assets/index.html")
        .await
        .unwrap();
    Html(contents)
}
