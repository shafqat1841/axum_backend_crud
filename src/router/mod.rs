mod person_routes;

use std::sync::Arc;

use axum::{Extension, Router, routing::get};

use crate::{AppState, router::person_routes::person_router};

pub fn create_routes(app_state: Arc<AppState>) -> axum::Router {
    let router = Router::new();
    let person_api = person_router();
    let app_api = router
        .route("/", get(home))
        .nest("/api", person_api)
        .layer(Extension(app_state));
    app_api
}

async fn home() -> &'static str {
    "hello world"
}
