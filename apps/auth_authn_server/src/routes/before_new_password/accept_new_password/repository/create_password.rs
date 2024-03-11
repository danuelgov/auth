use auth_database::{
    hasher::columns::HasherPrimaryKey,
    user_credential::columns::UserCredentialPrimaryKey,
    user_credential__has__hasher::{
        self, columns::UserCredentialHasHasherPrimaryKey, UserCredentialHasHasher,
    },
};
use chrono::{DateTime, Utc};
use database_toolkit::{QueryBuilder, Transaction};
use new_type::Hash;

pub trait CreatePasswordContract {
    async fn create_password(
        &self,
        transaction: &mut Transaction,
        user_credential__has__hasher_pk: UserCredentialHasHasherPrimaryKey,
        user_credential_pk: UserCredentialPrimaryKey,
        hasher_pk: HasherPrimaryKey,
        hash: Hash,
        expired_at: DateTime<Utc>,
    ) -> Result<(), CreatePasswordError>;
}

#[derive(Debug)]
pub enum CreatePasswordError {
    Database(sqlx::Error),
}

impl CreatePasswordContract for super::Repository {
    async fn create_password(
        &self,
        transaction: &mut Transaction,
        user_credential__has__hasher_pk: UserCredentialHasHasherPrimaryKey,
        user_credential_pk: UserCredentialPrimaryKey,
        hasher_pk: HasherPrimaryKey,
        hash: Hash,
        expired_at: DateTime<Utc>,
    ) -> Result<(), CreatePasswordError> {
        query(
            user_credential__has__hasher_pk,
            user_credential_pk,
            hasher_pk,
            hash,
            expired_at,
        )
        .build()
        .execute(&mut **transaction)
        .await
        .map_err(CreatePasswordError::Database)?;

        Ok(())
    }
}

fn query<'q>(
    user_credential__has__hasher_pk: UserCredentialHasHasherPrimaryKey,
    user_credential_pk: UserCredentialPrimaryKey,
    hasher_pk: HasherPrimaryKey,
    hash: Hash,
    expired_at: DateTime<Utc>,
) -> QueryBuilder<'q> {
    QueryBuilder::new()
        .insert_into(
            UserCredentialHasHasher,
            &[
                user_credential__has__hasher::columns::USER_CREDENTIAL__HAS__HASHER_PK,
                user_credential__has__hasher::columns::USER_CREDENTIAL_PK,
                user_credential__has__hasher::columns::HASHER_PK,
                user_credential__has__hasher::columns::HASH,
                user_credential__has__hasher::columns::EXPIRED_AT,
            ],
        )
        .values(|builder| {
            builder.nested(|builder| {
                let user_credential__has__hasher_pk: Vec<_> =
                    user_credential__has__hasher_pk.into();
                let user_credential_pk: Vec<_> = user_credential_pk.into();
                let hasher_pk: Vec<_> = hasher_pk.into();
                let hash = hash.as_str().to_owned();

                builder
                    .value(user_credential__has__hasher_pk)
                    .comma()
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
