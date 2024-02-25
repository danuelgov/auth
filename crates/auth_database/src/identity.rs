use chrono::NaiveDateTime;
use identity::Prefix;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PrimaryKey<T>(identity::PrimaryKey<T>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Identity<P: Prefix>(identity::Identity<P>);

impl<T> std::ops::Deref for PrimaryKey<T> {
    type Target = identity::PrimaryKey<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::fmt::Display for PrimaryKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> From<PrimaryKey<T>> for Vec<u8> {
    #[inline]
    fn from(internal_id: PrimaryKey<T>) -> Self {
        internal_id.0.into()
    }
}

impl<T> TryFrom<Vec<u8>> for PrimaryKey<T> {
    type Error = identity::PrimaryKeyError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

impl<T> TryFrom<PrimaryKey<T>> for NaiveDateTime {
    type Error = ();

    #[inline]
    fn try_from(internal_id: PrimaryKey<T>) -> Result<Self, Self::Error> {
        Ok(internal_id.0.try_into()?)
    }
}

impl<P: Prefix> std::ops::Deref for Identity<P> {
    type Target = identity::Identity<P>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<P: Prefix> std::fmt::Display for Identity<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<P: Prefix> From<Identity<P>> for Vec<u8> {
    #[inline]
    fn from(external_id: Identity<P>) -> Self {
        external_id.0.into()
    }
}

impl<P: Prefix> TryFrom<Vec<u8>> for Identity<P> {
    type Error = identity::PrefixKeyError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}
impl<T> PrimaryKey<T> {
    #[inline]
    pub fn new() -> Self {
        Self(identity::PrimaryKey::new())
    }

    #[inline]
    pub const unsafe fn new_unchecked(inner: u128) -> Self {
        Self(identity::PrimaryKey::new_unchecked(inner))
    }
}

impl<P: Prefix> Identity<P> {
    #[inline]
    pub fn new() -> Self {
        Self(identity::Identity::new())
    }

    #[inline]
    pub const unsafe fn new_unchecked(inner: u128) -> Self {
        Self(identity::Identity::new_unchecked(inner))
    }
}
