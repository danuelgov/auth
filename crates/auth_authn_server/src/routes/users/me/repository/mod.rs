pub mod extend_session;
pub mod find_user_profile_by_primary_key;

use extend_session::*;
use find_user_profile_by_primary_key::*;

pub trait RepositoryContract: FindUserProfileByPrimaryKeyContract + ExtendSessionContract {
    //
}

pub struct Repository;

impl RepositoryContract for Repository {
    //
}
