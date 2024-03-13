use crate::{IntoStyle, StyleAttribute};

#[derive(Debug, Clone, PartialEq)]
pub enum MaxHeight {
    Auto,
    Zero,
    Pixel(u32),
    Point(u32),
    Percentage(u32),
}

impl IntoStyle for MaxHeight {
    #[inline]
    fn into_style(self) -> StyleAttribute {
        match self {
            MaxHeight::Auto => ("max-height", "auto").into_style(),
            MaxHeight::Zero => ("max-height", "0").into_style(),
            MaxHeight::Pixel(value) => ("max-height", format!("{}px", value)).into_style(),
            MaxHeight::Point(value) => ("max-height", format!("{}pt", value)).into_style(),
            MaxHeight::Percentage(value) => ("max-height", format!("{}%", value)).into_style(),
        }
    }
}
