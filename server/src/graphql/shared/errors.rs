use async_graphql::{Error, ErrorExtensions};

#[derive(Debug, thiserror::Error)]
pub enum SharedError {
    #[error("Internal Server Error")]
    Internal,
}

impl ErrorExtensions for SharedError {
    fn extend(&self) -> async_graphql::Error {
        Error::new(self.to_string()).extend_with(|_, v| match self {
            Self::Internal => v.set("code", "500"),
        })
    }
}
