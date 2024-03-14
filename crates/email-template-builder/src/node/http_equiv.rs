use crate::{Attribute, IntoAttribute};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum HttpEquiv {
    #[default]
    XUaCompatible,
}

impl IntoAttribute for HttpEquiv {
    #[inline]
    fn into_attribute(self) -> Attribute {
        match self {
            HttpEquiv::XUaCompatible => ("http-equiv", "X-UA-Compatible").into_attribute(),
        }
    }
}
