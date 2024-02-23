use auth_database::{
    user::columns::UserPrimaryKey,
    user_profile::{self, columns::UserProfileName},
};
use database_toolkit::{QueryBuilder, Transaction};
use new_type::Handle;

pub trait CreateUserProfileContract {
    async fn create_user_profile(
        &self,
        transaction: &mut Transaction,
        user_pk: UserPrimaryKey,
        handle: Handle,
        name: UserProfileName,
    ) -> Result<(), CreateUserProfileError>;
}

#[derive(Debug)]
pub enum CreateUserProfileError {
    Database(sqlx::Error),
}

impl CreateUserProfileContract for super::Repository {
    async fn create_user_profile(
        &self,
        transaction: &mut Transaction,
        user_pk: UserPrimaryKey,
        handle: Handle,
        name: UserProfileName,
    ) -> Result<(), CreateUserProfileError> {
        query(user_pk, handle, name)
            .build()
            .execute(&mut **transaction)
            .await
            .map_err(CreateUserProfileError::Database)?;

        Ok(())
    }
}

fn query<'args>(
    user_pk: UserPrimaryKey,
    handle: Handle,
    name: UserProfileName,
) -> database_toolkit::QueryBuilder<'args> {
    QueryBuilder::new()
        .insert_into(
            user_profile::TABLE_NAME,
            &[
                user_profile::columns::USER_PK,
                user_profile::columns::HANDLE,
                user_profile::columns::NAME,
                user_profile::columns::BIO,
                user_profile::columns::IMAGE,
            ],
        )
        .values(|builder| {
            builder.nested(|builder| {
                let user_pk: Vec<u8> = user_pk.into();
                let handle = handle.as_str().to_owned();
                let name = name.as_str().to_owned();

                builder
                    .value(user_pk)
                    .comma()
                    .value(handle)
                    .comma()
                    .value(name)
                    .comma()
                    .value("")
                    .comma()
                    .value("")
            })
        })
}
