pub mod create_before_new_password;
pub mod expire_previous_before_password;
pub mod find_user_by_email_address;
pub mod send_email;

use create_before_new_password::*;
use expire_previous_before_password::*;
use find_user_by_email_address::*;
use send_email::*;

pub trait RepositoryContract:
    CreateBeforeNewPasswordContract
    + ExpirePreviousBeforePasswordContract
    + FindUserByEmailAddressContract
    + SendEmailContract
{
    //
}

pub struct Repository;

impl RepositoryContract for Repository {
    //
}
