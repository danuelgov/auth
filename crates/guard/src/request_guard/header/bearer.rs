use crate::Credential;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bearer(String);

#[derive(Debug)]
pub enum BearerError {
    InvalidType,
}

impl std::ops::Deref for Bearer {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Bearer {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::str::FromStr for Bearer {
    type Err = BearerError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        match source.strip_prefix("Bearer ") {
            Some(source) => Ok(Bearer(source.to_string())),
            None => Err(BearerError::InvalidType),
        }
    }
}

impl Credential for Bearer {
    //
}
