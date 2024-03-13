use crate::{IntoStyle, StyleAttribute};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontSize {
    Pixel(u32),
}

impl IntoStyle for FontSize {
    #[inline]
    fn into_style(self) -> StyleAttribute {
        match self {
            FontSize::Pixel(value) => ("font-size", format!("{}px", value)).into_style(),
        }
    }
}
