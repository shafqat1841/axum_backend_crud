mod auth_routes;
mod person_routes;

use std::sync::Arc;

use axum::{Extension, Router, routing::get};

use crate::{
    AppState,
    router::{auth_routes::auth_router, person_routes::person_router},
};

pub fn create_routes(app_state: Arc<AppState>) -> axum::Router {
    let router = Router::new();
    let person_api = person_router();
    let auth_api = auth_router();
    let api_route = router
        .nest("/auth", auth_api)
        .nest("/persons", person_api)
        .layer(Extension(app_state));

    let home_route = Router::new().route("/", get(home));

    let app_api = Router::new().merge(home_route).nest("/api", api_route);
    app_api
}

async fn home() -> &'static str {
    "hello world"
}
