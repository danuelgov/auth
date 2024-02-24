use crate::{visit_knowns, Known, KnownName, Visitor};
use convert_case::{Case, Casing};
use std::io::Write;

pub struct FromKnownKindToPrimaryKeyVisitor<'build> {
    file: &'build mut std::fs::File,
}

impl<'build> FromKnownKindToPrimaryKeyVisitor<'build> {
    #[inline]
    pub fn new(file: &'build mut std::fs::File) -> Self {
        Self { file }
    }
}

impl<'build> Visitor for FromKnownKindToPrimaryKeyVisitor<'build> {
    fn visit_knowns(
        &mut self,
        knowns: &std::collections::HashMap<KnownName, Known>,
    ) -> Result<(), std::io::Error> {
        writeln!(self.file)?;
        writeln!(self.file, "    impl From<KnownKind> for PrimaryKey {{")?;
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
