use crate::{Attribute, IntoAttribute};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Target {
    #[default]
    Blank,
    Parent,
    Self_,
    Top,
}

impl IntoAttribute for Target {
    #[inline]
    fn into_attribute(self) -> Attribute {
        match self {
            Target::Blank => ("target", "_blank").into_attribute(),
            Target::Parent => ("target", "_parent").into_attribute(),
            Target::Self_ => ("target", "_self").into_attribute(),
            Target::Top => ("target", "_top").into_attribute(),
        }
    }
}
