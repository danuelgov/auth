use rocket::serde::json::Json;

#[derive(Responder)]
pub struct Response {
    pub body: Json<Body>,
}

#[derive(Serialize)]
pub struct Body {
    pub user: User,
}

#[derive(Serialize)]
pub struct User {
   pub handle: String,
   pub name: String,
   pub bio: String,
   pub image: String,
}
