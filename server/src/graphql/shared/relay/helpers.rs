use super::{Base64Cursor, ConnectionFields, PaginationParams};
use async_graphql::{
    connection::{self, Connection, Edge, EmptyFields},
    Error, Result,
};
use sqlx::{Pool, Postgres};
use std::future::Future;

pub const DEFAULT_PAGE_SIZE: usize = 8;

pub async fn query_connection<'a, N, CF, CFR, VF, VFR>(
    params: PaginationParams,
    db_pool: &'a Pool<Postgres>,
    count_fn: CF,
    vec_fn: VF,
) -> Result<Connection<Base64Cursor, N, ConnectionFields, EmptyFields>>
where
    CF: FnOnce(&'a Pool<Postgres>) -> CFR,
    CFR: Future<Output = Result<usize>>,
    VF: FnOnce(&'a Pool<Postgres>, i64, i64) -> VFR,
    VFR: Future<Output = Result<Vec<N>>>,
{
    let PaginationParams {
        after,
        before,
        first,
        last,
    } = params;

    connection::query::<Base64Cursor, N, ConnectionFields, _, _, _, Error>(
        after,
        before,
        first,
        last,
        |after, before, first, last| async move {
            let count = count_fn(db_pool).await?;

            let (start, end) = get_start_end_cursor_offsets(after, before, first, last, count);

            let posts = vec_fn(db_pool, (end - start) as i64, start as i64).await?;

            Ok(create_connection(start, end, count, posts.into_iter()))
        },
    )
    .await
}

pub fn get_start_end_cursor_offsets(
    after: Option<Base64Cursor>,
    before: Option<Base64Cursor>,
    first: Option<usize>,
    last: Option<usize>,
    count: usize,
) -> (usize, usize) {
    let start_offset = after.map(|cursor| cursor.offset + 1).unwrap_or(0);
    let end_offset = before.map(|cursor| cursor.offset).unwrap_or(count);

    match (first, last) {
        (Some(first), _) => (start_offset, (start_offset + first).min(end_offset)),
        (_, Some(last)) => ((end_offset - last).max(start_offset), end_offset),
        _ => (start_offset, (start_offset + DEFAULT_PAGE_SIZE)),
    }
}

pub fn create_connection<N, I: ExactSizeIterator<Item = N>>(
    start: usize,
    end: usize,
    count: usize,
    iter: I,
) -> Connection<Base64Cursor, N, ConnectionFields, EmptyFields> {
    let mut connection =
        Connection::with_additional_fields(start > 0, end < count, ConnectionFields::new(count));

    connection.append(
        (start..end)
            .into_iter()
            .zip(iter)
            .map(|(offset, node)| Edge::new(Base64Cursor::new(offset), node)),
    );

    connection
}
