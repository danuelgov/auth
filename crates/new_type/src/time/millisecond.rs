use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Millisecond(u32);

#[derive(Debug)]
pub enum MillisecondError {
    Format(<u32 as std::str::FromStr>::Err),
    Range,
}

impl std::ops::Deref for Millisecond {
    type Target = u32;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Millisecond {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

impl std::str::FromStr for Millisecond {
    type Err = MillisecondError;

    #[inline]
    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let millisecond = source.parse().map_err(MillisecondError::Format)?;
        let millisecond = Self::new(millisecond)?;

        Ok(millisecond)
    }
}

impl Millisecond {
    #[inline]
    pub const fn new(millisecond: u32) -> Result<Self, MillisecondError> {
        if millisecond < 1000 {
            Ok(Self(millisecond))
        } else {
            Err(MillisecondError::Range)
        }
    }

    #[inline]
    pub const unsafe fn new_unchecked(millisecond: u32) -> Self {
        Self(millisecond)
    }
}
