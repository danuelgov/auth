pub mod create_password;
pub mod expire_before_new_password;
pub mod expire_previous_password;
pub mod find_before_new_password;
pub mod send_email;

use create_password::*;
use expire_before_new_password::*;
use expire_previous_password::*;
use find_before_new_password::*;
use send_email::*;

pub trait RepositoryContract:
    CreatePasswordContract
    + ExpireBeforeNewPasswordContract
    + ExpirePreviousPasswordContract
    + FindBeforeNewPasswordContract
    + SendEmailContract
{
    //
}

pub struct Repository;

impl RepositoryContract for Repository {
    //
}
