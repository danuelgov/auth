use crate::{Known, KnownName, Visitor};
use std::io::Write;

pub struct KeyToolsVisitor<'build> {
    file: &'build mut std::fs::File,
}

impl<'build> KeyToolsVisitor<'build> {
    #[inline]
    pub fn new(file: &'build mut std::fs::File) -> Self {
        Self { file }
    }
}

impl<'build> Visitor for KeyToolsVisitor<'build> {
    fn visit_knowns(
        &mut self,
        knowns: &std::collections::HashMap<KnownName, Known>,
    ) -> Result<(), std::io::Error> {
        writeln!(self.file)?;
        writeln!(self.file, "    impl PrimaryKey {{")?;
        writeln!(self.file, "        #[inline]")?;
        writeln!(self.file, "        pub fn known_kind(self) -> KnownKind {{")?;
        writeln!(self.file, "            KnownKind::from(self)")?;
        writeln!(self.file, "        }}")?;
        if knowns.iter().any(|(_, known)| known.identity.is_some()) {
            writeln!(self.file)?;
            writeln!(self.file, "        #[inline]")?;
            writeln!(self.file, "        pub fn identity(self) -> Identity {{")?;
            writeln!(self.file, "            Identity::from(self.known_kind())")?;
            writeln!(self.file, "        }}")?;
        }
        writeln!(self.file, "    }}")?;

        if knowns.iter().any(|(_, known)| known.identity.is_some()) {
            writeln!(self.file)?;
            writeln!(self.file, "    impl Identity {{")?;
            writeln!(self.file, "        #[inline]")?;
            writeln!(self.file, "        pub fn known_kind(self) -> KnownKind {{")?;
            writeln!(self.file, "            KnownKind::from(self)")?;
            writeln!(self.file, "        }}")?;
            writeln!(self.file)?;
            writeln!(self.file, "        #[inline]")?;
            writeln!(self.file, "        pub fn primary_key(self) -> PrimaryKey {{")?;
            writeln!(self.file, "            PrimaryKey::from(self.known_kind())")?;
            writeln!(self.file, "        }}")?;
            writeln!(self.file, "    }}")?;
        }

        Ok(())
    }
}
