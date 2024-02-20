use crate::{IpAddrV4, IpAddrV6};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub struct IpAddr(std::net::IpAddr);

impl std::fmt::Display for IpAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for IpAddr {
    type Err = std::net::AddrParseError;

    #[inline]
    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Ok(Self(source.parse()?))
    }
}

impl From<[u8; 4]> for IpAddr {
    #[inline]
    fn from(value: [u8; 4]) -> Self {
        Self(std::net::IpAddr::from(value))
    }
}

impl From<[u8; 16]> for IpAddr {
    #[inline]
    fn from(value: [u8; 16]) -> Self {
        Self(std::net::IpAddr::from(value))
    }
}

impl From<[u16; 8]> for IpAddr {
    #[inline]
    fn from(value: [u16; 8]) -> Self {
        Self(std::net::IpAddr::from(value))
    }
}

impl From<u32> for IpAddr {
    #[inline]
    fn from(value: u32) -> Self {
        let value: [u8; 4] = unsafe { std::mem::transmute(value) };

        Self(std::net::IpAddr::from(value))
    }
}

impl From<u128> for IpAddr {
    #[inline]
    fn from(value: u128) -> Self {
        let value: [u16; 8] = unsafe { std::mem::transmute(value) };

        Self(std::net::IpAddr::from(value))
    }
}

impl TryFrom<&[u8]> for IpAddr {
    type Error = Vec<u8>;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value.len() {
            4 => {
                let ip_addr: [u8; 4] = value.try_into().map_err(|_| value.to_vec())?;

                Ok(ip_addr.into())
            }
            16 => {
                let ip_addr: [u8; 16] = value.try_into().map_err(|_| value.to_vec())?;

                Ok(ip_addr.into())
            }
            _ => Err(value.to_vec()),
        }
    }
}

impl TryFrom<Vec<u8>> for IpAddr {
    type Error = Vec<u8>;

    #[inline]
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        value.as_slice().try_into()
    }
}

impl<'de> Deserialize<'de> for IpAddr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let ip_addr = String::deserialize(deserializer)?;
        let ip_addr = ip_addr.parse().map_err(serde::de::Error::custom)?;

        Ok(Self(ip_addr))
    }
}

impl Serialize for IpAddr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}
