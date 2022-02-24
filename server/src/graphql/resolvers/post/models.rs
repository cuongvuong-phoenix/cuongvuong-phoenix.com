use crate::graphql::shared::errors::SharedError;
use async_graphql::{ErrorExtensions, InputObject, Result, SimpleObject};
use chrono::{Local, NaiveDateTime};
use sqlx::{Pool, Postgres, Transaction};
use uuid::Uuid;

use super::PostError;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Post {
    pub id: Uuid,
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
            .map_err(|e| SharedError::Database(e).extend())?
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
        .map_err(|e| SharedError::Database(e).extend())
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
        .map_err(|e| SharedError::Database(e).extend())?
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
        .map_err(|e| SharedError::Database(e).extend())?
        .ok_or_else(|| PostError::NotFound.extend())
    }

    pub async fn create_tags_transaction(
        transaction: &mut Transaction<'_, Postgres>,
        id: Uuid,
        tag_ids: &[Uuid],
    ) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO post_has_tag(post_id, tag_id)
            SELECT $1 AS post_id, tag_id
            FROM unnest($2::UUID[]) tag_id;
            "#,
            id,
            &tag_ids
        )
        .execute(transaction)
        .await
        .map_err(|e| SharedError::Database(e).extend())?;

        Ok(())
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
    tag_ids: Vec<Uuid>,
}

impl PostCreate {
    pub async fn create(&self, db_pool: &Pool<Postgres>) -> Result<Post> {
        let mut transaction = db_pool
            .begin()
            .await
            .map_err(|e| SharedError::Database(e).extend())?;

        let post = sqlx::query_as!(
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
        .fetch_one(&mut transaction)
        .await
        .map_err(|e| SharedError::Database(e).extend())?;

        Post::create_tags_transaction(&mut transaction, post.id, &self.tag_ids).await?;

        transaction
            .commit()
            .await
            .map_err(|e| SharedError::Database(e).extend())?;

        Ok(post)
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
    tag_ids: Option<Vec<Uuid>>,
}

impl PostUpdate {
    pub async fn update(&self, db_pool: &Pool<Postgres>, id: Uuid) -> Result<Post> {
        let mut transaction = db_pool
            .begin()
            .await
            .map_err(|e| SharedError::Database(e).extend())?;

        let post = sqlx::query_as!(
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
        .fetch_optional(&mut transaction)
        .await
        .map_err(|e| SharedError::Database(e).extend())?
        .ok_or_else(|| PostError::NotFound.extend())?;

        if let Some(tag_ids) = &self.tag_ids {
            sqlx::query!(
                r#"
                DELETE FROM post_has_tag
                WHERE post_id = $1;
                "#,
                id
            )
            .execute(&mut transaction)
            .await
            .map_err(|e| SharedError::Database(e).extend())?;

            Post::create_tags_transaction(&mut transaction, id, tag_ids).await?;
        }

        transaction
            .commit()
            .await
            .map_err(|e| SharedError::Database(e).extend())?;

        Ok(post)
    }
}
