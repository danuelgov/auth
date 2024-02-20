use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub struct IpAddrV6(std::net::Ipv6Addr);

impl std::fmt::Display for IpAddrV6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for IpAddrV6 {
    type Err = std::net::AddrParseError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let ip_addr = source.parse()?;

        Ok(Self(ip_addr))
    }
}

impl From<[u8; 16]> for IpAddrV6 {
    #[inline]
    fn from(value: [u8; 16]) -> Self {
        Self(std::net::Ipv6Addr::from(value))
    }
}

impl From<[u16; 8]> for IpAddrV6 {
    #[inline]
    fn from(value: [u16; 8]) -> Self {
        Self(std::net::Ipv6Addr::from(value))
    }
}

impl From<u128> for IpAddrV6 {
    #[inline]
    fn from(value: u128) -> Self {
        Self(std::net::Ipv6Addr::from(value))
    }
}

impl TryFrom<&[u8]> for IpAddrV6 {
    type Error = Vec<u8>;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 16 {
            return Err(value.to_vec());
        }

        let ip_addr: [u8; 16] = value.try_into().map_err(|_| value.to_vec())?;

        Ok(ip_addr.into())
    }
}

impl TryFrom<&[u16]> for IpAddrV6 {
    type Error = Vec<u16>;

    fn try_from(value: &[u16]) -> Result<Self, Self::Error> {
        let ip_addr: [u16; 8] = value.try_into().map_err(|_| value.to_vec())?;

        Ok(ip_addr.into())
    }
}

impl TryFrom<Vec<u8>> for IpAddrV6 {
    type Error = Vec<u8>;

    #[inline]
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        value.as_slice().try_into()
    }
}

impl TryFrom<Vec<u16>> for IpAddrV6 {
    type Error = Vec<u16>;

    #[inline]
    fn try_from(value: Vec<u16>) -> Result<Self, Self::Error> {
        value.as_slice().try_into()
    }
}

impl<'de> Deserialize<'de> for IpAddrV6 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let ip_addr = String::deserialize(deserializer)?;
        let ip_addr = ip_addr.parse().map_err(serde::de::Error::custom)?;

        Ok(Self(ip_addr))
    }
}

impl Serialize for IpAddrV6 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}
