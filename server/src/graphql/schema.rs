use super::resolvers::{
    HomeMutation, HomeQuery, PostContentLoader, PostMutation, PostQuery, PostTagsLoader,
    TagMutation, TagQuery,
};
use crate::State;
use async_graphql::{dataloader::DataLoader, EmptySubscription, MergedObject, Schema};
use std::sync::Arc;

#[derive(Default, MergedObject)]
pub struct Query(HomeQuery, PostQuery, TagQuery);

#[derive(Default, MergedObject)]
pub struct Mutation(HomeMutation, PostMutation, TagMutation);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn init_schema(state: Arc<State>) -> AppSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(state.clone())
        .data(DataLoader::new(
            PostTagsLoader::new(state.clone()),
            tokio::spawn,
        ))
        .data(DataLoader::new(
            PostContentLoader::new(state.clone()),
            tokio::spawn,
        ))
        .finish()
}
