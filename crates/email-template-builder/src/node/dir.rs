use crate::{Attribute, IntoAttribute};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Dir {
    #[default]
    Auto,
    Ltr,
    Rtl,
}

impl IntoAttribute for Dir {
    #[inline]
    fn into_attribute(self) -> Attribute {
        match self {
            Dir::Auto => ("dir", "auto").into_attribute(),
            Dir::Ltr => ("dir", "ltr").into_attribute(),
            Dir::Rtl => ("dir", "rtl").into_attribute(),
        }
    }
}
