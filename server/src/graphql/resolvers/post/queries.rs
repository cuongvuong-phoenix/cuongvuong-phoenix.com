use super::models::Post;
use crate::{
    graphql::shared::relay::{
        create_connection, get_start_end_cursor_offsets, query_connection, Base64Cursor,
        ConnectionFields, PaginationParams,
    },
    State,
};
use async_graphql::{
    connection::{self, Connection, EmptyFields},
    Context, Error, Object, Result,
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
                &state.db_pool,
                Post::read_count,
                Post::read_many,
            )
            .await
        } else {
            let PaginationParams {
                after,
                before,
                first,
                last,
            } = pagination_params;

            connection::query::<Base64Cursor, Post, ConnectionFields, _, _, _, Error>(
                after,
                before,
                first,
                last,
                |after, before, first, last| async move {
                    let count = Post::read_count_by_tags(&state.db_pool, &tag_ids).await?;

                    let (start, end) =
                        get_start_end_cursor_offsets(after, before, first, last, count);

                    let posts = Post::read_many_by_tags(
                        &state.db_pool,
                        &tag_ids,
                        (end - start) as i64,
                        start as i64,
                    )
                    .await?;

                    Ok(create_connection(start, end, count, posts.into_iter()))
                },
            )
            .await
        }
    }

    async fn post(&self, ctx: &Context<'_>, slug: String) -> Result<Post> {
        let state = ctx.data::<Arc<State>>()?;

        Post::read_one_by_slug(&state.db_pool, slug).await
    }
}
