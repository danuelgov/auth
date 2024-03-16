use email_template::{body, doctype, fragment_node, head, html, p};

pub struct EmailTemplate {
    pub email_address: String,
}

impl EmailTemplate {
    pub fn render(&self) -> String {
        fragment_node()
            .children(doctype())
            .children(html([
                head(),
                body("🔑 새로운 로그인이 확인되었습니다.")
                    .children(p().children(format!("새로운 로그인이 확인되었습니다.",))),
            ]))
            .render()
    }
}
