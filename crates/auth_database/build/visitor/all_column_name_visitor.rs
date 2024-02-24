use crate::{visit_columns, visit_schema, Column, ColumnName, Schema, Visitor};
use std::{collections::HashMap, io::Write};

pub struct AllColumnNameVisitor<'build> {
    file: &'build mut std::fs::File,
}

impl<'build> AllColumnNameVisitor<'build> {
    #[inline]
    pub fn new(file: &'build mut std::fs::File) -> Self {
        Self { file }
    }
}

impl<'build> Visitor for AllColumnNameVisitor<'build> {
    fn visit_schema(&mut self, schema: &Schema) -> Result<(), std::io::Error> {
        writeln!(self.file)?;
        visit_schema(self, schema)?;
        writeln!(self.file, "        ];")?;

        Ok(())
    }

    fn visit_columns(
        &mut self,
        columns: &HashMap<ColumnName, Column>,
    ) -> Result<(), std::io::Error> {
        writeln!(
            self.file,
            "        pub const ALL: [Column; {}] = [",
            columns.len()
        )?;
        visit_columns(self, columns)?;

        Ok(())
    }

    fn visit_column(
        &mut self,
        column_name: &ColumnName,
        _column: &Column,
    ) -> Result<(), std::io::Error> {
        writeln!(
            self.file,
            "            {},",
            column_name.to_ascii_uppercase()
        )?;

        Ok(())
    }
}
