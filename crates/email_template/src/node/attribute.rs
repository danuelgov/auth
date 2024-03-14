use std::{borrow::Cow, collections::HashMap};

use crate::Numeric;

pub trait IntoAttribute {
    fn into_attribute(self) -> Attribute;
}

#[derive(Debug, Clone, Default)]
pub struct Attributes {
    pub(crate) values: HashMap<Cow<'static, str>, Option<String>>,
}

pub type Attribute = (Cow<'static, str>, Option<String>);

impl IntoAttribute for Attribute {
    #[inline]
    fn into_attribute(self) -> Attribute {
        self
    }
}

impl IntoAttribute for &'static str {
    #[inline]
    fn into_attribute(self) -> Attribute {
        (self.into(), None)
    }
}

impl IntoAttribute for (&'static str, &str) {
    #[inline]
    fn into_attribute(self) -> Attribute {
        (self.0.into(), Some(self.1.into()))
    }
}

impl IntoAttribute for (&'static str, String) {
    #[inline]
    fn into_attribute(self) -> Attribute {
        (self.0.into(), Some(self.1))
    }
}

impl IntoAttribute for (&'static str, Option<&str>) {
    #[inline]
    fn into_attribute(self) -> Attribute {
        (self.0.into(), self.1.map(Into::into))
    }
}

impl IntoAttribute for (&'static str, Option<String>) {
    #[inline]
    fn into_attribute(self) -> Attribute {
        (self.0.into(), self.1)
    }
}

impl IntoAttribute for (&'static str, Attributes) {
    #[inline]
    fn into_attribute(self) -> Attribute {
        (self.0.into(), Some(self.1.to_string()))
    }
}

impl<T> IntoAttribute for (&'static str, T)
where
    T: Numeric,
{
    #[inline]
    fn into_attribute(self) -> Attribute {
        (self.0.into(), Some(self.1.to_string()))
    }
}

impl<T> IntoAttribute for (&'static str, Option<T>)
where
    T: Numeric,
{
    #[inline]
    fn into_attribute(self) -> Attribute {
        (self.0.into(), self.1.map(|value| value.to_string()))
    }
}

#[inline]
pub fn attributes() -> Attributes {
    Default::default()
}
