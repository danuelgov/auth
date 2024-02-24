use crate::{visit_knowns, visit_schema, Known, KnownName, Visitor};
use convert_case::{Case, Casing};
use std::{collections::HashMap, io::Write};

pub struct FromIdentityToKnownKindVisitor<'build> {
    file: &'build mut std::fs::File,
}

impl<'build> FromIdentityToKnownKindVisitor<'build> {
    #[inline]
    pub fn new(file: &'build mut std::fs::File) -> Self {
        Self { file }
    }
}

impl<'build> Visitor for FromIdentityToKnownKindVisitor<'build> {
    fn visit_schema(&mut self, schema: &crate::Schema) -> Result<(), std::io::Error> {
        if schema.identity().is_some() {
            visit_schema(self, schema)?;
        }

        Ok(())
    }

    fn visit_knowns(&mut self, knowns: &HashMap<KnownName, Known>) -> Result<(), std::io::Error> {
        writeln!(self.file)?;
        writeln!(self.file, "    impl From<Identity> for KnownKind {{",)?;
        writeln!(self.file, "        fn from(identity: Identity) -> Self {{")?;
        writeln!(self.file, "            match identity {{")?;
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
            "                Identity::{} => Self::{},",
            name.0.to_ascii_uppercase(),
            name.0.to_case(Case::Pascal),
        )?;

        Ok(())
    }
}
