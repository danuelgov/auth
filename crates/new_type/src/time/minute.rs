use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Minute(u32);

#[derive(Debug)]
pub enum MinuteError {
    Format(<u32 as std::str::FromStr>::Err),
    Range,
}

impl std::ops::Deref for Minute {
    type Target = u32;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Minute {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

impl std::str::FromStr for Minute {
    type Err = MinuteError;

    #[inline]
    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let minute = source.parse().map_err(MinuteError::Format)?;
        let minute = Self::new(minute)?;

        Ok(minute)
    }
}

impl Minute {
    #[inline]
    pub const fn new(minute: u32) -> Result<Self, MinuteError> {
        if minute < 60 {
            Ok(Self(minute))
        } else {
            Err(MinuteError::Range)
        }
    }

    #[inline]
    pub const unsafe fn new_unchecked(minute: u32) -> Self {
        Self(minute)
    }
}
