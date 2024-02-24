use crate::{Hash, Hasher, Salt};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Password(String);

impl std::fmt::Display for Password {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Password(MASKED, len = {})", self.len())
    }
}

impl std::fmt::Debug for Password {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Password(MASKED, len = {})", self.len())
    }
}

impl std::str::FromStr for Password {
    type Err = std::convert::Infallible;

    #[inline]
    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Ok(Self(source.to_owned()))
    }
}

impl std::ops::Deref for Password {
    type Target = String;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Password {
    #[inline]
    pub async fn hash(&self, hasher: Hasher, salt: &Salt) -> Result<Hash, crate::HasherError> {
        hasher.hash(self.as_bytes(), salt).await
    }

    #[inline]
    pub async fn verify(&self, hasher: Hasher, hash: &Hash) -> Result<bool, crate::HasherError> {
        hasher.verify(self.as_bytes(), hash).await
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
