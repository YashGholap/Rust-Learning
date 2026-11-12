use axum::{routing::get, Router};

use crate::state::AppState;
use crate::handlers::tasks;


pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/tasks", get(tasks::list).post(tasks::create))
        .route("/tasks/{id}", get(tasks::get_one).delete(tasks::delete).patch(tasks::update))
        .with_state(state)

}


async fn health() -> &'static str {
    "ok"
}