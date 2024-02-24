use crate::{visit_knowns, Known, KnownName, Visitor};
use convert_case::{Case, Casing};
use std::{collections::HashMap, io::Write};

pub struct FromPrimaryKeyToKnownKindVisitor<'build> {
    file: &'build mut std::fs::File,
}

impl<'build> FromPrimaryKeyToKnownKindVisitor<'build> {
    #[inline]
    pub fn new(file: &'build mut std::fs::File) -> Self {
        Self { file }
    }
}

impl<'build> Visitor for FromPrimaryKeyToKnownKindVisitor<'build> {
    fn visit_knowns(&mut self, knowns: &HashMap<KnownName, Known>) -> Result<(), std::io::Error> {
        writeln!(self.file)?;
        writeln!(self.file, "    impl From<PrimaryKey> for KnownKind {{",)?;
        writeln!(
            self.file,
            "        fn from(primary_key: PrimaryKey) -> Self {{"
        )?;
        writeln!(self.file, "            match primary_key {{")?;
        visit_knowns(self, knowns)?;
        writeln!(self.file, "                _ => std::unreachable!(),")?;
        writeln!(self.file, "            }}")?;
        writeln!(self.file, "        }}")?;
        writeln!(self.file, "    }}")?;

        Ok(())
    }

    fn visit_known(&mut self, name: &KnownName, _known: &Known) -> Result<(), std::io::Error> {
        writeln!(
            self.file,
            "                PrimaryKey::{} => Self::{},",
            name.0.to_ascii_uppercase(),
            name.0.to_case(Case::Pascal),
        )?;

        Ok(())
    }
}
