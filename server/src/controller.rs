use axum::{
    routing::{delete, get, post},
    Router,
};
use tower_http::services::ServeDir;

use crate::{db::RedisDatabase, processor::Processor};

pub struct Controller {
    processor: Processor<RedisDatabase>,
}

impl Controller {
    fn new(database: RedisDatabase) -> Controller {
        let processor = Processor::new(database);
        return Controller { processor };
    }

    pub fn create_app() -> Router {
        let app = Router::new()
            .route("/", get(Controller::index))
            .route("/add", post(Controller::add_item))
            .route("/delete/:id", delete(Controller::remove_item))
            .nest_service("/dist", ServeDir::new("dist"));

        return app;
    }

    async fn index() {}

    async fn add_item() {}

    async fn remove_item() {}
}
