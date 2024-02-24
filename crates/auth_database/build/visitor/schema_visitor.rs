use crate::{ColumnsVisitor, KnownKindVisitor, Schema, TableVisitor, Visitor};
use std::io::Write;

pub struct SchemaVisitor<'build> {
    file: &'build mut std::fs::File,
}

impl<'build> SchemaVisitor<'build> {
    #[inline]
    pub fn new(file: &'build mut std::fs::File) -> Self {
        Self { file }
    }
}

impl<'build> Visitor for SchemaVisitor<'build> {
    fn visit_schema(&mut self, schema: &Schema) -> Result<(), std::io::Error> {
        writeln!(self.file)?;
        writeln!(self.file, "pub mod {} {{", schema.table.name)?;
        TableVisitor::new(self.file, schema.table.name.to_owned()).visit_schema(schema)?;
        ColumnsVisitor::new(self.file).visit_schema(schema)?;
        KnownKindVisitor::new(self.file).visit_schema(&schema)?;
        writeln!(self.file, "}}")?;

        Ok(())
    }
}
