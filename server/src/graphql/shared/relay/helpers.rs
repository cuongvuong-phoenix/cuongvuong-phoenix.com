use super::{Base64Cursor, ConnectionFields, PaginationParams};
use async_graphql::{
    connection::{self, Connection, Edge, EmptyFields},
    Error, OutputType, Result,
};
use std::future::Future;

pub async fn query_connection<N, CF, CFR, VF, VFR>(
    pagination_params: PaginationParams,
    count_fn: CF,
    vec_fn: VF,
) -> Result<Connection<Base64Cursor, N, ConnectionFields, EmptyFields>>
where
    N: OutputType,
    CF: FnOnce() -> CFR,
    CFR: Future<Output = Result<usize>>,
    VF: FnOnce(i64, i64) -> VFR,
    VFR: Future<Output = Result<Vec<N>>>,
{
    let PaginationParams {
        after,
        before,
        first,
        last,
    } = pagination_params;

    connection::query::<_, _, Base64Cursor, N, ConnectionFields, _, _, _, Error>(
        after,
        before,
        first,
        last,
        |after, before, first, last| async move {
            let count = count_fn().await?;

            let (start, end) = get_start_end_cursor_offsets(after, before, first, last, count);

            let nodes = vec_fn((end - start) as i64, start as i64).await?;

            Ok(create_connection(start, end, count, nodes.into_iter()))
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
        _ => (start_offset, end_offset),
    }
}

pub fn create_connection<N, I>(
    start: usize,
    end: usize,
    count: usize,
    iterator: I,
) -> Connection<Base64Cursor, N, ConnectionFields>
where
    N: OutputType,
    I: ExactSizeIterator<Item = N>,
{
    let mut connection =
        Connection::with_additional_fields(start > 0, end < count, ConnectionFields::new(count));

    connection.edges.extend(
        (start..end)
            .into_iter()
            .zip(iterator)
            .map(|(offset, node)| Edge::new(Base64Cursor::new(offset), node)),
    );

    connection
}
