use crate::base58::{self, Base58DecodeError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key(pub(crate) u128);

#[derive(Debug)]
pub enum KeyError {
    Base58,
    Bytes,
    Binary,
}

impl std::ops::Deref for Key {
    type Target = u128;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = base58::encode(self.to_be_bytes());

        write!(f, "{}", id)
    }
}

impl std::str::FromStr for Key {
    type Err = KeyError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let id = base58::decode(source).map_err(|error| match error {
            Base58DecodeError::Base58 => KeyError::Base58,
            Base58DecodeError::Bytes => KeyError::Bytes,
        })?;

        Ok(Self(id))
    }
}

impl From<Key> for Vec<u8> {
    #[inline]
    fn from(id: Key) -> Self {
        id.to_be_bytes().to_vec()
    }
}

impl TryFrom<Vec<u8>> for Key {
    type Error = KeyError;

    fn try_from(binary: Vec<u8>) -> Result<Self, Self::Error> {
        let binary: [u8; 16] = binary.try_into().map_err(|_| KeyError::Binary)?;
        let id = u128::from_be_bytes(binary);

        Ok(Self(id))
    }
}

impl serde::Serialize for Key {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let id = base58::encode(self.to_be_bytes());

        serializer.serialize_str(&id)
    }
}

impl<'de> serde::Deserialize<'de> for Key {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = String::deserialize(deserializer)?;
        let id = base58::decode(&id).map_err(serde::de::Error::custom)?;

        Ok(Self(id))
    }
}

impl std::fmt::Display for KeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Base58 => write!(f, "invalid base58"),
            Self::Bytes => write!(f, "invalid bytes"),
            Self::Binary => write!(f, "invalid binary"),
        }
    }
}

impl std::error::Error for KeyError {
    //
}

impl Key {
    #[inline]
    pub(crate) const fn new(id: u128) -> Self {
        Self(id)
    }

    #[inline]
    pub const fn as_u128(&self) -> u128 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    const HEX: u128 = 0x018d938bdf7e7d27a440bc2ae08ed412;

    #[test]
    fn to_string() {
        let id = crate::Key::new(HEX);

        assert_eq!(id.as_u128(), HEX);
        assert_eq!(format!("{}", id), "c88C3eJ5EnxKpwF1suH7S");
    }

    #[test]
    fn encode() -> Result<(), serde_json::Error> {
        let id = crate::Key::new(HEX);
        let id = serde_json::to_string(&id)?;

        assert_eq!(id, "\"c88C3eJ5EnxKpwF1suH7S\"");

        Ok(())
    }

    #[test]
    fn decode() -> Result<(), serde_json::Error> {
        let id = "\"c88C3eJ5EnxKpwF1suH7S\"";
        let id: crate::Key = serde_json::from_str(&id)?;

        assert_eq!(id.as_u128(), HEX);

        Ok(())
    }
}
