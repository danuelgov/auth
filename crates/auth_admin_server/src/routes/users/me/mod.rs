use auth_internal::AUTH_AUTHN_SERVER_HOST;
use guard::{Authorization, Header};
use rocket::{http::Status, serde::json::Json};

#[get("/users/me")]
pub async fn handler(authorization: Header<Authorization<String>>) -> Result<Response, Status> {
    let response = reqwest::Client::new()
        .get(format!(
            "{}/internal/authn/users/me",
            AUTH_AUTHN_SERVER_HOST
        ))
        .header("Authorization", authorization.as_str())
        .send()
        .await
        .map_err(|_| Status::InternalServerError)?;
    let body: Body = response
        .json()
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(Response { body: Json(body) })
}

#[derive(Responder)]
pub struct Response {
    pub body: Json<Body>,
}

#[derive(Serialize, Deserialize)]
pub struct Body {
    pub user: User,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub handle: String,
    pub name: String,
    pub bio: String,
    pub image: String,
}
