use crate::{Bearer, Credential, HeaderName};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Authorization<T: Credential>(T);

impl<T> std::ops::Deref for Authorization<T>
where
    T: Credential,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> std::str::FromStr for Authorization<T>
where
    T: std::str::FromStr + Credential,
{
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Authorization(s.parse()?))
    }
}

impl<T> HeaderName for Authorization<T>
where
    T: Send + Credential,
{
    const NAME: &'static str = "Authorization";
}

impl Authorization<Bearer> {
    #[inline]
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}
