use crate::{visit_knowns, Visitor};
use std::io::Write;

pub struct KnownPrimaryKeyVisitor<'build> {
    file: &'build mut std::fs::File,
}

impl<'build> KnownPrimaryKeyVisitor<'build> {
    #[inline]
    pub fn new(file: &'build mut std::fs::File) -> Self {
        Self { file }
    }
}

impl<'build> Visitor for KnownPrimaryKeyVisitor<'build> {
    fn visit_knowns(
        &mut self,
        knowns: &std::collections::HashMap<crate::KnownName, crate::Known>,
    ) -> Result<(), std::io::Error> {
        writeln!(self.file)?;
        writeln!(self.file, "    impl PrimaryKey {{")?;
        visit_knowns(self, knowns)?;
        writeln!(self.file, "    }}")?;

        Ok(())
    }

    fn visit_known(
        &mut self,
        name: &crate::KnownName,
        known: &crate::Known,
    ) -> Result<(), std::io::Error> {
        writeln!(
            self.file,
            "        pub const {}: Self =",
            name.0.to_ascii_uppercase()
        )?;
        writeln!(
            self.file,
            "            unsafe {{ Self::new_unchecked({}) }};",
            known.primary_key
        )?;

        Ok(())
    }
}
