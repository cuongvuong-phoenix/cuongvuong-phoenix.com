use crate::{graphql::resolvers::Tag, State};
use async_graphql::{dataloader::Loader, Result};
use async_trait::async_trait;
use itertools::Itertools;
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

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
                coalesce(t.updated_at, t.created_at) DESC;
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
