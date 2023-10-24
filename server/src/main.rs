use axum::routing::get;
use db::RedisDatabase;
use handlers::index;

mod db;
mod handlers;
mod processors;

#[tokio::main]
async fn main() {
    let _redis_db = RedisDatabase::new();

    let app = axum::Router::new().route("/", get(index::get));

    axum::Server::bind(&"0.0.0.0:3003".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
