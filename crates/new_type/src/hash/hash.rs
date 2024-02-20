#[derive(PartialEq, Eq)]
pub struct Hash(String);

impl std::fmt::Display for Hash {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hash(MASKED)")
    }
}

impl std::fmt::Debug for Hash {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hash(MASKED)")
    }
}

impl From<String> for Hash {
    #[inline]
    fn from(hash: String) -> Self {
        Self(hash)
    }
}

impl From<&str> for Hash {
    #[inline]
    fn from(hash: &str) -> Self {
        Self(hash.to_owned())
    }
}

impl std::ops::Deref for Hash {
    type Target = String;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Hash {
    #[inline]
    pub const fn new(hash: String) -> Self {
        Self(hash)
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
