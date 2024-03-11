use crate::Credential;
use base64::{engine::general_purpose::URL_SAFE, Engine};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Basic {
    id: String,
    password: String,
}

#[derive(Debug)]
pub enum BasicError {
    InvalidType,
    InvalidEncoding,
    InvalidUtf8,
    NoId,
    NoPassword,
}

impl std::str::FromStr for Basic {
    type Err = BasicError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let Some(source) = source.strip_prefix("Basic ") else {
            return Err(BasicError::InvalidType);
        };
        let Ok(source) = URL_SAFE.decode(source) else {
            return Err(BasicError::InvalidEncoding);
        };
        let Ok(source) = std::str::from_utf8(&source) else {
            return Err(BasicError::InvalidUtf8);
        };
        let mut parts = source.splitn(2, ':');
        let Some(id) = parts.next() else {
            return Err(BasicError::NoId);
        };
        let Some(password) = parts.next() else {
            return Err(BasicError::NoPassword);
        };

        Ok(Basic {
            id: id.to_owned(),
            password: password.to_owned(),
        })
    }
}

impl Credential for Basic {
    //
}

impl Basic {
    #[inline]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[inline]
    pub fn password(&self) -> &str {
        &self.password
    }
}
