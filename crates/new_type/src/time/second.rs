use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Second(u32);

#[derive(Debug)]
pub enum SecondError {
    Format(<u32 as std::str::FromStr>::Err),
    Range,
}

impl std::ops::Deref for Second {
    type Target = u32;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Second {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

impl std::str::FromStr for Second {
    type Err = SecondError;

    #[inline]
    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let second = source.parse().map_err(SecondError::Format)?;
        let second = Self::new(second)?;

        Ok(second)
    }
}

impl Second {
    #[inline]
    pub const fn new(second: u32) -> Result<Self, SecondError> {
        if second < 60 {
            Ok(Self(second))
        } else {
            Err(SecondError::Range)
        }
    }

    #[inline]
    pub const unsafe fn new_unchecked(second: u32) -> Self {
        Self(second)
    }
}
