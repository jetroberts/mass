use askama_axum::IntoResponse;

pub async fn get() -> impl IntoResponse {
    format!("Hello, world!")
}
