pub mod create_session;
pub mod find_user_by_email_address;
pub mod send_event;

use create_session::*;
use find_user_by_email_address::*;
use send_event::*;

pub trait RepositoryContract:
    FindUserByEmailAddressContract + CreateSessionContract + SendEventContract
{
    //
}

pub struct Repository;

impl RepositoryContract for Repository {
    //
}
