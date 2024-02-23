mod add_new_password;
mod try_new_password;

use rocket::{Build, Rocket};

// 이메일 인증 메일 발송 -> 이메일 인증 -> 비밀번호 입력 -> 비밀번호 변경
pub fn routes(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount(
        "/",
        routes![add_new_password::handler, try_new_password::handler],
    )
}
