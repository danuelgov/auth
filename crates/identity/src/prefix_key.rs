use crate::{base58, uuid, Key, KeyError};

pub trait Prefix {
    const PREFIX: &'static str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrefixKey<P: Prefix> {
    pub(crate) id: Key,
    _marker: std::marker::PhantomData<P>,
}

#[derive(Debug)]
pub enum PrefixKeyError {
    NoMatchPrefix,
    Base58,
    Bytes,
    Binary,
    NotUuidV4,
}

impl<P: Prefix> std::ops::Deref for PrefixKey<P> {
    type Target = Key;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl<P: Prefix> std::fmt::Display for PrefixKey<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = base58::encode(self.to_be_bytes());

        write!(f, "{}{}", P::PREFIX, id)
    }
}

impl<P: Prefix> std::str::FromStr for PrefixKey<P> {
    type Err = PrefixKeyError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let source = source
            .strip_prefix(P::PREFIX)
            .ok_or(PrefixKeyError::NoMatchPrefix)?;

        match Key::from_str(source) {
            Ok(Key(id)) if uuid::is_v4(id) => Ok(Self::new(id)),
            Ok(_) => Err(PrefixKeyError::NotUuidV4),
            Err(KeyError::Base58) => Err(PrefixKeyError::Base58),
            Err(KeyError::Bytes) => Err(PrefixKeyError::Bytes),
            Err(KeyError::Binary) => Err(PrefixKeyError::Binary),
        }
    }
}

impl<P: Prefix> From<PrefixKey<P>> for Vec<u8> {
    #[inline]
    fn from(prefix_id: PrefixKey<P>) -> Self {
        prefix_id.id.into()
    }
}

impl<P: Prefix> TryFrom<Vec<u8>> for PrefixKey<P> {
    type Error = PrefixKeyError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        match Key::try_from(value) {
            Ok(Key(id)) if uuid::is_v4(id) => Ok(Self::new(id)),
            Ok(_) => Err(PrefixKeyError::NotUuidV4),
            Err(KeyError::Base58) => Err(PrefixKeyError::Base58),
            Err(KeyError::Bytes) => Err(PrefixKeyError::Bytes),
            Err(KeyError::Binary) => Err(PrefixKeyError::Binary),
        }
    }
}

impl<P: Prefix> serde::Serialize for PrefixKey<P> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de, P: Prefix> serde::Deserialize<'de> for PrefixKey<P> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = String::deserialize(deserializer)?;
        let id = id.parse().map_err(serde::de::Error::custom)?;

        Ok(id)
    }
}

impl std::fmt::Display for PrefixKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoMatchPrefix => write!(f, "no match prefix"),
            Self::Base58 => write!(f, "base58"),
            Self::Bytes => write!(f, "bytes"),
            Self::Binary => write!(f, "binary"),
            Self::NotUuidV4 => write!(f, "not uuid v4"),
        }
    }
}

impl std::error::Error for PrefixKeyError {
    //
}

impl<P: Prefix> PrefixKey<P> {
    pub const fn new(id: u128) -> Self {
        Self {
            id: Key::new(id),
            _marker: std::marker::PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    struct Order;

    impl crate::Prefix for Order {
        const PREFIX: &'static str = "order_";
    }

    type OrderId = crate::PrefixKey<Order>;

    const HEX: u128 = 0x2bec0b6586d3440eb391722bb3de7b2c;

    #[test]
    fn to_string() {
        let id = OrderId::new(HEX);

        assert_eq!(id.as_u128(), HEX);
        assert_eq!(format!("{}", id), "order_6qzgpWd7sJYc5fuMkAuD2w");
    }

    #[test]
    fn encode() -> Result<(), serde_json::Error> {
        let id = OrderId::new(HEX);
        let id = serde_json::to_string(&id)?;

        assert_eq!(id, "\"order_6qzgpWd7sJYc5fuMkAuD2w\"");

        Ok(())
    }

    #[test]
    fn decode() -> Result<(), serde_json::Error> {
        let id = "\"order_6qzgpWd7sJYc5fuMkAuD2w\"";
        let id: OrderId = serde_json::from_str(&id)?;

        assert_eq!(id.as_u128(), HEX);

        Ok(())
    }
}
