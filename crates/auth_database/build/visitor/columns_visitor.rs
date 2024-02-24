use crate::{ColumnNameVisitor, ColumnTypeVisitor, Schema, Visitor};
use std::io::Write;

pub struct ColumnsVisitor<'build> {
    file: &'build mut std::fs::File,
}

impl<'build> ColumnsVisitor<'build> {
    #[inline]
    pub fn new(file: &'build mut std::fs::File) -> Self {
        Self { file }
    }
}

impl<'build> Visitor for ColumnsVisitor<'build> {
    fn visit_schema(&mut self, schema: &Schema) -> Result<(), std::io::Error> {
        writeln!(self.file)?;
        writeln!(self.file, "    pub mod columns {{")?;
        ColumnNameVisitor::new(self.file).visit_schema(schema)?;
        ColumnTypeVisitor::new(self.file, schema.table.name.to_owned()).visit_schema(schema)?;
        writeln!(self.file, "    }}")?;

        Ok(())
    }
}
