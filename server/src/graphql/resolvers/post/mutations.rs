use super::{Post, PostCreate, PostUpdate};
use crate::State;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;

#[derive(Default)]
pub struct PostMutation;

#[Object]
impl PostMutation {
    async fn create_post(&self, ctx: &Context<'_>, post: PostCreate) -> Result<Post> {
        let state = ctx.data::<Arc<State>>()?;

        post.create(&state.db_pool).await
    }

    async fn update_post(&self, ctx: &Context<'_>, id: i32, post: PostUpdate) -> Result<Post> {
        let state = ctx.data::<Arc<State>>()?;

        post.update(&state.db_pool, id).await
    }

    async fn delete_post(&self, ctx: &Context<'_>, id: i32) -> Result<Post> {
        let state = ctx.data::<Arc<State>>()?;

        Post::delete_one(&state.db_pool, id).await
    }
}
