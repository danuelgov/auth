use crate::{IntoStyle, StyleAttribute};

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Transparent,
    Hex(u32),
    Hexa(u32, f32),
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, f32),
}

impl IntoStyle for Color {
    #[inline]
    fn into_style(self) -> StyleAttribute {
        match self {
            Color::Transparent => ("color", "transparent").into_style(),
            Color::Hex(value) => ("color", format!("#{:06X}", value)).into_style(),
            Color::Hexa(value, alpha) => (
                "color",
                format!("#{:06X}{:02X}", value, (alpha * 255.0) as u8),
            )
                .into_style(),
            Color::Rgb(r, g, b) => ("color", format!("rgb({},{},{})", r, g, b)).into_style(),
            Color::Rgba(r, g, b, alpha) => {
                ("color", format!("rgba({},{},{},{})", r, g, b, alpha)).into_style()
            }
        }
    }
}
