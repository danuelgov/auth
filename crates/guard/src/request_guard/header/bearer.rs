use crate::Credential;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bearer(String);

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
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("Bearer ") {
            Ok(Bearer(s[7..].to_string()))
        } else {
            Err(())
        }
    }
}

impl Credential for Bearer {
    //
}
