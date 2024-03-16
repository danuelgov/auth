pub mod attribute;
pub mod charset;
pub mod dir;
pub mod http_equiv;
pub mod lang;
pub mod target;

pub use attribute::*;
pub use charset::*;
pub use dir::*;
pub use http_equiv::*;
pub use lang::*;
pub use target::*;

use crate::Style;

pub trait IntoNode: Clone {
    fn into_node(self) -> Node;
}

#[derive(Debug, Clone)]
pub enum Node {
    Fragment(FragmentNode),
    VoidElement(VoidElementNode),
    Element(ElementNode),
    Text(TextNode),
}

#[derive(Debug, Clone, Default)]
pub struct FragmentNode {
    pub(crate) children: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct VoidElementNode {
    pub(crate) name: &'static str,
    pub(crate) style: Style,
    pub(crate) attributes: Attributes,
}

#[derive(Debug, Clone)]
pub struct ElementNode {
    pub(crate) name: &'static str,
    pub(crate) style: Style,
    pub(crate) attributes: Attributes,
    pub(crate) children: FragmentNode,
}

#[derive(Debug, Clone)]
pub struct TextNode {
    pub(crate) value: String,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Fragment(node) => write!(f, "{}", node),
            Node::VoidElement(node) => write!(f, "{}", node),
            Node::Element(node) => write!(f, "{}", node),
            Node::Text(node) => write!(f, "{}", node),
        }
    }
}

impl std::fmt::Display for FragmentNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for child in &self.children {
            write!(f, "{}", child)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for VoidElementNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}", self.name)?;
        if !self.style.is_empty() {
            write!(f, " style=\"{}\"", self.style)?;
        }
        write!(f, "{}>", self.attributes)?;

        Ok(())
    }
}

impl std::fmt::Display for ElementNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}", self.name)?;
        if !self.style.is_empty() {
            write!(f, " style=\"{}\"", self.style)?;
        }
        write!(f, "{}>", self.attributes)?;
        write!(f, "{}", self.children)?;
        write!(f, "</{}>", self.name)?;

        Ok(())
    }
}

impl std::fmt::Display for TextNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)?;

        Ok(())
    }
}

impl std::fmt::Display for Attributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in &self.values {
            write!(f, " {}", key)?;
            if let Some(value) = value {
                write!(f, "=\"{}\"", value)?;
            }
        }

        Ok(())
    }
}

impl IntoNode for Node {
    #[inline]
    fn into_node(self) -> Node {
        self
    }
}

impl IntoNode for Vec<Node> {
    #[inline]
    fn into_node(self) -> Node {
        Node::Fragment(FragmentNode { children: self })
    }
}

impl<T> IntoNode for &[T]
where
    T: IntoNode,
{
    #[inline]
    fn into_node(self) -> Node {
        let mut children = Vec::with_capacity(self.len());
        for child in self {
            children.push(child.clone().into_node());
        }

        Node::Fragment(FragmentNode { children })
    }
}

impl<T, const N: usize> IntoNode for [T; N]
where
    T: IntoNode,
{
    #[inline]
    fn into_node(self) -> Node {
        let mut children = Vec::with_capacity(N);
        for child in self {
            children.push(child.into_node());
        }

        Node::Fragment(FragmentNode { children })
    }
}

impl<T, const N: usize> IntoNode for &[T; N]
where
    T: IntoNode,
{
    #[inline]
    fn into_node(self) -> Node {
        let mut children = Vec::with_capacity(N);
        for child in self {
            children.push(child.clone().into_node());
        }

        Node::Fragment(FragmentNode { children })
    }
}

impl IntoNode for FragmentNode {
    #[inline]
    fn into_node(self) -> Node {
        Node::Fragment(self)
    }
}

impl IntoNode for VoidElementNode {
    #[inline]
    fn into_node(self) -> Node {
        Node::VoidElement(self)
    }
}

impl IntoNode for ElementNode {
    #[inline]
    fn into_node(self) -> Node {
        Node::Element(self)
    }
}

impl IntoNode for TextNode {
    #[inline]
    fn into_node(self) -> Node {
        Node::Text(self)
    }
}

impl IntoNode for &str {
    #[inline]
    fn into_node(self) -> Node {
        Node::Text(TextNode {
            value: self.to_owned(),
        })
    }
}

impl IntoNode for String {
    #[inline]
    fn into_node(self) -> Node {
        Node::Text(TextNode { value: self })
    }
}

impl FragmentNode {
    #[inline]
    pub fn render(&self) -> String {
        self.to_string()
    }

    #[inline]
    pub fn children(mut self, node: impl IntoNode) -> Self {
        self.children.push(node.into_node());
        self
    }
}

impl VoidElementNode {
    #[inline]
    pub fn render(&self) -> String {
        self.to_string()
    }

    #[inline]
    pub fn attributes(mut self, attributes: Attributes) -> Self {
        self.attributes = self.attributes.merge(attributes);
        self
    }

    #[inline]
    pub fn attribute(mut self, attribute: impl IntoAttribute) -> Self {
        debug_assert_eq!(
            self.name, "style",
            "Use the style method to attach style attributes"
        );

        self.attributes = self.attributes.attach(attribute);
        self
    }

    #[inline]
    pub fn style(mut self, style: Style) -> Self {
        self.style = self.style.merge(style);
        self
    }
}

impl ElementNode {
    #[inline]
    pub fn render(&self) -> String {
        self.to_string()
    }

    #[inline]
    pub fn attributes(mut self, attributes: Attributes) -> Self {
        self.attributes = self.attributes.merge(attributes);
        self
    }

    #[inline]
    pub fn attribute(mut self, attribute: impl IntoAttribute) -> Self {
        debug_assert_eq!(
            self.name, "style",
            "Use the style method to attach style attributes"
        );

        self.attributes = self.attributes.attach(attribute);
        self
    }

    #[inline]
    pub fn style(mut self, style: Style) -> Self {
        self.style = self.style.merge(style);
        self
    }

    #[inline]
    pub fn children(mut self, node: impl IntoNode) -> Self {
        self.children = self.children.children(node);
        self
    }
}

impl TextNode {
    #[inline]
    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl Attributes {
    #[inline]
    pub fn attach(mut self, attribute: impl IntoAttribute) -> Self {
        let (name, value) = attribute.into_attribute();
        self.values.insert(name, value);
        self
    }

    #[inline]
    pub fn merge(mut self, attributes: Self) -> Self {
        self.values.extend(attributes.values);
        self
    }
}

#[inline]
pub fn fragment_node() -> FragmentNode {
    FragmentNode {
        children: Default::default(),
    }
}

#[inline]
pub fn void_element_node(name: &'static str) -> VoidElementNode {
    VoidElementNode {
        name,
        style: Default::default(),
        attributes: Default::default(),
    }
}

#[inline]
pub fn element_node(name: &'static str) -> ElementNode {
    ElementNode {
        name,
        style: Default::default(),
        attributes: Default::default(),
        children: Default::default(),
    }
}

#[inline]
pub fn text_node(value: impl Into<String>) -> TextNode {
    TextNode {
        value: value.into(),
    }
}

#[inline]
pub fn render(node: impl IntoNode) -> String {
    node.into_node().to_string()
}
