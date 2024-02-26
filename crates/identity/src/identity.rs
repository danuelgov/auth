use crate::{uuid, Prefix, PrefixKey, PrefixKeyError};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Identity<P: Prefix>(PrefixKey<P>);

impl<P: Prefix> std::ops::Deref for Identity<P> {
    type Target = PrefixKey<P>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<P: Prefix> std::fmt::Debug for Identity<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identity(HEX({}), BASE58({}))", self.as_u128(), self)
    }
}

impl<P: Prefix> std::fmt::Display for Identity<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<P: Prefix> From<Identity<P>> for Vec<u8> {
    #[inline]
    fn from(identity: Identity<P>) -> Self {
        identity.0.into()
    }
}

impl<P: Prefix> TryFrom<Vec<u8>> for Identity<P> {
    type Error = PrefixKeyError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(PrefixKey::try_from(value)?))
    }
}

impl<P: Prefix> std::str::FromStr for Identity<P> {
    type Err = PrefixKeyError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Ok(Self(source.parse()?))
    }
}

impl<P: Prefix> Identity<P> {
    #[inline]
    pub fn new() -> Self {
        Self(PrefixKey::new(uuid::new_v4()))
    }

    #[inline]
    pub const unsafe fn new_unchecked(id: u128) -> Self {
        Self(PrefixKey::new(id))
    }
}

#[cfg(test)]
mod tests {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    struct Identity<P: crate::Prefix>(crate::Identity<P>);

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    struct OrderPrefix;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    struct Response {
        id: OrderIdentity,
    }

    type OrderIdentity = Identity<OrderPrefix>;

    impl crate::Prefix for OrderPrefix {
        const PREFIX: &'static str = "order_";
    }

    impl std::fmt::Display for OrderIdentity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl OrderIdentity {
        fn new() -> Self {
            Self(crate::Identity::new())
        }

        const unsafe fn new_unchecked(id: u128) -> Self {
            Self(crate::Identity::new_unchecked(id))
        }
    }

    impl OrderIdentity {
        pub const EMAIL: Self = unsafe { Self::new_unchecked(0x2bec0b6586d3440eb391722bb3de7b2c) };
    }

    #[test]
    fn to_string() {
        let identity = OrderIdentity::EMAIL;

        assert_eq!(identity.to_string(), "order_6qzgpWd7sJYc5fuMkAuD2w");
    }

    #[test]
    fn encode() -> Result<(), Box<dyn std::error::Error>> {
        let identity = OrderIdentity::EMAIL;
        let identity = serde_json::to_string(&identity)?;

        assert_eq!(identity, "\"order_6qzgpWd7sJYc5fuMkAuD2w\"");

        Ok(())
    }

    #[test]
    fn decode() -> Result<(), Box<dyn std::error::Error>> {
        let identity = "\"order_6qzgpWd7sJYc5fuMkAuD2w\"";
        let identity: OrderIdentity = serde_json::from_str(identity)?;

        assert_eq!(identity, OrderIdentity::EMAIL);

        Ok(())
    }

    #[ignore]
    #[test]
    fn generate() {
        for _ in 0..10 {
            let id = OrderIdentity::new();

            println!("{:#?}", id);
        }
    }

    #[ignore]
    #[test]
    fn hex_to_base58() {
        let hex = 0x681049B6D1CC453BA880BC8411216FFF;
        let identity = unsafe { OrderIdentity::new_unchecked(hex) };
        dbg!(identity);
    }
}
