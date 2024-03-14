use crate::{IntoStyle, StyleAttribute};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum MaxWidth {
    #[default]
    Auto,
    Zero,
    Pixel(u32),
    Point(u32),
    Percentage(u32),
}

impl IntoStyle for MaxWidth {
    #[inline]
    fn into_style(self) -> StyleAttribute {
        match self {
            MaxWidth::Auto => ("max-width", "auto").into_style(),
            MaxWidth::Zero => ("max-width", "0").into_style(),
            MaxWidth::Pixel(value) => ("max-width", format!("{}px", value)).into_style(),
            MaxWidth::Point(value) => ("max-width", format!("{}pt", value)).into_style(),
            MaxWidth::Percentage(value) => ("max-width", format!("{}%", value)).into_style(),
        }
    }
}
