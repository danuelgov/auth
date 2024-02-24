use crate::{
    visit_schema, AllColumnNameVisitor, ColumnName, IdentityType, Schema, TableName, Visitor,
};
use convert_case::Case;
use std::io::Write;

pub struct TableVisitor<'build> {
    file: &'build mut std::fs::File,
    table_name: TableName,
    visited_primary_key: bool,
    visited_identity: bool,
}

impl<'build> TableVisitor<'build> {
    #[inline]
    pub fn new(file: &'build mut std::fs::File, table_name: TableName) -> Self {
        Self {
            file,
            table_name,
            visited_primary_key: false,
            visited_identity: false,
        }
    }
}

impl<'build> Visitor for TableVisitor<'build> {
    fn visit_schema(&mut self, schema: &Schema) -> Result<(), std::io::Error> {
        writeln!(self.file, "    use database_toolkit::Table;")?;
        writeln!(self.file)?;
        writeln!(
            self.file,
            "    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]"
        )?;
        writeln!(
            self.file,
            "    pub struct {};",
            self.table_name.to_case(Case::Pascal)
        )?;
        writeln!(self.file)?;
        writeln!(
            self.file,
            "    impl Table for {} {{",
            self.table_name.to_case(Case::Pascal)
        )?;
        writeln!(
            self.file,
            "        const NAME: &'static str = \"{}\";",
            self.table_name
        )?;
        writeln!(self.file, "    }}")?;
        visit_schema(self, schema)?;
        AllColumnNameVisitor::new(self.file).visit_schema(schema)?;

        Ok(())
    }

    fn visit_primary_key_type(&mut self, _name: &ColumnName) -> Result<(), std::io::Error> {
        if self.visited_primary_key {
            return Ok(());
        }

        self.visited_primary_key = true;

        writeln!(self.file)?;
        writeln!(
            self.file,
            "    pub type PrimaryKey = crate::identity::PrimaryKey<{}>;",
            self.table_name.to_case(Case::Pascal),
        )?;

        Ok(())
    }

    fn visit_identity_type(
        &mut self,
        _name: &ColumnName,
        identity_type: &IdentityType,
    ) -> Result<(), std::io::Error> {
        if self.visited_identity {
            return Ok(());
        }

        self.visited_identity = true;

        writeln!(self.file)?;
        writeln!(
            self.file,
            "    pub type Identity = crate::identity::Identity<{}>;",
            self.table_name.to_case(Case::Pascal),
        )?;
        writeln!(self.file)?;
        writeln!(
            self.file,
            "    impl identity::Prefix for {} {{",
            self.table_name.to_case(Case::Pascal),
        )?;
        writeln!(
            self.file,
            "        const PREFIX: &'static str = \"{}\";",
            identity_type.prefix,
        )?;
        writeln!(self.file, "    }}")?;

        Ok(())
    }
}
