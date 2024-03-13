use crate::{IntoStyle, StyleAttribute};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Overflow {
    Hidden,
    Visible,
    Scroll,
    Auto,
}

impl IntoStyle for Overflow {
    #[inline]
    fn into_style(self) -> StyleAttribute {
        match self {
            Overflow::Hidden => ("overflow", "hidden").into_style(),
            Overflow::Visible => ("overflow", "visible").into_style(),
            Overflow::Scroll => ("overflow", "scroll").into_style(),
            Overflow::Auto => ("overflow", "auto").into_style(),
        }
    }
}
