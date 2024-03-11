pub mod exists_before_new_password_id;

use exists_before_new_password_id::*;

pub trait RepositoryContract: ExistsBeforeNewPasswordIdContract {
    //
}

pub struct Repository;

impl RepositoryContract for Repository {
    //
}
