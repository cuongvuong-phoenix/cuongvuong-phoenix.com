use async_graphql::connection::CursorType;

#[derive(Debug, thiserror::Error)]
pub enum Base64CursorError {
    #[error("Invalid cursor")]
    Invalid,
    #[error("Error while decoing cursor")]
    Decode(#[from] base64::DecodeError),
}

pub struct Base64Cursor {
    id: &'static str,
    pub offset: usize,
}

impl Base64Cursor {
    pub fn new(offset: usize) -> Self {
        Self {
            id: "cursor",
            offset,
        }
    }

    fn encode(&self) -> String {
        base64::encode_config(format!("{}:{}", self.id, self.offset), base64::URL_SAFE)
    }

    fn decode(s: &str) -> Result<Self, Base64CursorError> {
        let bytes = base64::decode_config(s, base64::URL_SAFE)?;

        let cursor = String::from_utf8(bytes).map_err(|_| Base64CursorError::Invalid)?;
        let offset = cursor
            .split(":")
            .into_iter()
            .last()
            .ok_or_else(|| Base64CursorError::Invalid)?
            .parse::<usize>()
            .map_err(|_| Base64CursorError::Invalid)?;

        Ok(Self::new(offset))
    }
}

impl CursorType for Base64Cursor {
    type Error = Base64CursorError;

    fn decode_cursor(s: &str) -> Result<Self, Self::Error> {
        Self::decode(s)
    }

    fn encode_cursor(&self) -> String {
        self.encode()
    }
}
