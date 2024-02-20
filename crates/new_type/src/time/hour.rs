use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Hour(u32);

#[derive(Debug)]
pub enum HourError {
    Format(<u32 as std::str::FromStr>::Err),
    Range,
}

impl std::ops::Deref for Hour {
    type Target = u32;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Hour {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

impl std::str::FromStr for Hour {
    type Err = HourError;

    #[inline]
    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let hour = source.parse().map_err(HourError::Format)?;
        let hour = Self::new(hour)?;

        Ok(hour)
    }
}

impl Hour {
    #[inline]
    pub const fn new(hour: u32) -> Result<Self, HourError> {
        if hour < 24 {
            Ok(Self(hour))
        } else {
            Err(HourError::Range)
        }
    }

    #[inline]
    pub const unsafe fn new_unchecked(hour: u32) -> Self {
        Self(hour)
    }
}
