use crate::{IntoStyle, StyleAttribute};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FontFamily(pub &'static str);

impl IntoStyle for FontFamily {
    #[inline]
    fn into_style(self) -> StyleAttribute {
        ("font-family", self.0).into_style()
    }
}
