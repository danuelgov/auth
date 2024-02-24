use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct EmailAddress(dep_email_address::EmailAddress);

impl std::fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}

impl std::fmt::Debug for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EmailAddress({:?})", self.as_str())
    }
}

impl std::str::FromStr for EmailAddress {
    type Err = dep_email_address::Error;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let email_address = source.parse()?;

        Ok(Self(email_address))
    }
}

impl<'de> Deserialize<'de> for EmailAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let email_address = String::deserialize(deserializer)?;
        let email_address = email_address.parse().map_err(serde::de::Error::custom)?;

        Ok(Self(email_address))
    }
}

impl Serialize for EmailAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl EmailAddress {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
