use async_graphql::{Error, ErrorExtensions};

#[derive(Debug, thiserror::Error)]
pub enum PostError {
    #[error("Post not found")]
    NotFound,
}

impl ErrorExtensions for PostError {
    fn extend(&self) -> async_graphql::Error {
        Error::new(self.to_string()).extend_with(|_, v| match self {
            Self::NotFound => v.set("code", "404"),
        })
    }
}
