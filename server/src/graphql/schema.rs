use super::resolvers::{PostMutation, PostQuery};
use crate::State;
use async_graphql::{EmptySubscription, MergedObject, Schema};
use std::sync::Arc;

#[derive(Default, MergedObject)]
pub struct Query(PostQuery);

#[derive(Default, MergedObject)]
pub struct Mutation(PostMutation);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn init_schema(state: Arc<State>) -> AppSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(state)
        .finish()
}
