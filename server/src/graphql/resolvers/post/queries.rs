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
use uuid::Uuid;

#[derive(Default)]
pub struct PostQuery;

#[Object]
impl PostQuery {
    async fn posts(
        &self,
        ctx: &Context<'_>,
        tag_ids: Option<Vec<Uuid>>,
        pagination_params: PaginationParams,
    ) -> Result<Connection<Base64Cursor, Post, ConnectionFields, EmptyFields>> {
        let state = ctx.data::<Arc<State>>()?;

        match tag_ids {
            None => {
                query_connection(
                    pagination_params,
                    &state.db_pool,
                    Post::read_count,
                    Post::read_many,
                )
                .await
            }
            Some(tag_ids) => {
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
    }

    async fn post(&self, ctx: &Context<'_>, id: Uuid) -> Result<Post> {
        let state = ctx.data::<Arc<State>>()?;

        Post::read_one(&state.db_pool, id).await
    }
}
