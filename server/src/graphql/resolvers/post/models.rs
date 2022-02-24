use crate::graphql::shared::errors::SharedError;
use async_graphql::{ErrorExtensions, InputObject, Result, SimpleObject};
use chrono::{Local, NaiveDateTime};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::PostError;

#[derive(SimpleObject)]
pub struct Post {
    id: Uuid,
    title: String,
    slug: String,
    reading_time: Option<i32>,
    visible: bool,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
}

impl Post {
    pub async fn read_count(db_pool: &Pool<Postgres>) -> Result<usize> {
        Ok(sqlx::query!(r#"SELECT COUNT(*) AS "count!" FROM post"#)
            .fetch_one(db_pool)
            .await
            .map_err(|_| SharedError::Internal.extend())?
            .count as usize)
    }

    pub async fn read_many(db_pool: &Pool<Postgres>, limit: i64, offset: i64) -> Result<Vec<Self>> {
        sqlx::query_as!(
            Post,
            r#"
            SELECT id, title, slug, reading_time, visible, created_at, updated_at
            FROM post
            ORDER BY coalesce(updated_at, created_at) DESC, reading_time DESC, title ASC
            LIMIT $1
            OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(db_pool)
        .await
        .map_err(|_| SharedError::Internal.extend())
    }

    pub async fn read_one(db_pool: &Pool<Postgres>, id: Uuid) -> Result<Post> {
        sqlx::query_as!(
            Post,
            r#"
            SELECT id, title, slug, reading_time, visible, created_at, updated_at
            FROM post
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(db_pool)
        .await
        .map_err(|_| SharedError::Internal.extend())?
        .ok_or_else(|| PostError::NotFound.extend())
    }

    pub async fn delete_one(db_pool: &Pool<Postgres>, id: Uuid) -> Result<Post> {
        sqlx::query_as!(
            Post,
            r#"
            DELETE FROM post
            WHERE id = $1
            RETURNING id, title, slug, reading_time, visible, created_at, updated_at
            "#,
            id
        )
        .fetch_optional(db_pool)
        .await
        .map_err(|_| SharedError::Internal.extend())?
        .ok_or_else(|| PostError::NotFound.extend())
    }
}

// ----------------------------------------------------------------
// CREATE
// ----------------------------------------------------------------
#[derive(InputObject)]
pub struct PostCreate {
    title: String,
    slug: String,
    reading_time: Option<i32>,
    visible: bool,
}

impl PostCreate {
    pub async fn create(&self, db_pool: &Pool<Postgres>) -> Result<Post> {
        sqlx::query_as!(
            Post,
            r#"
            INSERT INTO post(title, slug, reading_time, visible)
            VALUES ($1, $2, $3, $4)
            RETURNING id, title, slug, reading_time, visible, created_at, updated_at
            "#,
            self.title,
            self.slug,
            self.reading_time,
            self.visible
        )
        .fetch_one(db_pool)
        .await
        .map_err(|_| SharedError::Internal.extend())
    }
}

// ----------------------------------------------------------------
// UPDATE
// ----------------------------------------------------------------
#[derive(InputObject)]
pub struct PostUpdate {
    title: Option<String>,
    slug: Option<String>,
    reading_time: Option<i32>,
    visible: Option<bool>,
}

impl PostUpdate {
    pub async fn update(&self, db_pool: &Pool<Postgres>, id: Uuid) -> Result<Post> {
        sqlx::query_as!(
            Post,
            r#"
            UPDATE post
            SET
                title = coalesce($2, title),
                slug = coalesce($3, slug),
                reading_time = coalesce($4, reading_time),
                visible = coalesce($5, visible),
                updated_at = $6
            WHERE id = $1
            RETURNING id, title, slug, reading_time, visible, created_at, updated_at;
            "#,
            id,
            self.title,
            self.slug,
            self.reading_time,
            self.visible,
            Local::now().naive_local()
        )
        .fetch_optional(db_pool)
        .await
        .map_err(|_| SharedError::Internal.extend())?
        .ok_or_else(|| PostError::NotFound.extend())
    }
}
