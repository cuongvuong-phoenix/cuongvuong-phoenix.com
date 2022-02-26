use super::PostError;
use crate::graphql::shared::errors::SharedError;
use async_graphql::{ErrorExtensions, InputObject, Result, SimpleObject};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Transaction};
use uuid::Uuid;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Post {
    pub id: Uuid,
    title: String,
    slug: String,
    reading_time: i32,
    visible: bool,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
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

    pub async fn read_count_by_tags(db_pool: &Pool<Postgres>, tag_ids: &[Uuid]) -> Result<usize> {
        Ok(sqlx::query!(
            r#"
            SELECT COUNT(*) AS "count!"
            FROM
                post p
                JOIN post_has_tag pht ON p.id = pht.post_id
            WHERE pht.tag_id = ANY($1);
            "#,
            tag_ids
        )
        .fetch_one(db_pool)
        .await
        .map_err(|e| SharedError::Database(e).extend())?
        .count as usize)
    }

    pub async fn read_many_by_tags(
        db_pool: &Pool<Postgres>,
        tag_ids: &[Uuid],
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Post>> {
        sqlx::query_as!(
            Post,
            r#"
            SELECT p.id, p.title, p.slug, p.reading_time, p.visible, p.created_at, p.updated_at
            FROM
                post p
                JOIN post_has_tag pht ON p.id = pht.post_id
            WHERE pht.tag_id = ANY($1)
            ORDER BY
                coalesce(p.updated_at, p.created_at) DESC,
                p.reading_time DESC,
                p.title ASC
            LIMIT $2
            OFFSET $3;
            "#,
            &tag_ids,
            limit,
            offset
        )
        .fetch_all(db_pool)
        .await
        .map_err(|e| SharedError::Database(e).extend())
    }

    pub async fn read_one_by_slug(db_pool: &Pool<Postgres>, slug: String) -> Result<Post> {
        sqlx::query_as!(
            Post,
            r#"
            SELECT id, title, slug, reading_time, visible, created_at, updated_at
            FROM post
            WHERE slug = $1
            "#,
            slug
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
    reading_time: i32,
    visible: bool,
    content: String,
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
            INSERT INTO post(title, slug, reading_time, visible, content)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, title, slug, reading_time, visible, created_at, updated_at
            "#,
            self.title,
            self.slug,
            self.reading_time,
            self.visible,
            self.content
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
    content: Option<String>,
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
                content = coalesce($6, content),
                updated_at = $7
            WHERE id = $1
            RETURNING id, title, slug, reading_time, visible, created_at, updated_at;
            "#,
            id,
            self.title,
            self.slug,
            self.reading_time,
            self.visible,
            self.content,
            Utc::now()
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
