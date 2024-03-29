use super::PostError;
use crate::graphql::shared::errors::SharedError;
use async_graphql::{ErrorExtensions, InputObject, Result, SimpleObject};
use sqlx::{FromRow, Pool, Postgres, Transaction};
use time::OffsetDateTime;

#[derive(SimpleObject, FromRow)]
#[graphql(complex)]
pub struct Post {
    pub id: i32,
    title: String,
    slug: String,
    reading_time: i32,
    visible: bool,
    created_at: OffsetDateTime,
    updated_at: Option<OffsetDateTime>,
}

impl Post {
    // ----------------------------------------------------------------
    // read_count
    // ----------------------------------------------------------------
    pub async fn read_count(
        db_pool: &Pool<Postgres>,
        search: &str,
        tag_ids: &[i32],
    ) -> Result<usize> {
        match (search.is_empty(), tag_ids.len()) {
            (true, 0) => Post::read_count_no_filter(db_pool).await,
            (false, 0) => Post::read_count_by_search(db_pool, search).await,
            (true, _) => Post::read_count_by_tags(db_pool, tag_ids).await,
            (false, _) => Post::read_count_by_search_n_tags(db_pool, search, tag_ids).await,
        }
    }

    pub async fn read_count_no_filter(db_pool: &Pool<Postgres>) -> Result<usize> {
        Ok(sqlx::query!(r#"SELECT COUNT(*) AS "count!" FROM post"#)
            .fetch_one(db_pool)
            .await
            .map_err(|e| SharedError::Database(e).extend())?
            .count as usize)
    }

    pub async fn read_count_by_search(db_pool: &Pool<Postgres>, search: &str) -> Result<usize> {
        Ok(sqlx::query!(
            r#"
            SELECT COUNT(*) AS "count!"
            FROM post
            WHERE title ILIKE ('%' || $1 || '%')"#,
            search
        )
        .fetch_one(db_pool)
        .await
        .map_err(|e| SharedError::Database(e).extend())?
        .count as usize)
    }

    pub async fn read_count_by_tags(db_pool: &Pool<Postgres>, tag_ids: &[i32]) -> Result<usize> {
        Ok(sqlx::query!(
            r#"
            SELECT COUNT(*) AS "count!"
            FROM post p
                JOIN post_has_tag pht ON p.id = pht.post_id
            WHERE pht.tag_id = ANY($1)
            "#,
            tag_ids
        )
        .fetch_one(db_pool)
        .await
        .map_err(|e| SharedError::Database(e).extend())?
        .count as usize)
    }

    pub async fn read_count_by_search_n_tags(
        db_pool: &Pool<Postgres>,
        search: &str,
        tag_ids: &[i32],
    ) -> Result<usize> {
        Ok(sqlx::query!(
            r#"
            SELECT COUNT(*) AS "count!"
            FROM post p
                JOIN post_has_tag pht ON p.id = pht.post_id
            WHERE title ILIKE ('%' || $1 || '%') AND pht.tag_id = ANY($2)
            "#,
            search,
            tag_ids
        )
        .fetch_one(db_pool)
        .await
        .map_err(|e| SharedError::Database(e).extend())?
        .count as usize)
    }

    // ----------------------------------------------------------------
    // read_many
    // ----------------------------------------------------------------
    pub async fn read_many(
        db_pool: &Pool<Postgres>,
        limit: i64,
        offset: i64,
        search: &str,
        tag_ids: &[i32],
    ) -> Result<Vec<Self>> {
        match (search.is_empty(), tag_ids.len()) {
            (true, 0) => Post::read_many_no_filter(db_pool, limit, offset).await,
            (false, 0) => Post::read_many_by_search(db_pool, limit, offset, search).await,
            (true, _) => Post::read_many_by_tags(db_pool, limit, offset, tag_ids).await,
            (false, _) => {
                Post::read_many_by_search_n_tags(db_pool, limit, offset, search, tag_ids).await
            }
        }
    }

    pub async fn read_many_no_filter(
        db_pool: &Pool<Postgres>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Post>> {
        sqlx::query_as!(
            Post,
            r#"
            SELECT id, title, slug, reading_time, visible, created_at, updated_at 
            FROM post
            ORDER BY
                coalesce(updated_at, created_at) DESC,
                reading_time DESC,
                title ASC
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

    pub async fn read_many_by_search(
        db_pool: &Pool<Postgres>,
        limit: i64,
        offset: i64,
        search: &str,
    ) -> Result<Vec<Post>> {
        sqlx::query_as!(
            Post,
            r#"
            SELECT id, title, slug, reading_time, visible, created_at, updated_at 
            FROM post
            WHERE title ILIKE ('%' || $3 || '%')
            ORDER BY
                coalesce(updated_at, created_at) DESC,
                reading_time DESC,
                title ASC
            LIMIT $1
            OFFSET $2
            "#,
            limit,
            offset,
            search
        )
        .fetch_all(db_pool)
        .await
        .map_err(|e| SharedError::Database(e).extend())
    }

    pub async fn read_many_by_tags(
        db_pool: &Pool<Postgres>,
        limit: i64,
        offset: i64,
        tag_ids: &[i32],
    ) -> Result<Vec<Post>> {
        sqlx::query_as!(
            Post,
            r#"
            SELECT p.id, p.title, p.slug, p.reading_time, p.visible, p.created_at, p.updated_at
            FROM post p
                JOIN post_has_tag pht ON p.id = pht.post_id
            WHERE pht.tag_id = ANY($3)
            ORDER BY
                coalesce(p.updated_at, p.created_at) DESC,
                p.reading_time DESC,
                p.title ASC
            LIMIT $1
            OFFSET $2
            "#,
            limit,
            offset,
            tag_ids
        )
        .fetch_all(db_pool)
        .await
        .map_err(|e| SharedError::Database(e).extend())
    }

    pub async fn read_many_by_search_n_tags(
        db_pool: &Pool<Postgres>,
        limit: i64,
        offset: i64,
        search: &str,
        tag_ids: &[i32],
    ) -> Result<Vec<Post>> {
        sqlx::query_as!(
            Post,
            r#"
            SELECT p.id, p.title, p.slug, p.reading_time, p.visible, p.created_at, p.updated_at
            FROM post p
                JOIN post_has_tag pht ON p.id = pht.post_id
            WHERE title ILIKE ('%' || $3 || '%') AND pht.tag_id = ANY($4)
            ORDER BY
                coalesce(p.updated_at, p.created_at) DESC,
                p.reading_time DESC,
                p.title ASC
            LIMIT $1
            OFFSET $2
            "#,
            limit,
            offset,
            search,
            tag_ids
        )
        .fetch_all(db_pool)
        .await
        .map_err(|e| SharedError::Database(e).extend())
    }

    // ----------------------------------------------------------------
    // read_one
    // ----------------------------------------------------------------
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

    pub async fn delete_one(db_pool: &Pool<Postgres>, id: i32) -> Result<Post> {
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
        id: i32,
        tag_ids: &[i32],
    ) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO post_has_tag(post_id, tag_id)
            SELECT $1 AS post_id, tag_id
            FROM unnest($2::INTEGER[]) tag_id;
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
    tag_ids: Vec<i32>,
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
    tag_ids: Option<Vec<i32>>,
}

impl PostUpdate {
    pub async fn update(&self, db_pool: &Pool<Postgres>, id: i32) -> Result<Post> {
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
            OffsetDateTime::now_local()?,
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
