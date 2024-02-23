pub mod create_before_signup;
pub mod exists_email_address;
pub mod exists_user_name;
pub mod send_email;

use create_before_signup::*;
use exists_email_address::*;
use exists_user_name::*;
use send_email::*;

pub trait RepositoryContract:
    ExistsEmailAddressContract + ExistsUserNameContract + CreateBeforeSignupContract + SendEmailContract
{
    //
}

pub struct Repository;

impl RepositoryContract for Repository {
    //
}
