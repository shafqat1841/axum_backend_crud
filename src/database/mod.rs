use async_trait::async_trait;

use crate::{db::DBClient, models::PersonModel};

#[async_trait]
trait PersonModelExt {
    async fn get_persons(&self) -> Result<Option<Vec<PersonModel>>, sqlx::Error>;
}

#[async_trait]
impl PersonModelExt for DBClient {
    async fn get_persons(&self) -> Result<Option<Vec<PersonModel>>, sqlx::Error> {
        let query = r#"
        SELECT  
                 id, 
                username, 
                age, 
                created_at, 
                updated_at 
        FROM persons 
        "#;

        let user: Vec<PersonModel> = sqlx::query_as::<_, PersonModel>(query)
            .fetch_all(&self.pool)
            .await?;

        Ok(Some(user))
    }
}
