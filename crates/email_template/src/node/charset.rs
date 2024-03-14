use crate::{Attribute, IntoAttribute};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Charset {
    #[default]
    Utf8,
    Specific(String),
}

impl IntoAttribute for Charset {
    #[inline]
    fn into_attribute(self) -> Attribute {
        match self {
            Charset::Utf8 => ("charset", "UTF-8").into_attribute(),
            Charset::Specific(value) => ("charset", value).into_attribute(),
        }
    }
}
