use super::{Post, PostContentLoader, PostTagsLoader};
use crate::graphql::resolvers::Tag;
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Result};

#[ComplexObject]
impl Post {
    async fn content(&self, ctx: &Context<'_>) -> Result<String> {
        let loader = ctx.data::<DataLoader<PostContentLoader>>()?;

        Ok(loader
            .load_one(self.id)
            .await?
            .unwrap_or_else(|| String::default()))
    }

    async fn tags(&self, ctx: &Context<'_>) -> Result<Vec<Tag>> {
        let loader = ctx.data::<DataLoader<PostTagsLoader>>()?;

        Ok(loader.load_one(self.id).await?.unwrap_or_else(|| vec![]))
    }
}
