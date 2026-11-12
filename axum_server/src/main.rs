use crate::state::AppState;


mod models;
mod state;
mod routes;
mod handlers;

#[tokio::main]
async fn main() {

    let state = AppState::new();

    let app = routes::app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3004").await.unwrap();

    println!("Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}