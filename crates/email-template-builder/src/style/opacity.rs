use crate::{IntoStyle, StyleAttribute};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Opacity {
    #[default]
    Zero,
    Specific(f32),
}

impl IntoStyle for Opacity {
    #[inline]
    fn into_style(self) -> StyleAttribute {
        match self {
            Opacity::Zero => ("opacity", "0").into_style(),
            Opacity::Specific(value) => ("opacity", value).into_style(),
        }
    }
}
