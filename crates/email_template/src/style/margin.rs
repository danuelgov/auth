use crate::{IntoStyle, StyleAttribute};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Margin {
    #[default]
    Auto,
    All(u32),
    Horizontal(u32),
    Vertical(u32),
    Top(u32),
    Right(u32),
    Bottom(u32),
    Left(u32),
}

impl IntoStyle for Margin {
    #[inline]
    fn into_style(self) -> StyleAttribute {
        match self {
            Margin::Auto => ("margin", "auto").into_style(),
            Margin::All(value) => ("margin", format!("{}px", value)).into_style(),
            Margin::Horizontal(value) => ("margin", format!("0 {}px", value)).into_style(),
            Margin::Vertical(value) => ("margin", format!("{}px 0", value)).into_style(),
            Margin::Top(value) => ("margin", format!("{}px 0 0 0", value)).into_style(),
            Margin::Right(value) => ("margin", format!("0 {}px 0 0", value)).into_style(),
            Margin::Bottom(value) => ("margin", format!("0 0 {}px 0", value)).into_style(),
            Margin::Left(value) => ("margin", format!("0 0 0 {}px", value)).into_style(),
        }
    }
}
