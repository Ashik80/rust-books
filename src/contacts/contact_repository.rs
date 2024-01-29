use sqlx::{PgPool, postgres::PgQueryResult};
use super::models::{Contact, NewContact};

pub struct ContactRepository {
    pool: PgPool
}

impl ContactRepository {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn get_all(self) -> Result<Vec<Contact>, sqlx::Error> {
        sqlx::query_as!(Contact, "SELECT * FROM contacts").fetch_all(&self.pool).await
    }

    pub async fn get_by_id(self, id: i32) -> Result<Contact, sqlx::Error> {
        sqlx::query_as!(Contact, "SELECT * FROM contacts WHERE id = $1", id).fetch_one(&self.pool).await
    }

    pub async fn create(self, contact: NewContact) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "INSERT INTO contacts (name, email, phone) VALUES ($1, $2, $3)",
            contact.name, contact.email, contact.phone
        ).execute(&self.pool).await
    }

    pub async fn update(self, id:i32, contact: NewContact) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "UPDATE contacts SET name = $1, email = $2, phone = $3 WHERE id = $4",
            contact.name, contact.email, contact.phone, id
        ).execute(&self.pool).await
    }

    pub async fn delete(self, id: i32) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!("DELETE FROM contacts WHERE id = $1", id).execute(&self.pool).await
    }
}
