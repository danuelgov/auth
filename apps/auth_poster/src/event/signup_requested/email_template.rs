use email_template::{body, doctype, fragment_node, head, html, link, p};

pub struct EmailTemplate {
    pub email_address: String,
    pub verification_url: String,
}

impl EmailTemplate {
    pub fn render(&self) -> String {
        fragment_node().children(doctype()).children(html([
            head(),
            body("🔑 회원가입 인증을 완료해 주세요.")
                .children(p().children(format!(
                    "아래 링크를 선택하면 다뉴엘 거버넌스에 {} 계정 생성이 완료 됩니다.",
                    self.email_address
                )))
                .children(link(&self.verification_url).children("이메일 인증하기"))
                .children(p().children("링크가 작동하지 않는 경우, 아래 링크를 직접 브라우저 주소창에 입력해 주세요"))
                .children(
                    link(&self.verification_url).children(self.verification_url.to_owned()),
                )
                .children(p().children(format!(
                    "이 이메일은 다뉴엘 거버넌스에서 {} 계정 생성을 요청한 것으로 확인됩니다",
                    self.email_address
                )))
                .children(p().children(
                    "이 이메일은 다뉴엘 거버넌스에서 회원가입을 요청한 것으로 확인됩니다. 이 이메일을 요청하지 않았다면 무시해 주세요."
                )),
        ]))
        .render()
    }
}
