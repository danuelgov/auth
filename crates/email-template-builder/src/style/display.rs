use crate::{IntoStyle, StyleAttribute};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Display {
    None,
    Inline,
    #[default]
    Block,
    InlineBlock,
}

impl IntoStyle for Display {
    #[inline]
    fn into_style(self) -> StyleAttribute {
        match self {
            Display::None => ("display", "none").into_style(),
            Display::Inline => ("display", "inline").into_style(),
            Display::Block => ("display", "block").into_style(),
            Display::InlineBlock => ("display", "inline-block").into_style(),
        }
    }
}
