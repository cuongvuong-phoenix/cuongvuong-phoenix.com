use super::resolvers::{PostMutation, PostQuery, TagMutation, TagQuery};
use crate::State;
use async_graphql::{EmptySubscription, MergedObject, Schema};
use std::sync::Arc;

#[derive(Default, MergedObject)]
pub struct Query(PostQuery, TagQuery);

#[derive(Default, MergedObject)]
pub struct Mutation(PostMutation, TagMutation);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn init_schema(state: Arc<State>) -> AppSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(state)
        .finish()
}
