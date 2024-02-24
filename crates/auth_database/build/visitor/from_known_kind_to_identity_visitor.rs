use crate::{visit_knowns, visit_schema, Known, KnownName, Visitor};
use convert_case::{Case, Casing};
use std::io::Write;

pub struct FromKnownKindToIdentityVisitor<'build> {
    file: &'build mut std::fs::File,
}

impl<'build> FromKnownKindToIdentityVisitor<'build> {
    #[inline]
    pub fn new(file: &'build mut std::fs::File) -> Self {
        Self { file }
    }
}

impl<'build> Visitor for FromKnownKindToIdentityVisitor<'build> {
    fn visit_schema(&mut self, schema: &crate::Schema) -> Result<(), std::io::Error> {
        if schema.identity().is_some() {
            visit_schema(self, schema)?;
        }

        Ok(())
    }

    fn visit_knowns(
        &mut self,
        knowns: &std::collections::HashMap<KnownName, Known>,
    ) -> Result<(), std::io::Error> {
        writeln!(self.file)?;
        writeln!(self.file, "    impl From<KnownKind> for Identity {{")?;
        writeln!(
            self.file,
            "        fn from(known_kind: KnownKind) -> Self {{"
        )?;
        writeln!(self.file, "            match known_kind {{")?;
        visit_knowns(self, knowns)?;
        writeln!(self.file, "            }}")?;
        writeln!(self.file, "        }}")?;
        writeln!(self.file, "    }}")?;

        Ok(())
    }

    fn visit_known(&mut self, name: &KnownName, _known: &Known) -> Result<(), std::io::Error> {
        writeln!(
            self.file,
            "                KnownKind::{} => Self::{},",
            name.0.to_case(Case::Pascal),
            name.0.to_ascii_uppercase(),
        )?;

        Ok(())
    }
}
