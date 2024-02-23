use auth_database::{
    hasher::columns::HasherPrimaryKey, user_credential::columns::UserCredentialPrimaryKey,
    user_credential__has__hasher,
};
use database_toolkit::{QueryBuilder, Transaction};
use new_type::{Password, Salt};

pub trait CreateUserCredentialHasherContract {
    async fn create_user_credential_hasher(
        &self,
        transaction: &mut Transaction,
        user_credential_pk: UserCredentialPrimaryKey,
        password: Password,
        salt: Salt,
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
        password: Password,
        salt: Salt,
        expired_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), CreateUserCredentialHasherError> {
        query(user_credential_pk, password, salt, expired_at)
            .build()
            .execute(&mut **transaction)
            .await
            .map_err(CreateUserCredentialHasherError::Database)?;

        Ok(())
    }
}

fn query<'args>(
    user_credential_pk: UserCredentialPrimaryKey,
    password: Password,
    salt: Salt,
    expired_at: chrono::DateTime<chrono::Utc>,
) -> QueryBuilder<'args> {
    QueryBuilder::new()
        .insert_into(
            user_credential__has__hasher::TABLE_NAME,
            &[
                user_credential__has__hasher::columns::USER_CREDENTIAL_PK,
                user_credential__has__hasher::columns::HASHER_PK,
                user_credential__has__hasher::columns::HASH,
                user_credential__has__hasher::columns::SALT,
                user_credential__has__hasher::columns::EXPIRED_AT,
            ],
        )
        .values(|builder| {
            builder.nested(|builder| {
                let user_credential_pk: Vec<u8> = user_credential_pk.into();
                let hasher_pk: Vec<u8> = HasherPrimaryKey::ARGON2.into();
                let hash = password.as_str().to_owned();
                let salt: Vec<u8> = salt.into();
                let expired_at = expired_at;

                builder
                    .value(user_credential_pk)
                    .comma()
                    .value(hasher_pk)
                    .comma()
                    .value(hash)
                    .comma()
                    .value(salt)
                    .comma()
                    .value(expired_at)
            })
        })
}
