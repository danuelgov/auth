use crate::{base58, uuid, Key, KeyError};
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PrimaryKey<T> {
    pub(crate) id: Key,
    #[serde(skip)]
    _marker: std::marker::PhantomData<T>,
}

#[derive(Debug)]
pub enum PrimaryKeyError {
    NotUuidV4,
    Base58,
    Bytes,
    Binary,
}

impl<T> std::ops::Deref for PrimaryKey<T> {
    type Target = Key;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl<T> std::fmt::Debug for PrimaryKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PrimaryKey(HEX({}), BASE58({}))", self.as_u128(), self)
    }
}

impl<T> std::fmt::Display for PrimaryKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = base58::encode(self.to_be_bytes());

        write!(f, "{}", id)
    }
}

impl<T> From<PrimaryKey<T>> for Vec<u8> {
    #[inline]
    fn from(primary_key: PrimaryKey<T>) -> Self {
        primary_key.id.into()
    }
}

impl<T> TryFrom<Vec<u8>> for PrimaryKey<T> {
    type Error = PrimaryKeyError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        match Key::try_from(value) {
            Ok(Key(id)) if uuid::is_v7(id) => Ok(unsafe { Self::new_unchecked(id) }),
            Ok(_) => Err(PrimaryKeyError::NotUuidV4),
            Err(KeyError::Base58) => Err(PrimaryKeyError::Base58),
            Err(KeyError::Bytes) => Err(PrimaryKeyError::Bytes),
            Err(KeyError::Binary) => Err(PrimaryKeyError::Binary),
        }
    }
}

impl<T> TryFrom<PrimaryKey<T>> for NaiveDateTime {
    type Error = ();

    #[inline]
    fn try_from(primary_key: PrimaryKey<T>) -> Result<Self, Self::Error> {
        primary_key.as_date_time().ok_or(())
    }
}

impl std::fmt::Display for PrimaryKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimaryKeyError::NotUuidV4 => write!(f, "NotUuidV4"),
            PrimaryKeyError::Base58 => write!(f, "Base58"),
            PrimaryKeyError::Bytes => write!(f, "Bytes"),
            PrimaryKeyError::Binary => write!(f, "Binary"),
        }
    }
}

impl std::error::Error for PrimaryKeyError {
    //
}

impl<T> PrimaryKey<T> {
    #[inline]
    pub fn new() -> Self {
        Self {
            id: Key::new(uuid::new_v7()),
            _marker: std::marker::PhantomData,
        }
    }

    #[inline]
    pub const unsafe fn new_unchecked(id: u128) -> Self {
        Self {
            id: Key::new(id),
            _marker: std::marker::PhantomData,
        }
    }

    #[inline]
    pub const fn timestamp(&self) -> u64 {
        (self.id.0 >> 64) as u64
    }

    #[inline]
    pub fn as_date_time(&self) -> Option<NaiveDateTime> {
        NaiveDateTime::from_timestamp_nanos(self.timestamp() as i64)
    }
}

#[cfg(test)]
mod tests {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct PrimaryKey<T>(crate::PrimaryKey<T>);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct OrderPrefix;

    type CredentialPrimaryKey = PrimaryKey<OrderPrefix>;

    impl std::ops::Deref for CredentialPrimaryKey {
        type Target = crate::PrimaryKey<OrderPrefix>;

        #[inline]
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::fmt::Display for CredentialPrimaryKey {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl CredentialPrimaryKey {
        #[inline]
        fn new() -> Self {
            Self(crate::PrimaryKey::new())
        }

        #[inline]
        const unsafe fn new_unchecked(id: u128) -> Self {
            Self(crate::PrimaryKey::new_unchecked(id))
        }
    }

    impl CredentialPrimaryKey {
        pub const PIZZA: Self = unsafe { Self::new_unchecked(0x18dac4439297e1ab634048e2a283657) };
    }

    #[test]
    fn primary_key() {
        let primary_key = CredentialPrimaryKey::PIZZA;

        assert_eq!(primary_key.as_u128(), 0x18dac4439297e1ab634048e2a283657);
        assert_eq!(format!("{}", primary_key), "c8hH97Y4QWMjHtZNY3VNF");
    }

    #[ignore]
    #[test]
    fn generate() {
        for _ in 0..10 {
            let id = CredentialPrimaryKey::new();

            println!("{:#?}", id);
        }
    }

    #[ignore]
    #[test]
    fn hex_to_base58() {
        let hex = 0x681049B6D1CC453BA880BC8411216FFF;
        let primary_key = unsafe { CredentialPrimaryKey::new_unchecked(hex) };
        dbg!(primary_key);
    }
}
