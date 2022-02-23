use super::{Post, PostCreate, PostUpdate};
use crate::State;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Default)]
pub struct PostMutation;

#[Object]
impl PostMutation {
    pub async fn create_post(&self, ctx: &Context<'_>, post: PostCreate) -> Result<Post> {
        let state = ctx.data::<Arc<State>>()?;

        post.create(&state.db_pool).await
    }

    pub async fn update_post(&self, ctx: &Context<'_>, id: Uuid, post: PostUpdate) -> Result<Post> {
        let state = ctx.data::<Arc<State>>()?;

        post.update(&state.db_pool, id).await
    }

    pub async fn delete_post(&self, ctx: &Context<'_>, id: Uuid) -> Result<Post> {
        let state = ctx.data::<Arc<State>>()?;

        Post::delete_one(&state.db_pool, id).await
    }
}
