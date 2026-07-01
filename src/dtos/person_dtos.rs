use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreatePersonDto {
    #[validate(length(min = 3, max = 99, message = "Username must be at least 3 characters long and at most 99 characters long"))]
    #[validate(custom (function = "validate_username"))]
    pub name: String,
     #[validate(range(min = 12, max = 200, message = "Age must be between 12 and 200"))]
    pub age: u32,
}

fn validate_username(username: &str) -> Result<(), ValidationError> {
    let re = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    if !re.is_match(username) {
        return Err(ValidationError::new(
            "Username can only contain letters, numbers, and underscores",
        ));
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct GetPersonResDto {
    pub id: Uuid,
    pub username: String,
}

#[derive(Validate, Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct UpdatePersonReqDto {
    #[validate(length(min = 3, max = 99, message = "Name must be at least 3 characters long and at most 99 characters long"))]
    pub name: String,
    #[validate(range(min = 12, max = 200, message = "Age must be between 12 and 200"))]
    pub age: u32,
}
