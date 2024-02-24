use crate::{visit_knowns, Known, KnownName, Visitor};
use std::io::Write;

pub struct AllKnownNameVisitor<'build> {
    file: &'build mut std::fs::File,
}

impl<'build> AllKnownNameVisitor<'build> {
    #[inline]
    pub fn new(file: &'build mut std::fs::File) -> Self {
        Self { file }
    }
}

impl<'build> Visitor for AllKnownNameVisitor<'build> {
    fn visit_knowns(
        &mut self,
        knowns: &std::collections::HashMap<KnownName, Known>,
    ) -> Result<(), std::io::Error> {
        writeln!(self.file)?;
        writeln!(self.file, "    impl PrimaryKey {{")?;
        writeln!(
            self.file,
            "        pub const ALL: [Self; {}] = [",
            knowns.len()
        )?;
        visit_knowns(self, knowns)?;
        writeln!(self.file, "        ];")?;
        writeln!(self.file, "    }}")?;
        if knowns.iter().any(|(_, known)| known.identity.is_some()) {
            writeln!(self.file)?;
            writeln!(self.file, "    impl Identity {{")?;
            writeln!(
                self.file,
                "        pub const ALL: [Self; {}] = [",
                knowns.len()
            )?;
            visit_knowns(self, knowns)?;
            writeln!(self.file, "        ];")?;
            writeln!(self.file, "    }}")?;
        }

        Ok(())
    }

    fn visit_known(&mut self, name: &KnownName, _known: &Known) -> Result<(), std::io::Error> {
        writeln!(
            self.file,
            "            Self::{},",
            name.0.to_ascii_uppercase(),
        )?;

        Ok(())
    }
}
