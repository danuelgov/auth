use crate::{IntoStyle, StyleAttribute};

pub const DEFAULT_FONT_FAMILY: FontFamily =
    FontFamily(r#""Google Sans",Roboto,RobotoDraft,Helvetica,Arial,sans-serif"#);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FontFamily(pub &'static str);

impl IntoStyle for FontFamily {
    #[inline]
    fn into_style(self) -> StyleAttribute {
        ("font-family", self.0).into_style()
    }
}
