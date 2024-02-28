use auth_database::{
    before_new_password::{
        self,
        columns::{BeforeNewPasswordIdentity, BeforeNewPasswordPrimaryKey},
        BeforeNewPassword,
    },
    user_credential::columns::UserCredentialPrimaryKey,
};
use chrono::{DateTime, Utc};
use database_toolkit::{QueryBuilder, Transaction};

pub trait CreateBeforeNewPasswordContract {
    async fn create_before_new_password(
        &self,
        transaction: &mut Transaction,
        before_new_password_pk: BeforeNewPasswordPrimaryKey,
        before_new_password_id: BeforeNewPasswordIdentity,
        user_credential_pk: UserCredentialPrimaryKey,
        expired_at: DateTime<Utc>,
    ) -> Result<(), CreateBeforeNewPasswordError>;
}

#[derive(Debug)]
pub enum CreateBeforeNewPasswordError {
    Database(sqlx::Error),
}

impl CreateBeforeNewPasswordContract for super::Repository {
    async fn create_before_new_password(
        &self,
        transaction: &mut Transaction,
        before_new_password_pk: BeforeNewPasswordPrimaryKey,
        before_new_password_id: BeforeNewPasswordIdentity,
        user_credential_pk: UserCredentialPrimaryKey,
        expired_at: DateTime<Utc>,
    ) -> Result<(), CreateBeforeNewPasswordError> {
        query(
            before_new_password_pk,
            before_new_password_id,
            user_credential_pk,
            expired_at,
        )
        .build()
        .execute(&mut **transaction)
        .await
        .map_err(CreateBeforeNewPasswordError::Database)?;

        Ok(())
    }
}

fn query<'q>(
    before_new_password_pk: BeforeNewPasswordPrimaryKey,
    before_new_password_id: BeforeNewPasswordIdentity,
    user_credential_pk: UserCredentialPrimaryKey,
    expired_at: DateTime<Utc>,
) -> QueryBuilder<'q> {
    QueryBuilder::new()
        .insert_into(
            BeforeNewPassword,
            &[
                before_new_password::columns::BEFORE_NEW_PASSWORD_PK,
                before_new_password::columns::ID,
                before_new_password::columns::USER_CREDENTIAL_PK,
                before_new_password::columns::EXPIRED_AT,
            ],
        )
        .values(|builder| {
            builder.nested(|builder| {
                let before_new_password_pk: Vec<u8> = before_new_password_pk.into();
                let before_new_password_id: Vec<u8> = before_new_password_id.into();
                let user_credential_pk: Vec<u8> = user_credential_pk.into();

                builder
                    .value(before_new_password_pk)
                    .comma()
                    .value(before_new_password_id)
                    .comma()
                    .value(user_credential_pk)
                    .comma()
                    .value(expired_at)
            })
        })
}
