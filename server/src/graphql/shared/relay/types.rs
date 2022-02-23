use async_graphql::{InputObject, SimpleObject};

#[derive(SimpleObject)]
pub struct ConnectionFields {
    total_count: usize,
}

impl ConnectionFields {
    pub fn new(total_count: usize) -> Self {
        Self { total_count }
    }
}

#[derive(InputObject)]
pub struct PaginationParams {
    pub after: Option<String>,
    pub before: Option<String>,
    pub first: Option<i32>,
    pub last: Option<i32>,
}
