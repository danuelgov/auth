use crate::{TableName, Visitor};
use std::io::Write;

pub struct TableNameVisitor<'build> {
    file: &'build mut std::fs::File,
}

impl<'build> TableNameVisitor<'build> {
    #[inline]
    pub fn new(file: &'build mut std::fs::File) -> Self {
        Self { file }
    }
}

impl<'build> Visitor for TableNameVisitor<'build> {
    fn visit_table_name(&mut self, table_name: &TableName) -> Result<(), std::io::Error> {
        writeln!(self.file, "    use database_toolkit::Table;")?;
        writeln!(self.file)?;
        writeln!(
            self.file,
            "    pub const TABLE_NAME: Table = Table({:?});",
            table_name.0,
        )?;

        Ok(())
    }
}
