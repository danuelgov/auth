use crate::{ColumnName, IdentityType, StringType, TableName, Visitor};
use convert_case::{Case, Casing};
use std::io::Write;

pub struct ColumnTypeVisitor<'build> {
    file: &'build mut std::fs::File,
    table_name: TableName,
}

impl<'build> ColumnTypeVisitor<'build> {
    #[inline]
    pub fn new(file: &'build mut std::fs::File, table_name: TableName) -> Self {
        Self { file, table_name }
    }
}

impl<'build> Visitor for ColumnTypeVisitor<'build> {
    fn visit_primary_key_type(&mut self, name: &ColumnName) -> Result<(), std::io::Error> {
        let name = &name.0[..&name.0.len() - 3];

        writeln!(self.file)?;
        writeln!(
            self.file,
            "        pub type {}PrimaryKey = crate::generated::{}::PrimaryKey;",
            name.to_case(Case::Pascal),
            name,
        )?;

        Ok(())
    }

    fn visit_identity_type(
        &mut self,
        _name: &ColumnName,
        _identity_type: &IdentityType,
    ) -> Result<(), std::io::Error> {
        writeln!(self.file)?;
        writeln!(
            self.file,
            "        pub type {}Identity = super::Identity;",
            self.table_name.to_case(Case::Pascal),
        )?;

        Ok(())
    }

    fn visit_string_type(
        &mut self,
        name: &ColumnName,
        string_type: &StringType,
    ) -> Result<(), std::io::Error> {
        writeln!(self.file)?;
        writeln!(
            self.file,
            "        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]"
        )?;
        writeln!(
            self.file,
            "        pub struct {}{}(String);",
            self.table_name.to_case(Case::Pascal),
            name.to_case(Case::Pascal)
        )?;
        writeln!(self.file)?;
        writeln!(
            self.file,
            "        impl std::ops::Deref for {}{} {{",
            self.table_name.to_case(Case::Pascal),
            name.to_case(Case::Pascal)
        )?;
        writeln!(self.file, "            type Target = str;")?;
        writeln!(self.file)?;
        writeln!(self.file, "            #[inline]")?;
        writeln!(self.file, "            fn deref(&self) -> &Self::Target {{")?;
        writeln!(self.file, "                &self.0")?;
        writeln!(self.file, "            }}")?;
        writeln!(self.file, "        }}")?;
        writeln!(self.file)?;
        writeln!(
            self.file,
            "        impl<'de> serde::Deserialize<'de> for {}{} {{",
            self.table_name.to_case(Case::Pascal),
            name.to_case(Case::Pascal)
        )?;
        writeln!(
            self.file,
            "            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>"
        )?;
        writeln!(self.file, "            where")?;
        writeln!(self.file, "                D: serde::Deserializer<'de>,")?;
        writeln!(self.file, "            {{")?;
        writeln!(
            self.file,
            "                let source = String::deserialize(deserializer)?;"
        )?;
        let min_length = string_type.min_length.unwrap_or(0);
        if min_length != 0 {
            writeln!(
                self.file,
                "                if source.len() < {} {{",
                string_type.min_length.unwrap_or(0)
            )?;
            writeln!(
                self.file,
                "                    return Err(serde::de::Error::custom(\"too short\"));"
            )?;
            writeln!(self.file, "                }}")?;
        }
        let max_length = string_type.max_length.unwrap_or(255);
        if max_length != 0 {
            writeln!(
                self.file,
                "                if source.len() > {} {{",
                max_length
            )?;
            writeln!(
                self.file,
                "                    return Err(serde::de::Error::custom(\"too long\"));"
            )?;
            writeln!(self.file, "                }}")?;
        }
        writeln!(self.file, "                Ok(Self(source))")?;
        writeln!(self.file, "            }}")?;
        writeln!(self.file, "        }}")?;
        writeln!(self.file)?;
        writeln!(
            self.file,
            "        impl {}{} {{",
            self.table_name.to_case(Case::Pascal),
            name.to_case(Case::Pascal)
        )?;
        writeln!(self.file, "            #[inline]")?;
        writeln!(self.file, "            pub fn as_str(&self) -> &str {{")?;
        writeln!(self.file, "                &self.0")?;
        writeln!(self.file, "            }}")?;
        writeln!(self.file, "        }}")?;

        Ok(())
    }

    fn visit_date_time_type(&mut self, name: &ColumnName) -> Result<(), std::io::Error> {
        writeln!(self.file)?;
        writeln!(self.file, "        #[derive(Serialize, Deserialize)]")?;
        writeln!(self.file, "        #[serde(transparent)]")?;
        writeln!(
            self.file,
            "        pub struct {}{}(chrono::DateTime<chrono::Utc>);",
            self.table_name.to_case(Case::Pascal),
            name.to_case(Case::Pascal)
        )?;

        Ok(())
    }

    fn visit_handle_type(&mut self, _name: &ColumnName) -> Result<(), std::io::Error> {
        writeln!(self.file)?;
        writeln!(
            self.file,
            "        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]"
        )?;
        writeln!(
            self.file,
            "        pub struct {}Handle(new_type::Handle);",
            self.table_name.to_case(Case::Pascal),
        )?;

        Ok(())
    }

    fn visit_hash_type(&mut self, _name: &ColumnName) -> Result<(), std::io::Error> {
        writeln!(self.file)?;
        writeln!(
            self.file,
            "        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]"
        )?;
        writeln!(
            self.file,
            "        pub struct {}Hash(new_type::Hash);",
            self.table_name.to_case(Case::Pascal),
        )?;

        Ok(())
    }

    fn visit_salt_type(&mut self, _name: &ColumnName) -> Result<(), std::io::Error> {
        writeln!(self.file)?;
        writeln!(
            self.file,
            "        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]"
        )?;
        writeln!(
            self.file,
            "        pub struct {}Salt(new_type::Salt);",
            self.table_name.to_case(Case::Pascal),
        )?;

        Ok(())
    }

    fn visit_json_type(&mut self, _name: &ColumnName) -> Result<(), std::io::Error> {
        writeln!(self.file)?;
        writeln!(
            self.file,
            "        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]"
        )?;
        writeln!(
            self.file,
            "        pub struct {}Json<T>(T);",
            self.table_name.to_case(Case::Pascal),
        )?;

        Ok(())
    }

    fn visit_image_type(&mut self, _name: &ColumnName) -> Result<(), std::io::Error> {
        writeln!(self.file)?;
        writeln!(
            self.file,
            "        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]"
        )?;
        writeln!(
            self.file,
            "        pub struct {}Image(String);",
            self.table_name.to_case(Case::Pascal),
        )?;

        Ok(())
    }

    fn visit_ip_addr_type(&mut self, _name: &ColumnName) -> Result<(), std::io::Error> {
        writeln!(self.file)?;
        writeln!(
                self.file,
                "        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]"
            )?;
        writeln!(
            self.file,
            "        pub struct {}IpAddr(new_type::IpAddr);",
            self.table_name.to_case(Case::Pascal),
        )?;

        Ok(())
    }
}
