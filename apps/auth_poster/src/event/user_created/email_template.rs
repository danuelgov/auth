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
                body("🔑 새로운 계정이 생성 되었습니다.").children(p().children(format!(
                    "다뉴엘 거버넌스에 새로운 계정({})이 생성되었습니다.",
                    self.email_address
                ))),
            ]))
            .render()
    }
}
