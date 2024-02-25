pub mod create_session;
pub mod find_user_by_email_address;

use create_session::*;
use find_user_by_email_address::*;

pub trait RepositoryContract: FindUserByEmailAddressContract + CreateSessionContract {
    //
}

pub struct Repository;

impl RepositoryContract for Repository {
    //
}
