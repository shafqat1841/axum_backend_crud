use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::handlers::person_handlers::{
    create_person, delete_person, get_person, get_persons, update_person,
};

pub fn person_router() -> Router {
    let router = Router::new();
    let api = router
        .route("/get_persons", get(get_persons))
        .route("/create_person", post(create_person))
        .route("/get_person/{id}", get(get_person))
        .route("/update_person/{id}", put(update_person))
        .route("/delete_person/{id}", delete(delete_person));
    api
}
