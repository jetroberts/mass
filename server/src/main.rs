use std::sync::{Arc, RwLock};

use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{rejection::FormRejection, Path, State},
    response::Html,
    routing::{delete, get, post},
    Form, Router,
};
use chrono::Utc;
use list::WeightEntry;
use serde::Deserialize;
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, trace::TraceLayer};

mod add;
mod hello;
mod list;

#[derive(Clone)]
pub struct AppState {
    entries: Arc<RwLock<Vec<list::WeightEntry>>>,
    latest_id: Arc<RwLock<i16>>,
}

#[tokio::main]
async fn main() {
    let entries: Vec<list::WeightEntry> = vec![WeightEntry {
        id: 0,
        weight_type: String::from("Bench"),
        mass: 100,
        reps: 3,
        date: Utc::now(),
    }];

    let app_state = AppState {
        entries: Arc::new(RwLock::new(entries)),
        latest_id: Arc::new(RwLock::new(1)),
    };

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(hello))
        .route("/add", post(add_item))
        .route("/delete/:id", delete(remove_item))
        .nest_service("/dist", ServeDir::new("dist"))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(app_state);

    axum::Server::bind(&"0.0.0.0:3003".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello(State(state): State<AppState>) -> impl IntoResponse {
    let entries = state.entries.read().unwrap();

    let hello = hello::HelloWorld {
        title: "hello",
        list: list::List {
            list_items: entries.to_vec(),
            err: None,
        },
        add: add::Add {
            weight_types: vec![
                String::from("Bench"),
                String::from("Squat"),
                String::from("Deadlift"),
            ],
        },
    };

    Html(hello.render().unwrap())
}

#[derive(Deserialize)]
struct AddForm {
    weight_type: String,
    mass: i32,
    reps: i32,
}

async fn add_item(
    State(state): State<AppState>,
    form_res: Result<Form<AddForm>, FormRejection>,
) -> impl IntoResponse {
    match form_res {
        Ok(form) => {
            let f = form.0;
            let current_id = state.latest_id.read().unwrap();
            let new_id = *current_id + 1;

            drop(current_id);

            let new_entry = WeightEntry {
                id: new_id,
                weight_type: f.weight_type,
                mass: f.mass,
                reps: f.reps,
                date: Utc::now(),
            };

            let mut write_id = state.latest_id.write().unwrap();
            *write_id = new_id;

            let mut entries = state.entries.write().unwrap();
            entries.push(new_entry);

            let l = list::List {
                list_items: entries.to_vec(),
                err: None,
            };

            Html(l.render().unwrap())
        }
        Err(_) => Html("Invalid form data".to_string()),
    }
}

async fn remove_item(State(state): State<AppState>, Path(id): Path<i16>) -> impl IntoResponse {
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
        err: None,
    };

    Html(l.render().unwrap())
}
