use std::sync::{Arc, RwLock};

use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::State,
    response::Html,
    routing::{delete, get, post},
    Json, Router,
};
use list::Row;
use serde::Deserialize;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

mod hello;
mod list;

#[derive(Clone)]
pub struct AppState {
    entries: Arc<RwLock<Vec<list::Row>>>,
    latest_id: Arc<RwLock<i16>>,
}

#[tokio::main]
async fn main() {
    let entries: Vec<Row> = vec![Row {
        id: 0,
        name: "initial".to_string(),
    }];

    let app_state = AppState {
        entries: Arc::new(RwLock::new(entries)),
        latest_id: Arc::new(RwLock::new(0)),
    };

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(hello))
        .route("/add", post(add_item))
        .route("/delete", delete(remove_item))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(app_state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello(State(state): State<AppState>) -> impl IntoResponse {
    let entries = state.entries.read().unwrap();

    let hello = hello::HelloWorld {
        title: "hello",
        name: "Josh",
        list_items: entries.to_vec(),
    };

    Html(hello.render().unwrap())
}

async fn add_item(State(state): State<AppState>) -> impl IntoResponse {
    let current_id = state.latest_id.read().unwrap();
    let new_id = *current_id + 1;

    drop(current_id);

    let new_entry = Row {
        id: new_id,
        name: "test".to_string(),
    };

    let mut write_id = state.latest_id.write().unwrap();
    *write_id = new_id;

    let mut entries = state.entries.write().unwrap();
    entries.push(new_entry);

    let l = list::List {
        list_items: entries.to_vec(),
    };

    Html(l.render().unwrap())
}

#[derive(Deserialize, Debug)]
struct Remove {
    id: i16,
}

async fn remove_item(
    State(state): State<AppState>,
    Json(remove): Json<Remove>,
) -> impl IntoResponse {
    println!("{:?}", remove);
    let id = remove.id;

    let mut items = state.entries.write().unwrap();

    match items.iter().position(|x| x.id == id) {
        Some(position) => {
            let removed_item = items.remove(position);
            println!("Removed {}", removed_item);
        }
        None => println!("Could not find id: {}", id),
    };

    let l = list::List {
        list_items: items.to_vec(),
    };

    Html(l.render().unwrap())
}
