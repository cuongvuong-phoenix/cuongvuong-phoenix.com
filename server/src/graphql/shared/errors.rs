use async_graphql::{Error, ErrorExtensions};

#[derive(Debug, thiserror::Error)]
pub enum SharedError {
    #[error("Database Error")]
    Database(#[from] sqlx::Error),
}

impl ErrorExtensions for SharedError {
    fn extend(&self) -> async_graphql::Error {
        Error::new(self.to_string()).extend_with(|_, v| match self {
            SharedError::Database(error) => {
                tracing::debug!("{}", error);
                v.set("code", "500")
            }
        })
    }
}
