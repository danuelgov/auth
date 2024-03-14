pub mod color;
pub mod display;
pub mod font_family;
pub mod font_size;
pub mod line_height;
pub mod margin;
pub mod max_height;
pub mod max_width;
pub mod opacity;
pub mod overflow;
pub mod text_decoration;

pub use color::*;
pub use display::*;
pub use font_family::*;
pub use font_size::*;
pub use line_height::*;
pub use margin::*;
pub use max_height::*;
pub use max_width::*;
pub use opacity::*;
pub use overflow::*;
pub use text_decoration::*;

use crate::Numeric;
use std::{borrow::Cow, collections::HashMap};

pub trait IntoStyle {
    fn into_style(self) -> StyleAttribute;
}

pub type StyleAttribute = (Cow<'static, str>, String);

#[derive(Debug, Clone, Default)]
pub struct Style {
    values: HashMap<Cow<'static, str>, String>,
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in &self.values {
            write!(f, "{}: {};", key, value)?;
        }

        Ok(())
    }
}

impl IntoStyle for StyleAttribute {
    #[inline]
    fn into_style(self) -> StyleAttribute {
        self
    }
}

impl IntoStyle for (&'static str, &str) {
    #[inline]
    fn into_style(self) -> StyleAttribute {
        (self.0.into(), self.1.into())
    }
}

impl IntoStyle for (&'static str, String) {
    #[inline]
    fn into_style(self) -> StyleAttribute {
        (self.0.into(), self.1)
    }
}

impl<T> IntoStyle for (&'static str, T)
where
    T: Numeric,
{
    #[inline]
    fn into_style(self) -> StyleAttribute {
        (self.0.into(), self.1.to_string())
    }
}

impl Style {
    #[inline]
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    #[inline]
    pub fn attach(mut self, style: impl IntoStyle) -> Self {
        let (name, value) = style.into_style();
        self.values.insert(name, value);
        self
    }

    #[inline]
    pub fn merge(mut self, style: Style) -> Self {
        self.values.extend(style.values);
        self
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[inline]
pub fn style() -> Style {
    Default::default()
}
