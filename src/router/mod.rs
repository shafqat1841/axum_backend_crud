mod person_routes;

use axum::{Router, routing::get};

use crate::router::person_routes::person_router;

pub fn create_routes() -> axum::Router {
    let router = Router::new();
    let person_api = person_router();
    let app_api = router.route("/", get(home)).nest("/api", person_api);
    app_api
}

async fn home() -> &'static str {
    "hello world"
}
