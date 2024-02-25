use rocket::serde::json::Json;

#[derive(Responder)]
pub struct Response {
    pub body: Json<Body>,
}

#[derive(Serialize)]
pub struct Body {
    pub expired_password: bool,
}
