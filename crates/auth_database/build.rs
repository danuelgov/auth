use convert_case::{Case, Casing};
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut generated = std::fs::File::create("src/generated.rs")?;
    writeln!(generated, r#"// !"#)?;
    writeln!(generated, r#"// ! This file is generated."#)?;
    writeln!(generated, r#"// ! Do not modify it manually."#)?;
    writeln!(generated, r#"// ! Regenerate it by running `cargo build`."#)?;
    writeln!(generated, r#"// !"#)?;

    let mut paths: Vec<_> = std::fs::read_dir("schema")?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .collect();
    paths.sort();
    for path in paths {
        let source = std::fs::read_to_string(path)?;
        let schema: Schema = toml::from_str(&source)?;
        writeln!(generated)?;
        write!(generated, "{}", schema)?;
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct Schema {
    table: TableSchema,
    columns: HashMap<ColumnName, Column>,
    knowns: Option<HashMap<KnownName, Known>>,
}

#[derive(Debug, Deserialize)]
struct TableSchema {
    name: TableName,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(transparent)]
struct TableName(String);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(transparent)]
struct ColumnName(String);

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum Column {
    PrimaryKey,
    Identity(IdentityType),
    String(StringType),
    DateTime,
    Handle,
    Hash,
    Salt,
    Json,
    Image,
    IpAddr,
}

#[derive(Debug, Deserialize)]
struct IdentityType {
    prefix: String,
}

#[derive(Debug, Deserialize)]
struct StringType {
    min_length: Option<u8>,
    max_length: Option<u8>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(transparent)]
struct KnownName(String);

#[derive(Debug, Deserialize)]
struct Known {
    primary_key: Key,
    identity: Option<Key>,
}

#[derive(Debug)]
struct Key(u128);

impl std::fmt::Display for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "pub mod {} {{", self.table.name)?;
        {
            writeln!(f, "    use database_toolkit::Table;")?;
            writeln!(f)?;
            writeln!(
                f,
                "    pub const TABLE_NAME: Table = Table({:?});",
                self.table.name.0,
            )?;
        }
        {
            writeln!(f)?;
            writeln!(f, "    pub mod columns {{")?;
            writeln!(f, "        use database_toolkit::Column;")?;
            {
                let mut columns: Vec<_> = self.columns.iter().collect();
                columns.sort_by_key(|(name, _)| name.to_owned());
                for (name, column) in columns {
                    writeln!(f)?;
                    match column {
                        Column::PrimaryKey => {
                            writeln!(
                                f,
                                "        pub type {}PrimaryKey = crate::generated::{}::PrimaryKey;",
                                (&name[..name.len() - 3]).to_case(Case::Pascal),
                                &name[..name.len() - 3],
                            )?;
                        }
                        Column::Identity(_) => {
                            writeln!(
                                f,
                                "        pub type {}Identity = super::Identity;",
                                self.table.name.to_case(Case::Pascal),
                            )?;
                        }
                        Column::String(ty) => {
                            writeln!(f, "        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]")?;
                            writeln!(
                                f,
                                "        pub struct {}{}(String);",
                                self.table.name.to_case(Case::Pascal),
                                name.to_case(Case::Pascal),
                            )?;
                            writeln!(f)?;
                            writeln!(
                                f,
                                "        impl std::ops::Deref for {}{} {{",
                                self.table.name.to_case(Case::Pascal),
                                name.to_case(Case::Pascal)
                            )?;
                            writeln!(f, "            type Target = str;")?;
                            writeln!(f)?;
                            writeln!(f, "            #[inline]")?;
                            writeln!(f, "            fn deref(&self) -> &Self::Target {{")?;
                            writeln!(f, "                &self.0")?;
                            writeln!(f, "            }}")?;
                            writeln!(f, "        }}")?;
                            writeln!(f)?;
                            writeln!(
                                f,
                                "        impl<'de> serde::Deserialize<'de> for {}{} {{",
                                self.table.name.to_case(Case::Pascal),
                                name.to_case(Case::Pascal),
                            )?;
                            writeln!(f, "            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>")?;
                            writeln!(f, "            where")?;
                            writeln!(f, "                D: serde::Deserializer<'de>,")?;
                            writeln!(f, "            {{")?;
                            writeln!(
                                f,
                                "                let source = String::deserialize(deserializer)?;"
                            )?;
                            let min_length = ty.min_length.unwrap_or(0);
                            if min_length != 0 {
                                writeln!(
                                    f,
                                    "                if source.len() < {} {{",
                                    ty.min_length.unwrap_or(0)
                                )?;
                                writeln!(f, "                    return Err(serde::de::Error::custom(\"too short\"));")?;
                                writeln!(f, "                }}")?;
                            }
                            let max_length = ty.max_length.unwrap_or(255);
                            if max_length != 0 {
                                writeln!(
                                    f,
                                    "                if source.len() > {} {{",
                                    ty.max_length.unwrap_or(255)
                                )?;
                                writeln!(f, "                    return Err(serde::de::Error::custom(\"too long\"));")?;
                                writeln!(f, "                }}")?;
                            }
                            writeln!(f, "                Ok(Self(source))")?;
                            writeln!(f, "            }}")?;
                            writeln!(f, "        }}")?;
                            writeln!(f)?;
                            writeln!(
                                f,
                                "        impl {}{} {{",
                                self.table.name.to_case(Case::Pascal),
                                name.to_case(Case::Pascal)
                            )?;
                            writeln!(f, "            #[inline]")?;
                            writeln!(f, "            pub fn as_str(&self) -> &str {{")?;
                            writeln!(f, "                &self.0")?;
                            writeln!(f, "            }}")?;
                            writeln!(f, "        }}")?;
                        }
                        Column::DateTime => {
                            writeln!(f, "        #[derive(Serialize, Deserialize)]")?;
                            writeln!(f, "        #[serde(transparent)]")?;
                            writeln!(
                                f,
                                "        pub struct {}{}(chrono::DateTime<chrono::Utc>);",
                                self.table.name.to_case(Case::Pascal),
                                name.to_case(Case::Pascal),
                            )?;
                        }
                        Column::Handle => {
                            writeln!(f, "        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]")?;
                            writeln!(
                                f,
                                "        pub struct {}Handle(new_type::Handle);",
                                self.table.name.to_case(Case::Pascal),
                            )?;
                        }
                        Column::Hash => {
                            writeln!(f, "        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]")?;
                            writeln!(
                                f,
                                "        pub struct {}Hash(new_type::Hash);",
                                self.table.name.to_case(Case::Pascal),
                            )?;
                        }
                        Column::Salt => {
                            writeln!(f, "        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]")?;
                            writeln!(
                                f,
                                "        pub struct {}Salt(new_type::Salt);",
                                self.table.name.to_case(Case::Pascal),
                            )?;
                        }
                        Column::Json => {
                            writeln!(f, "        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]")?;
                            writeln!(
                                f,
                                "        pub struct {}Json<T>(T);",
                                self.table.name.to_case(Case::Pascal),
                            )?;
                        }
                        Column::Image => {
                            writeln!(f, "        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]")?;
                            writeln!(
                                f,
                                "        pub struct {}Image(String);",
                                self.table.name.to_case(Case::Pascal),
                            )?;
                        }
                        Column::IpAddr => {
                            writeln!(f, "        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]")?;
                            writeln!(
                                f,
                                "        pub struct {}IpAddr(new_type::IpAddr);",
                                self.table.name.to_case(Case::Pascal),
                            )?;
                        }
                    }
                }
            }
            writeln!(f)?;
            writeln!(
                f,
                "        pub const ALL: [Column; {}] = [",
                self.columns.len()
            )?;
            for column in self.columns.keys() {
                writeln!(f, "            {},", column.to_ascii_uppercase())?;
            }
            writeln!(f, "        ];")?;
            writeln!(f)?;
            for column in self.columns.keys() {
                writeln!(
                    f,
                    "        pub const {}: Column = Column(\"{}\");",
                    column.to_ascii_uppercase(),
                    column,
                )?;
            }
            writeln!(f, "    }}")?;
        }
        {
            writeln!(f)?;
            writeln!(
                f,
                "    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]"
            )?;
            writeln!(
                f,
                "    pub struct {};",
                self.table.name.to_case(Case::Pascal)
            )?;
            writeln!(f)?;
            writeln!(
                f,
                "    pub type PrimaryKey = crate::identity::PrimaryKey<{}>;",
                self.table.name.to_case(Case::Pascal),
            )?;
        }
        if let Some(identity) = self.identity() {
            writeln!(f)?;
            writeln!(
                f,
                "    pub type Identity = crate::identity::Identity<{}>;",
                self.table.name.to_case(Case::Pascal),
            )?;
            writeln!(f)?;
            writeln!(
                f,
                "    impl identity::Prefix for {} {{",
                self.table.name.to_case(Case::Pascal),
            )?;
            writeln!(
                f,
                "        const PREFIX: &'static str = \"{}\";",
                identity.prefix,
            )?;
            writeln!(f, "    }}")?;
        }
        if let Some(knowns) = &self.knowns {
            if !knowns.is_empty() {
                writeln!(f)?;
                writeln!(f, "    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]")?;
                writeln!(f, "    pub enum KnownKind {{")?;
                for (name, _) in knowns {
                    writeln!(f, "        {},", name.0.to_case(Case::Pascal),)?;
                }
                writeln!(f, "    }}")?;
                writeln!(f)?;
                writeln!(f, "    impl From<KnownKind> for PrimaryKey {{")?;
                writeln!(f, "        #[inline]")?;
                writeln!(f, "        fn from(kind: KnownKind) -> Self {{")?;
                writeln!(f, "            match kind {{")?;
                for name in knowns.keys() {
                    writeln!(
                        f,
                        "                KnownKind::{} => Self::{},",
                        name.to_case(Case::Pascal),
                        name.to_ascii_uppercase(),
                    )?;
                }
                writeln!(f, "            }}")?;
                writeln!(f, "        }}")?;
                writeln!(f, "    }}")?;
                writeln!(f)?;
                writeln!(f, "    impl From<PrimaryKey> for KnownKind {{")?;
                writeln!(f, "        fn from(primary_key: PrimaryKey) -> Self {{")?;
                writeln!(f, "            match primary_key {{")?;
                for name in knowns.keys() {
                    writeln!(
                        f,
                        "                PrimaryKey::{} => KnownKind::{},",
                        name.to_ascii_uppercase(),
                        name.to_case(Case::Pascal),
                    )?;
                }
                writeln!(f, "                _ => std::unreachable!()")?;
                writeln!(f, "            }}")?;
                writeln!(f, "        }}")?;
                writeln!(f, "    }}")?;
                if self.identity().is_some() {
                    writeln!(f)?;
                    writeln!(f, "    impl From<KnownKind> for Identity {{")?;
                    writeln!(f, "        #[inline]")?;
                    writeln!(f, "        fn from(kind: KnownKind) -> Self {{")?;
                    writeln!(f, "            match kind {{")?;
                    for name in knowns.keys() {
                        writeln!(
                            f,
                            "                KnownKind::{} => Self::{},",
                            name.to_case(Case::Pascal),
                            name.to_ascii_uppercase(),
                        )?;
                    }
                    writeln!(f, "            }}")?;
                    writeln!(f, "        }}")?;
                    writeln!(f, "    }}")?;
                    writeln!(f)?;
                    writeln!(f, "    impl From<Identity> for KnownKind {{")?;
                    writeln!(f, "        fn from(identity: Identity) -> Self {{")?;
                    writeln!(f, "            match identity {{")?;
                    for name in knowns.keys() {
                        writeln!(
                            f,
                            "                Identity::{} => KnownKind::{},",
                            name.to_ascii_uppercase(),
                            name.to_case(Case::Pascal),
                        )?;
                    }
                    writeln!(f, "                _ => std::unreachable!()")?;
                    writeln!(f, "            }}")?;
                    writeln!(f, "        }}")?;
                    writeln!(f, "    }}")?;
                }
            }
            {
                writeln!(f)?;
                writeln!(f, "    impl PrimaryKey {{")?;
                writeln!(f, "        pub const ALL: &'static [Self] = &[")?;
                for name in knowns.keys() {
                    writeln!(f, "            Self::{},", name.to_ascii_uppercase(),)?;
                }
                writeln!(f, "        ];")?;
                writeln!(f)?;
                for (name, known) in knowns {
                    writeln!(f, "        pub const {}: Self =", name.to_ascii_uppercase(),)?;
                    writeln!(
                        f,
                        "            unsafe {{ PrimaryKey::new_unchecked({}) }};",
                        known.primary_key,
                    )?;
                }
                if self.identity().is_some() {
                    writeln!(f)?;
                    writeln!(f, "        #[inline]")?;
                    writeln!(f, "        pub fn kind(self) -> KnownKind {{")?;
                    writeln!(f, "            KnownKind::from(self)")?;
                    writeln!(f, "        }}")?;
                    writeln!(f)?;
                    writeln!(f, "        #[inline]")?;
                    writeln!(f, "        pub fn identity(self) -> Identity {{")?;
                    writeln!(f, "            self.kind().into()")?;
                    writeln!(f, "        }}")?;
                }
                writeln!(f, "    }}")?;
            }
            if self.identity().is_some() {
                writeln!(f)?;
                writeln!(f, "    impl Identity {{")?;
                writeln!(f, "        pub const ALL: &'static [Self] = &[")?;
                for name in knowns.keys() {
                    writeln!(f, "            Self::{},", name.to_ascii_uppercase(),)?;
                }
                writeln!(f, "        ];")?;
                writeln!(f)?;
                for (name, known) in knowns {
                    if let Some(identity) = &known.identity {
                        writeln!(f, "        pub const {}: Self =", name.to_ascii_uppercase(),)?;
                        writeln!(
                            f,
                            "            unsafe {{ Identity::new_unchecked({}) }};",
                            identity,
                        )?;
                    }
                }
                writeln!(f)?;
                writeln!(f, "        #[inline]")?;
                writeln!(f, "        pub fn kind(self) -> KnownKind {{")?;
                writeln!(f, "            KnownKind::from(self)")?;
                writeln!(f, "        }}")?;
                writeln!(f)?;
                writeln!(f, "        #[inline]")?;
                writeln!(f, "        pub fn primary_key(self) -> PrimaryKey {{")?;
                writeln!(f, "            self.kind().into()")?;
                writeln!(f, "        }}")?;
                writeln!(f, "    }}")?;
            }
        }
        writeln!(f, "}}")?;

        Ok(())
    }
}

impl Schema {
    fn identity(&self) -> Option<&IdentityType> {
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
    fn to_case(&self, case: Case) -> String {
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
    fn to_case(&self, case: Case) -> String {
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

impl KnownName {
    #[inline]
    fn to_case(&self, case: Case) -> String {
        self.0.to_case(case)
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
