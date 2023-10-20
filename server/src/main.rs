use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{rejection::FormRejection, Path, State},
    response::Html,
};
use chrono::Utc;
use controller::Controller;
use db::RedisDatabase;
use list::WeightEntry;
use processor::Processor;
use serde::Deserialize;

mod add;
mod components;
mod controller;
mod db;
mod hello;
mod list;
mod processor;

#[tokio::main]
async fn main() {
    let redis_db = RedisDatabase::new();
    let processor = Processor::new(redis_db);

    let app = Controller::create_app();

    axum::Server::bind(&"0.0.0.0:3003".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
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
