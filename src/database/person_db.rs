use async_trait::async_trait;

use crate::{
    db::DBClient, dtos::person_dtos::{CreatePersonDto, GetPersonResDto}, models::persons_model::PersonModel,
};

#[async_trait]
pub trait PersonModelExt {
    async fn get_persons(&self) -> Result<Vec<GetPersonResDto>, sqlx::Error>;

    async fn create_person(&self, person: CreatePersonDto) -> Result<GetPersonResDto, sqlx::Error>;

    async fn get_person(&self, id: uuid::Uuid) -> Result<PersonModel, sqlx::Error>;

    async fn update_person(
        &self,
        id: uuid::Uuid,
        name: String,
        age: u32,
    ) -> Result<PersonModel, sqlx::Error>;

    async fn delete_person(&self, id: uuid::Uuid) -> Result<(), sqlx::Error>;
}

#[async_trait]
impl PersonModelExt for DBClient {
    async fn get_persons(&self) -> Result<Vec<GetPersonResDto>, sqlx::Error> {
        let query = r#"
        SELECT  
                 id, 
                username
        FROM persons 
        "#;

        let persons: Vec<GetPersonResDto> = sqlx::query_as::<_, GetPersonResDto>(query)
            .fetch_all(&self.pool)
            .await?;

        Ok(persons)
    }

    async fn create_person(&self, person: CreatePersonDto) -> Result<GetPersonResDto, sqlx::Error> {
        let query = r#"
        INSERT INTO persons (username, age) 
        VALUES ($1, $2) 
        RETURNING id, username
        "#;

        let user: GetPersonResDto = sqlx::query_as::<_, GetPersonResDto>(query)
            .bind(person.name)
            .bind(person.age as i32)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    async fn get_person(&self, id: uuid::Uuid) -> Result<PersonModel, sqlx::Error> {
        let query = r#"
        SELECT  
                 id, 
                username, 
                age, 
                created_at, 
                updated_at 
        FROM persons 
        WHERE id = $1
        "#;

        let user: PersonModel = sqlx::query_as::<_, PersonModel>(query)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    async fn update_person(
        &self,
        id: uuid::Uuid,
        name: String,
        age: u32,
    ) -> Result<PersonModel, sqlx::Error> {
        let query = r#"
        UPDATE persons 
        SET username = $2, age = $3, updated_at = CURRENT_TIMESTAMP 
        WHERE id = $1 
        RETURNING id, username, age, created_at, updated_at
        "#;

        let user: PersonModel = sqlx::query_as::<_, PersonModel>(query)
            .bind(id)
            .bind(name)
            .bind(age as i32)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    async fn delete_person(&self, id: uuid::Uuid) -> Result<(), sqlx::Error> {
        let query = r#"
        DELETE FROM persons 
        WHERE id = $1
        "#;

        sqlx::query(query)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
