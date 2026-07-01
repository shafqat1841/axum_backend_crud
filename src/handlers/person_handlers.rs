use std::sync::Arc;

use axum::{Extension, Json, extract::Path, http::StatusCode, response::IntoResponse};
use uuid::Uuid;

use crate::{
    AppState,
    dtos::person_dtos::{CreatePersonDto, UpdatePersonReqDto},
    errors::ErrorMessage,
};
use crate::{database::person_db::PersonModelExt, errors::HttpError};
use validator::Validate;

pub async fn get_persons(
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let persons_list = app_state
        .db_client
        .get_persons()
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let all_persons = serde_json::to_vec_pretty(&*persons_list);

    let res = match all_persons {
        Ok(json) => (StatusCode::OK, json).into_response(),
        Err(e) => {
            let error = HttpError::server_error(e.to_string());
            error.into_http_response()
        }
    };

    Ok(res)
}

pub async fn create_person(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<CreatePersonDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let person = app_state.db_client.create_person(body).await;

    match person {
        Ok(person) => {
            let res = serde_json::to_string_pretty(&person);
            match res {
                Ok(json) => Ok((StatusCode::CREATED, json).into_response()),
                Err(e) => {
                    let error = HttpError::server_error(e.to_string());
                    Err(error)
                }
            }
        }
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                let constraint = db_err.constraint().unwrap_or_default();

                if constraint.contains("username") {
                    Err(HttpError::unique_constraint_violation(
                        ErrorMessage::UsernameExist.to_string(),
                    ))
                } else {
                    Err(HttpError::server_error(
                        "Unique constraint violation".to_string(),
                    ))
                }
            } else {
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

pub async fn get_person(
    Extension(app_state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, HttpError> {
    let person = app_state.db_client.get_person(id).await;

    match person {
        Ok(person) => {
            let person_data = serde_json::to_string_pretty(&person);
            match person_data {
                Ok(json) => Ok((StatusCode::CREATED, json).into_response()),
                Err(e) => {
                    let error = HttpError::server_error(e.to_string());
                    Err(error)
                }
            }
        }
        Err(sqlx::Error::RowNotFound) => {
            let res = format!("person with id {} not found", id);
            Err(HttpError::not_found(res))
        }
        Err(e) => {
            let error = HttpError::server_error(e.to_string());
            Err(error)
        }
    }
}

pub async fn update_person(
    Extension(app_state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdatePersonReqDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let person = app_state
        .db_client
        .update_person(id, body.name, body.age)
        .await;

    match person {
        Ok(person) => {
            let person_data = serde_json::to_string_pretty(&person);
            match person_data {
                Ok(json) => Ok((StatusCode::OK, json).into_response()),
                Err(e) => {
                    let error = HttpError::server_error(e.to_string());
                    Err(error)
                }
            }
        }
        Err(sqlx::Error::RowNotFound) => {
            let res = format!("person with id {} not found", id);
            Err(HttpError::not_found(res))
        }
        Err(e) => {
            let error = HttpError::server_error(e.to_string());
            Err(error)
        }
    }
}

pub async fn delete_person(
    Extension(app_state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, HttpError> {
    let person = app_state.db_client.delete_person(id).await;

    match person {
        Ok(_) => {
            let res = format!("person with id {} deleted", id);
            Ok((StatusCode::OK, res).into_response())
        }
        Err(sqlx::Error::RowNotFound) => {
            let res = format!("person with id {} not found", id);
            Err(HttpError::not_found(res))
        }
        Err(e) => {
            let error = HttpError::server_error(e.to_string());
            Err(error)
        }
    }
}
