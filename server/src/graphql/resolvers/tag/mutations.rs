use super::{Tag, TagCreate, TagUpdate};
use crate::State;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;

#[derive(Default)]
pub struct TagMutation;

#[Object]
impl TagMutation {
    async fn create_tag(&self, ctx: &Context<'_>, tag: TagCreate) -> Result<Tag> {
        let state = ctx.data::<Arc<State>>()?;

        tag.create(&state.db_pool).await
    }

    async fn update_tag(&self, ctx: &Context<'_>, id: i32, tag: TagUpdate) -> Result<Tag> {
        let state = ctx.data::<Arc<State>>()?;

        tag.update(&state.db_pool, id).await
    }

    async fn delete_tag(&self, ctx: &Context<'_>, id: i32) -> Result<Tag> {
        let state = ctx.data::<Arc<State>>()?;

        Tag::delete(&state.db_pool, id).await
    }
}
