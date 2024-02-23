use auth_database::user;

#[derive(Debug)]
pub struct Session {
    pub user_id: user::Identity,
}
