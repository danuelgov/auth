use crate::{Attribute, IntoAttribute};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Lang {
    #[default]
    English,
    Korean,
    Manual(String),
}

impl IntoAttribute for Lang {
    #[inline]
    fn into_attribute(self) -> Attribute {
        match self {
            Lang::English => ("lang", "en").into_attribute(),
            Lang::Korean => ("lang", "ko").into_attribute(),
            Lang::Manual(value) => ("lang", value).into_attribute(),
        }
    }
}
