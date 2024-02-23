pub mod complete_before_signup;
pub mod create_user;
pub mod create_user_agreement;
pub mod create_user_credential;
pub mod create_user_credential_hasher;
pub mod create_user_profile;
pub mod find_before_signup;

use complete_before_signup::*;
use create_user::*;
use create_user_agreement::*;
use create_user_credential::*;
use create_user_credential_hasher::*;
use create_user_profile::*;
use find_before_signup::*;

pub trait RepositoryContract:
    FindBeforeSignupContract
    + CreateUserContract
    + CreateUserCredentialContract
    + CreateUserCredentialHasherContract
    + CreateUserProfileContract
    + CreateUserAgreementContract
    + CompleteBeforeSignupContract
{
    //
}

pub struct Repository;

impl RepositoryContract for Repository {
    //
}
