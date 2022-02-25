use crate::{graphql::resolvers::Tag, State};
use async_graphql::{dataloader::Loader, Result};
use async_trait::async_trait;
use itertools::Itertools;
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

// ----------------------------------------------------------------
// PostContentLoader
// ----------------------------------------------------------------
pub struct PostContentLoader {
    state: Arc<State>,
}

impl PostContentLoader {
    pub fn new(state: Arc<State>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl Loader<Uuid> for PostContentLoader {
    type Value = String;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        Ok(sqlx::query!(
            r#"
            SELECT id, content
            FROM post
            WHERE id = ANY($1)
            "#,
            keys
        )
        .fetch_all(&self.state.db_pool)
        .await?
        .into_iter()
        .map(|record| (record.id, record.content))
        .collect())
    }
}

// ----------------------------------------------------------------
// PostTagsLoader
// ----------------------------------------------------------------
pub struct PostTagsLoader {
    state: Arc<State>,
}

impl PostTagsLoader {
    pub fn new(state: Arc<State>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl Loader<Uuid> for PostTagsLoader {
    type Value = Vec<Tag>;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        Ok(sqlx::query!(
            r#"
            SELECT pht.post_id, t.id, t.name, t.icon, t.created_at, t.updated_at
            FROM post_has_tag pht JOIN tag t ON pht.tag_id = t.id
            WHERE pht.post_id = ANY($1)
            ORDER BY
                coalesce(pht.updated_at, pht.created_at) DESC,
                t.name ASC;
            "#,
            keys
        )
        .fetch_all(&self.state.db_pool)
        .await?
        .into_iter()
        .map(|record| {
            (
                record.post_id,
                Tag::new(
                    record.id,
                    record.name,
                    record.icon,
                    record.created_at,
                    record.updated_at,
                ),
            )
        })
        .into_group_map())
    }
}
