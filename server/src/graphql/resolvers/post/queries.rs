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

#[derive(Default)]
pub struct PostQuery;

#[Object]
impl PostQuery {
    async fn posts_count(&self, ctx: &Context<'_>) -> Result<usize> {
        let state = ctx.data::<Arc<State>>()?;

        Post::read_count(&state.db_pool).await
    }

    async fn posts(
        &self,
        ctx: &Context<'_>,
        tag_ids: Vec<i32>,
        pagination_params: PaginationParams,
    ) -> Result<Connection<Base64Cursor, Post, ConnectionFields, EmptyFields>> {
        let state = ctx.data::<Arc<State>>()?;

        if tag_ids.len() == 0 {
            query_connection(
                pagination_params,
                || Post::read_count(&state.db_pool),
                |limit, offset| Post::read_many(&state.db_pool, limit, offset),
            )
            .await
        } else {
            query_connection(
                pagination_params,
                || Post::read_count_by_tags(&state.db_pool, &tag_ids),
                |limit, offset| Post::read_many_by_tags(&state.db_pool, &tag_ids, limit, offset),
            )
            .await
        }
    }

    async fn post(&self, ctx: &Context<'_>, slug: String) -> Result<Post> {
        let state = ctx.data::<Arc<State>>()?;

        Post::read_one_by_slug(&state.db_pool, slug).await
    }
}
