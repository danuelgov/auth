use auth_database::user::{
    self,
    columns::{UserIdentity, UserPrimaryKey},
};
use database_toolkit::{QueryBuilder, Transaction};

pub trait CreateUserContract {
    async fn create_user(
        &self,
        transaction: &mut Transaction,
        user_pk: UserPrimaryKey,
        user_id: UserIdentity,
    ) -> Result<(), CreateUserError>;
}

#[derive(Debug)]
pub enum CreateUserError {
    Database(sqlx::Error),
}

impl CreateUserContract for super::Repository {
    async fn create_user(
        &self,
        transaction: &mut Transaction,
        user_pk: UserPrimaryKey,
        user_id: UserIdentity,
    ) -> Result<(), CreateUserError> {
        query(user_pk, user_id)
            .build()
            .execute(&mut **transaction)
            .await
            .map_err(CreateUserError::Database)?;

        Ok(())
    }
}

fn query<'args>(user_pk: UserPrimaryKey, user_id: UserIdentity) -> QueryBuilder<'args> {
    QueryBuilder::new()
        .insert_into(
            user::TABLE_NAME,
            &[user::columns::USER_PK, user::columns::ID],
        )
        .values(|builder| {
            builder.nested(|builder| {
                let user_pk: Vec<u8> = user_pk.into();
                let user_id: Vec<u8> = user_id.into();

                builder.separated([user_pk, user_id].into_iter(), |builder, value| {
                    builder.value(value)
                })
            })
        })
}
