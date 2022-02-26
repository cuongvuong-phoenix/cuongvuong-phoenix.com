use super::HomeContent;
use crate::State;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;

#[derive(Default)]
pub struct HomeQuery;

#[Object]
impl HomeQuery {
    async fn home_content(&self, ctx: &Context<'_>) -> Result<HomeContent> {
        let state = ctx.data::<Arc<State>>()?;

        HomeContent::read(&state.db_pool).await
    }
}
