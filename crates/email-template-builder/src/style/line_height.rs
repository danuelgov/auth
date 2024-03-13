use crate::{IntoStyle, StyleAttribute};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineHeight {
    Auto,
    Normal,
    Pixel(u32),
    Point(u32),
    Percentage(u32),
}

impl IntoStyle for LineHeight {
    #[inline]
    fn into_style(self) -> StyleAttribute {
        match self {
            LineHeight::Auto => ("line-height", "auto").into_style(),
            LineHeight::Normal => ("line-height", "normal").into_style(),
            LineHeight::Pixel(value) => ("line-height", format!("{}px", value)).into_style(),
            LineHeight::Point(value) => ("line-height", format!("{}pt", value)).into_style(),
            LineHeight::Percentage(value) => ("line-height", format!("{}%", value)).into_style(),
        }
    }
}
