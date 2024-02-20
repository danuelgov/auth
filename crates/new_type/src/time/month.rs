use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Month(u32);

#[derive(Debug)]
pub enum MonthError {
    Format(<u32 as std::str::FromStr>::Err),
    Range,
}

impl std::ops::Deref for Month {
    type Target = u32;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Month {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

impl std::str::FromStr for Month {
    type Err = MonthError;

    #[inline]
    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let month = source.parse().map_err(MonthError::Format)?;
        let month = Self::new(month)?;

        Ok(month)
    }
}

impl Month {
    #[inline]
    pub const fn new(month: u32) -> Result<Self, MonthError> {
        match month {
            1..=12 => Ok(Self(month)),
            _ => Err(MonthError::Range),
        }
    }

    #[inline]
    pub const unsafe fn new_unchecked(month: u32) -> Self {
        Self(month)
    }
}
