use super::errors::TagError;
use crate::graphql::shared::errors::SharedError;
use async_graphql::{ErrorExtensions, InputObject, Result, SimpleObject};
use chrono::{Local, NaiveDateTime};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

// ----------------------------------------------------------------
// READ
// ----------------------------------------------------------------
#[derive(SimpleObject)]
pub struct Tag {
    id: Uuid,
    name: String,
    icon: Option<String>,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
}

impl Tag {
    pub async fn read_count(db_pool: &Pool<Postgres>) -> Result<usize> {
        Ok(sqlx::query!(r#"SELECT COUNT(*) AS "count!" FROM tag"#)
            .fetch_one(db_pool)
            .await
            .map_err(|_| SharedError::Internal.extend())?
            .count as usize)
    }

    pub async fn read_many(db_pool: &Pool<Postgres>, limit: i64, offset: i64) -> Result<Vec<Tag>> {
        sqlx::query_as!(
            Tag,
            r#"
            SELECT id, name, icon, created_at, updated_at
            FROM tag
            ORDER BY coalesce(updated_at, created_at) DESC
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

    pub async fn read_one(db_pool: &Pool<Postgres>, id: Uuid) -> Result<Tag> {
        sqlx::query_as!(
            Tag,
            r#"
            SELECT id, name, icon, created_at, updated_at
            FROM tag
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(db_pool)
        .await
        .map_err(|_| SharedError::Internal.extend())?
        .ok_or_else(|| TagError::NotFound.extend())
    }

    pub async fn delete(db_pool: &Pool<Postgres>, id: Uuid) -> Result<Tag> {
        sqlx::query_as!(
            Tag,
            r#"
            DELETE FROM tag
            WHERE id = $1
            RETURNING id, name, icon, created_at, updated_at
            "#,
            id
        )
        .fetch_optional(db_pool)
        .await
        .map_err(|_| SharedError::Internal.extend())?
        .ok_or_else(|| TagError::NotFound.extend())
    }
}

// ----------------------------------------------------------------
// CREATE
// ----------------------------------------------------------------
#[derive(InputObject)]
pub struct TagCreate {
    name: String,
    icon: Option<String>,
}

impl TagCreate {
    pub async fn create(&self, db_pool: &Pool<Postgres>) -> Result<Tag> {
        sqlx::query_as!(
            Tag,
            r#"
            INSERT INTO tag(name, icon)
            VALUES ($1, $2)
            RETURNING id, name, icon, created_at, updated_at
            "#,
            self.name,
            self.icon
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
pub struct TagUpdate {
    name: Option<String>,
    icon: Option<String>,
}

impl TagUpdate {
    pub async fn update(&self, db_pool: &Pool<Postgres>, id: Uuid) -> Result<Tag> {
        sqlx::query_as!(
            Tag,
            r#"
            UPDATE tag
            SET
                name = coalesce($2, name),
                icon = coalesce($3, icon),
                updated_at = $4
            WHERE id = $1
            RETURNING id, name, icon, created_at, updated_at
            "#,
            id,
            self.name,
            self.icon,
            Local::now().naive_local()
        )
        .fetch_optional(db_pool)
        .await
        .map_err(|_| SharedError::Internal.extend())?
        .ok_or_else(|| TagError::NotFound.extend())
    }
}
