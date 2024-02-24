use crate::{visit_knowns, Visitor};
use std::io::Write;

pub struct KnownIdentityVisitor<'build> {
    file: &'build mut std::fs::File,
}

impl<'build> KnownIdentityVisitor<'build> {
    #[inline]
    pub fn new(file: &'build mut std::fs::File) -> Self {
        Self { file }
    }
}

impl<'build> Visitor for KnownIdentityVisitor<'build> {
    fn visit_knowns(
        &mut self,
        knowns: &std::collections::HashMap<crate::KnownName, crate::Known>,
    ) -> Result<(), std::io::Error> {
        if knowns.iter().any(|(_, known)| known.identity.is_some()) {
            writeln!(self.file)?;
            writeln!(self.file, "    impl Identity {{")?;
            visit_knowns(self, knowns)?;
            writeln!(self.file, "    }}")?;
        }

        Ok(())
    }

    fn visit_known(
        &mut self,
        name: &crate::KnownName,
        known: &crate::Known,
    ) -> Result<(), std::io::Error> {
        if let Some(identity) = &known.identity {
            writeln!(
                self.file,
                "        pub const {}: Self =",
                name.0.to_ascii_uppercase()
            )?;
            writeln!(
                self.file,
                "            unsafe {{ Self::new_unchecked({}) }};",
                identity
            )?;
        }

        Ok(())
    }
}
