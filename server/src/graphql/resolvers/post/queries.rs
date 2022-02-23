use super::models::Post;
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
pub struct PostQuery;

#[Object]
impl PostQuery {
    pub async fn read_posts(
        &self,
        ctx: &Context<'_>,
        params: PaginationParams,
    ) -> Result<Connection<Base64Cursor, Post, ConnectionFields, EmptyFields>> {
        let state = ctx.data::<Arc<State>>()?;

        query_connection(params, &state.db_pool, Post::read_count, Post::read_many).await
    }

    pub async fn read_post(&self, ctx: &Context<'_>, id: Uuid) -> Result<Post> {
        let state = ctx.data::<Arc<State>>()?;

        Post::read_one(&state.db_pool, id).await
    }
}
