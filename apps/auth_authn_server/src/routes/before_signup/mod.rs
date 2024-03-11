mod accept_signup;
mod try_signup;

use rocket::{Build, Rocket};

// 회원가입 시도 -> 회원가입 인증 이메일 발송 -> 회원가입 인증 이메일 확인 -> 회원가입 완료
pub fn routes(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/", routes![accept_signup::handler, try_signup::handler])
}
