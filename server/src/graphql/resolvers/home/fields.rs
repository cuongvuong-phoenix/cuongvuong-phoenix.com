use super::HomeContent;
use crate::{graphql::resolvers::Post, State};
use async_graphql::{ComplexObject, Context, Result};
use std::sync::Arc;

#[ComplexObject]
impl HomeContent {
    async fn num_blog_posts(&self, ctx: &Context<'_>) -> Result<usize> {
        let state = ctx.data::<Arc<State>>()?;

        Post::read_count(&state.db_pool).await
    }
}
