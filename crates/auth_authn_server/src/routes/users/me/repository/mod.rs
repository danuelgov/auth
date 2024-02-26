pub mod find_user_profile_by_primary_key;

use find_user_profile_by_primary_key::*;

pub trait RepositoryContract: FindUserProfileByPrimaryKeyContract {
    //
}

pub struct Repository;

impl RepositoryContract for Repository {
    //
}
