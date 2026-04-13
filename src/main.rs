use axum::{Router, routing::get};
use minijinja::Environment;
use std::sync::Arc;

mod error;
mod routes;

use routes::{index, open_redirect};

pub struct AppState {
    templates: Environment<'static>,
}

#[tokio::main]
async fn main() {
    let mut templates = Environment::new();
    templates.set_loader(minijinja::path_loader("templates"));

    let app_state = Arc::new(AppState { templates });

    let app = Router::new()
        .route("/", get(index))
        .route("/open-redirect", get(open_redirect))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
