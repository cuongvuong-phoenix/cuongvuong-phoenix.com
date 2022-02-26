use crate::graphql::shared::errors::SharedError;
use async_graphql::{ErrorExtensions, InputObject, Result, SimpleObject};
use chrono::{Local, NaiveDateTime};
use sqlx::{Pool, Postgres};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct HomeContent {
    biography: String,
    contact: String,
    years_of_experience: i32,
    num_projects: i32,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
}

impl HomeContent {
    pub async fn read(db_pool: &Pool<Postgres>) -> Result<HomeContent> {
        sqlx::query_as!(
            HomeContent,
            r#"
            SELECT biography, contact, years_of_experience, num_projects, created_at, updated_at
            FROM home
            "#
        )
        .fetch_one(db_pool)
        .await
        .map_err(|e| SharedError::Database(e).extend())
    }
}

// ----------------------------------------------------------------
// CREATE
// ----------------------------------------------------------------
#[derive(InputObject)]
pub struct HomeContentCreate {
    biography: String,
    contact: String,
    years_of_experience: i32,
    num_projects: i32,
}

impl HomeContentCreate {
    pub async fn create(&self, db_pool: &Pool<Postgres>) -> Result<HomeContent> {
        sqlx::query_as!(
            HomeContent,
            r#"
            INSERT INTO home(biography, contact, years_of_experience, num_projects)
            VALUES ($1, $2, $3, $4)
            RETURNING biography, contact, years_of_experience, num_projects, created_at, updated_at
            "#,
            self.biography,
            self.contact,
            self.years_of_experience,
            self.num_projects
        )
        .fetch_one(db_pool)
        .await
        .map_err(|e| SharedError::Database(e).extend())
    }
}

// ----------------------------------------------------------------
// UPDATE
// ----------------------------------------------------------------
#[derive(InputObject)]
pub struct HomeContentUpdate {
    biography: Option<String>,
    contact: Option<String>,
    years_of_experience: Option<i32>,
    num_projects: Option<i32>,
}

impl HomeContentUpdate {
    pub async fn update(&self, db_pool: &Pool<Postgres>) -> Result<HomeContent> {
        sqlx::query_as!(
            HomeContent,
            r#"
            UPDATE home
            SET
                biography = coalesce($1, biography),
                contact = coalesce($2, contact),
                years_of_experience = coalesce($3, years_of_experience),
                num_projects = coalesce($4, num_projects),
                updated_at = $5
            RETURNING biography, contact, years_of_experience, num_projects, created_at, updated_at
            "#,
            self.biography,
            self.contact,
            self.years_of_experience,
            self.num_projects,
            Local::now().naive_local(),
        )
        .fetch_one(db_pool)
        .await
        .map_err(|e| SharedError::Database(e).extend())
    }
}
