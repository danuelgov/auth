use auth_database::{
    hasher::columns::HasherPrimaryKey,
    user_credential::columns::UserCredentialPrimaryKey,
    user_credential__has__hasher::{self, UserCredentialHasHasher},
};
use database_toolkit::{QueryBuilder, Transaction};
use new_type::Hash;

pub trait CreateUserCredentialHasherContract {
    async fn create_user_credential_hasher(
        &self,
        transaction: &mut Transaction,
        user_credential_pk: UserCredentialPrimaryKey,
        hasher_pk: HasherPrimaryKey,
        hash: Hash,
        expired_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), CreateUserCredentialHasherError>;
}

#[derive(Debug)]
pub enum CreateUserCredentialHasherError {
    Database(sqlx::Error),
}

impl CreateUserCredentialHasherContract for super::Repository {
    async fn create_user_credential_hasher(
        &self,
        transaction: &mut Transaction,
        user_credential_pk: UserCredentialPrimaryKey,
        hasher_pk: HasherPrimaryKey,
        hash: Hash,
        expired_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), CreateUserCredentialHasherError> {
        query(user_credential_pk, hasher_pk, hash, expired_at)
            .build()
            .execute(&mut **transaction)
            .await
            .map_err(CreateUserCredentialHasherError::Database)?;

        Ok(())
    }
}

fn query<'args>(
    user_credential_pk: UserCredentialPrimaryKey,
    hasher_pk: HasherPrimaryKey,
    hash: Hash,
    expired_at: chrono::DateTime<chrono::Utc>,
) -> QueryBuilder<'args> {
    QueryBuilder::new()
        .insert_into(
            UserCredentialHasHasher,
            &[
                user_credential__has__hasher::columns::USER_CREDENTIAL_PK,
                user_credential__has__hasher::columns::HASHER_PK,
                user_credential__has__hasher::columns::HASH,
                user_credential__has__hasher::columns::EXPIRED_AT,
            ],
        )
        .values(|builder| {
            builder.nested(|builder| {
                let user_credential_pk: Vec<u8> = user_credential_pk.into();
                let hasher_pk: Vec<u8> = hasher_pk.into();
                let hash = hash.as_str().to_owned();
                let expired_at = expired_at;

                builder
                    .value(user_credential_pk)
                    .comma()
                    .value(hasher_pk)
                    .comma()
                    .value(hash)
                    .comma()
                    .value(expired_at)
            })
        })
}
