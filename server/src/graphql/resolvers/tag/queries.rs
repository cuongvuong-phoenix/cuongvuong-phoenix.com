use super::Tag;
use crate::{
    graphql::shared::relay::{query_connection, Base64Cursor, ConnectionFields, PaginationParams},
    State,
};
use async_graphql::{
    connection::{Connection, EmptyFields},
    Context, Object, Result,
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Default)]
pub struct TagQuery;

#[Object]
impl TagQuery {
    async fn tags(
        &self,
        ctx: &Context<'_>,
        pagination_params: PaginationParams,
    ) -> Result<Connection<Base64Cursor, Tag, ConnectionFields, EmptyFields>> {
        let state = ctx.data::<Arc<State>>()?;

        query_connection(
            pagination_params,
            &state.db_pool,
            Tag::read_count,
            Tag::read_many,
        )
        .await
    }

    async fn tag(&self, ctx: &Context<'_>, id: Uuid) -> Result<Tag> {
        let state = ctx.data::<Arc<State>>()?;

        Tag::read_one(&state.db_pool, id).await
    }
}
