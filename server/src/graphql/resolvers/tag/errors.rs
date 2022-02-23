use async_graphql::{Error, ErrorExtensions};

#[derive(Debug, thiserror::Error)]
pub enum TagError {
    #[error("Tag not found")]
    NotFound,
}

impl ErrorExtensions for TagError {
    fn extend(&self) -> async_graphql::Error {
        Error::new(self.to_string()).extend_with(|_, v| match self {
            Self::NotFound => v.set("code", "404"),
        })
    }
}
