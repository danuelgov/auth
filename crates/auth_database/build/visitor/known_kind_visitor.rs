use crate::{
    visit_knowns, visit_schema, AllKnownNameVisitor, FromIdentityToKnownKindVisitor,
    FromKnownKindToIdentityVisitor, FromKnownKindToPrimaryKeyVisitor,
    FromPrimaryKeyToKnownKindVisitor, KeyToolsVisitor, Known, KnownIdentityVisitor, KnownName,
    KnownPrimaryKeyVisitor, Visitor,
};
use convert_case::{Case, Casing};
use std::io::Write;

pub struct KnownKindVisitor<'build> {
    file: &'build mut std::fs::File,
}

impl<'build> KnownKindVisitor<'build> {
    #[inline]
    pub fn new(file: &'build mut std::fs::File) -> Self {
        Self { file }
    }
}

impl<'build> Visitor for KnownKindVisitor<'build> {
    fn visit_schema(&mut self, schema: &crate::Schema) -> Result<(), std::io::Error> {
        visit_schema(self, schema)?;
        FromPrimaryKeyToKnownKindVisitor::new(self.file).visit_schema(schema)?;
        FromKnownKindToPrimaryKeyVisitor::new(self.file).visit_schema(schema)?;
        FromIdentityToKnownKindVisitor::new(self.file).visit_schema(schema)?;
        FromKnownKindToIdentityVisitor::new(self.file).visit_schema(schema)?;
        KeyToolsVisitor::new(self.file).visit_schema(schema)?;
        KnownPrimaryKeyVisitor::new(self.file).visit_schema(schema)?;
        KnownIdentityVisitor::new(self.file).visit_schema(schema)?;
        AllKnownNameVisitor::new(self.file).visit_schema(schema)?;

        Ok(())
    }

    fn visit_knowns(
        &mut self,
        knowns: &std::collections::HashMap<KnownName, Known>,
    ) -> Result<(), std::io::Error> {
        writeln!(self.file)?;
        writeln!(
            self.file,
            "    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]"
        )?;
        writeln!(self.file, "    pub enum KnownKind {{")?;
        visit_knowns(self, knowns)?;
        writeln!(self.file, "    }}")?;

        Ok(())
    }

    fn visit_known(&mut self, name: &KnownName, _known: &Known) -> Result<(), std::io::Error> {
        writeln!(self.file, "        {},", name.0.to_case(Case::Pascal),)?;

        Ok(())
    }
}
