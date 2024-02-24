use crate::{Column, ColumnName, Visitor};
use std::io::Write;

pub struct ColumnNameVisitor<'build> {
    file: &'build mut std::fs::File,
}

impl<'build> ColumnNameVisitor<'build> {
    #[inline]
    pub fn new(file: &'build mut std::fs::File) -> Self {
        Self { file }
    }
}

impl<'build> Visitor for ColumnNameVisitor<'build> {
    fn visit_column(
        &mut self,
        column_name: &ColumnName,
        _column: &Column,
    ) -> Result<(), std::io::Error> {
        writeln!(
            self.file,
            "        pub const {}: super::Column = super::Column::new(\"{}\");",
            column_name.to_ascii_uppercase(),
            column_name,
        )?;

        Ok(())
    }
}
