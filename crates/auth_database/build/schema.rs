use convert_case::{Case, Casing};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Schema {
    pub table: TableSchema,
    pub columns: HashMap<ColumnName, Column>,
    pub knowns: Option<HashMap<KnownName, Known>>,
}

#[derive(Debug, Deserialize)]
pub struct TableSchema {
    pub name: TableName,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(transparent)]
pub struct TableName(pub String);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(transparent)]
pub struct ColumnName(pub String);

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Column {
    PrimaryKey,
    Identity(IdentityType),
    String(StringType),
    DateTime,
    Handle,
    Hash,
    Json,
    Image,
    IpAddr,
}

#[derive(Debug, Deserialize)]
pub struct IdentityType {
    pub prefix: String,
}

#[derive(Debug, Deserialize)]
pub struct StringType {
    pub min_length: Option<u8>,
    pub max_length: Option<u8>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(transparent)]
pub struct KnownName(pub String);

#[derive(Debug, Deserialize)]
pub struct Known {
    pub primary_key: Key,
    pub identity: Option<Key>,
}

#[derive(Debug)]
pub struct Key(pub u128);

impl Schema {
    pub fn identity(&self) -> Option<&IdentityType> {
        self.columns.values().find_map(|column| match column {
            Column::Identity(identity) => Some(identity),
            _ => None,
        })
    }
}

impl std::ops::Deref for TableName {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for TableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TableName {
    #[inline]
    pub fn to_case(&self, case: Case) -> String {
        self.0.to_case(case)
    }
}

impl std::ops::Deref for ColumnName {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for ColumnName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ColumnName {
    #[inline]
    pub fn to_case(&self, case: Case) -> String {
        self.0.to_case(case)
    }
}

impl std::ops::Deref for KnownName {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for KnownName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for Key {
    fn deserialize<D>(deserializer: D) -> Result<Key, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let source = String::deserialize(deserializer)?;
        let key = u128::from_str_radix(&source[2..], 16).map_err(serde::de::Error::custom)?;
        Ok(Key(key))
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:032x}", self.0)
    }
}
