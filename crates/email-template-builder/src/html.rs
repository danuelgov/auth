use crate::{
    element_node, style, text_node, void_element_node, Color, Display, ElementNode, FontFamily,
    FontSize, IntoNode, LineHeight, Margin, MaxHeight, MaxWidth, Opacity, Overflow, TextDecoration,
    VoidElementNode,
};

#[inline]
pub fn doctype() -> VoidElementNode {
    void_element_node(
        r#"!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd""#,
    )
}

#[inline]
pub fn html<Children>(children: Children) -> ElementNode
where
    Children: IntoNode,
{
    element_node("html")
        .attribute(("dir", "ltr"))
        .attribute(("lang", "en"))
        .children(children)
}

#[inline]
pub fn head() -> ElementNode {
    element_node("head").children(
        meta()
            .attribute(("charset", "UTF-8"))
            .attribute(("http-equiv", "X-UA-Compatible")),
    )
}

#[inline]
pub fn meta() -> VoidElementNode {
    void_element_node("meta")
}

#[inline]
pub fn title() -> ElementNode {
    element_node("title")
}

#[inline]
pub fn body(preview: &str) -> ElementNode {
    element_node("body").children(crate::preview(preview))
}

pub fn preview(preview: &str) -> ElementNode {
    const PREVIEW_MAX_LEN: usize = 150;
    const WHITESPACE: &str = "\u{00A0}\u{200C}\u{200B}\u{200D}\u{200E}\u{200F}\u{FEFF}";

    let preview_len = preview.chars().count();
    let preview = div().children([text_node(preview)]).style(
        style()
            .attach(Display::None)
            .attach(Overflow::Hidden)
            .attach(LineHeight::Pixel(1))
            .attach(Opacity::Zero)
            .attach(MaxHeight::Zero)
            .attach(MaxWidth::Zero),
    );

    if preview_len >= PREVIEW_MAX_LEN {
        preview
    } else {
        preview.children(text_node(WHITESPACE.repeat(PREVIEW_MAX_LEN - preview_len)))
    }
}

#[inline]
pub fn div() -> ElementNode {
    element_node("div")
}

#[inline]
pub fn p() -> ElementNode {
    element_node("p")
}

#[inline]
pub fn text() -> ElementNode {
    p().style(
        style()
            .attach(FontSize::Pixel(14))
            .attach(LineHeight::Pixel(24))
            .attach(Margin::Vertical(16))
            .attach(FontFamily(
                r#""Google Sans",Roboto,RobotoDraft,Helvetica,Arial,sans-serif"#,
            )),
    )
}

#[inline]
pub fn a() -> ElementNode {
    element_node("a")
}

#[inline]
pub fn link(to: &str) -> ElementNode {
    a().style(
        style()
            .attach(Color::Hex(0x067df7))
            .attach(TextDecoration::Underline),
    )
    .attribute(("target", "_blank"))
    .attribute(("href", to))
}
