use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;


#[derive(Deserialize, Serialize, Debug)]
pub struct Person {
    id: u32,
    name: String,
    age: u32,
}

impl Person {
    fn new(id: u32, name: String, age: u32) -> Self {
        Person { id, name, age }
    }
}

pub type ShareState = Arc<Mutex<Vec<Person>>>;

pub type ShareStateExt = Extension<ShareState>;

#[derive(Deserialize, Serialize, Debug)]
struct CreatePersonReq {
    name: String,
    age: u32,
}

#[derive(Deserialize, Serialize, Debug)]
struct UpdatePersonReq {
    name: String,
    age: u32,
}

fn create_shared_state() -> ShareState {
    let person_list: Vec<Person> = Vec::new();
    let shared_state = Arc::new(Mutex::new(person_list));
    shared_state
}

pub fn person_router() -> Router {
        let shared_state = create_shared_state();
    let router = Router::new();
    let api = router
        .route("/get_persons", get(get_persons))
        .route("/get_person/{id}", get(get_person))
        .route("/create_person", post(create_person))
        .route("/update_person/{id}", put(update_person))
        .route("/delete_person/{id}", delete(delete_person))
        .layer(Extension(shared_state));
    api
}

async fn get_persons(Extension(shared_state): ShareStateExt) -> impl IntoResponse {
    let person_list = shared_state.lock().await;
    if person_list.is_empty() {
        let res = format!("no data found");
        return (StatusCode::NOT_FOUND, res).into_response();
    }

    let all_persons = serde_json::to_vec_pretty(&*person_list);

    let res = match all_persons {
        Ok(json) => (StatusCode::OK, json).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to serialize persons",
        )
            .into_response(),
    };

    res
}

async fn get_person(
    Extension(shared_state): ShareStateExt,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    let person_list = shared_state.lock().await;
    if person_list.is_empty() {
        let res = format!("no data found");
        return (StatusCode::NOT_FOUND, res).into_response();
    }

    let person = person_list.iter().find(|p| p.id == id);
    if let Some(person) = person {
        let person_data = serde_json::to_string_pretty(person);
        match person_data {
            Ok(json) => (StatusCode::OK, json).into_response(),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to serialize person",
            )
                .into_response(),
        }
    } else {
        let res = format!("person with id {} not found", id);
        (StatusCode::NOT_FOUND, res).into_response()
    }
}

async fn create_person(
    Extension(shared_state): ShareStateExt,
    Json(person): Json<CreatePersonReq>,
) -> impl IntoResponse {
    let mut person_list = shared_state.lock().await;
    let id = person_list.len() as u32 + 1;
    let person_data = Person::new(id, person.name, person.age);
    person_list.push(person_data);
    let res = format!("person added");
    (StatusCode::OK, res).into_response()
}

async fn update_person(
    Extension(shared_state): ShareStateExt,
    Path(id): Path<u32>,
    Json(person): Json<UpdatePersonReq>,
) -> impl IntoResponse {
    let mut person_list = shared_state.lock().await;

    let person_exists = person_list.iter().any(|p| p.id == id);

    if !person_exists {
        let res = format!("person with id {} not found", id);
        return (StatusCode::NOT_FOUND, res).into_response();
    }

    person_list.iter_mut().for_each(|p| {
        if p.id == id {
            p.name = person.name.clone();
            p.age = person.age;
        }
    });
    let res = format!("person with id {} updated", id);
    (StatusCode::OK, res).into_response()
}

async fn delete_person(
    Extension(shared_state): ShareStateExt,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    let mut person_list = shared_state.lock().await;
    let person_exists = person_list.iter().any(|p| p.id == id);

    if !person_exists {
        let res = format!("person with id {} not found", id);
        return (StatusCode::NOT_FOUND, res).into_response();
    }

    person_list.retain(|p| p.id != id);
    let res = format!("person with id {} deleted", id);
    (StatusCode::OK, res).into_response()
}
