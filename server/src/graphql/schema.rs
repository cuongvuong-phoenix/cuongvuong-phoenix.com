use crate::State;
use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};
use std::sync::Arc;

#[derive(Default, MergedObject)]
pub struct Query();

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn init_schema(state: Arc<State>) -> AppSchema {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(state)
        .finish()
}
