use serde::{Deserialize, Serialize};
use std::slice::from_raw_parts;

#[derive(Debug, PartialEq)]
pub struct IpAddrV4(std::net::Ipv4Addr);

impl std::fmt::Display for IpAddrV4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for IpAddrV4 {
    type Err = std::net::AddrParseError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Ok(Self(source.parse()?))
    }
}

impl From<[u8; 4]> for IpAddrV4 {
    #[inline]
    fn from(ip_addr: [u8; 4]) -> Self {
        Self(std::net::Ipv4Addr::from(ip_addr))
    }
}

impl From<u32> for IpAddrV4 {
    #[inline]
    fn from(ip_addr: u32) -> Self {
        Self(std::net::Ipv4Addr::from(ip_addr))
    }
}

impl TryFrom<&[u8]> for IpAddrV4 {
    type Error = Vec<u8>;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 4 {
            return Err(value.to_vec());
        }

        let mut ip_addr = [0; 4];
        ip_addr.copy_from_slice(value);

        Ok(ip_addr.into())
    }
}

impl TryFrom<Vec<u8>> for IpAddrV4 {
    type Error = Vec<u8>;

    #[inline]
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        value.as_slice().try_into()
    }
}

impl<'de> Deserialize<'de> for IpAddrV4 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let ip_addr = String::deserialize(deserializer)?;
        let ip_addr = ip_addr.parse().map_err(serde::de::Error::custom)?;

        Ok(Self(ip_addr))
    }
}

impl Serialize for IpAddrV4 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}
