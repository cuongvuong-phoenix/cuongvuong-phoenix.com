use super::{HomeContent, HomeContentCreate, HomeContentUpdate};
use crate::State;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;

#[derive(Default)]
pub struct HomeMutation;

#[Object]
impl HomeMutation {
    async fn create_home_content(
        &self,
        ctx: &Context<'_>,
        home: HomeContentCreate,
    ) -> Result<HomeContent> {
        let state = ctx.data::<Arc<State>>()?;

        home.create(&state.db_pool).await
    }

    async fn update_home_content(
        &self,
        ctx: &Context<'_>,
        home: HomeContentUpdate,
    ) -> Result<HomeContent> {
        let state = ctx.data::<Arc<State>>()?;

        home.update(&state.db_pool).await
    }
}
