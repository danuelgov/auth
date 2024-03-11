use auth_database::{
    user_credential::columns::UserCredentialPrimaryKey,
    user_credential__has__hasher::{self, UserCredentialHasHasher},
};
use chrono::{DateTime, Utc};
use database_toolkit::{QueryBuilder, Transaction};

pub trait ExpirePreviousPasswordContract {
    async fn expire_previous_password(
        &self,
        transaction: &mut Transaction,
        user_credential_pk: UserCredentialPrimaryKey,
        now: DateTime<Utc>,
    ) -> Result<(), ExpirePreviousPasswordError>;
}

#[derive(Debug)]
pub enum ExpirePreviousPasswordError {
    Database(sqlx::Error),
}

impl ExpirePreviousPasswordContract for super::Repository {
    async fn expire_previous_password(
        &self,
        transaction: &mut Transaction,
        user_credential_pk: UserCredentialPrimaryKey,
        now: DateTime<Utc>,
    ) -> Result<(), ExpirePreviousPasswordError> {
        query(user_credential_pk, now)
            .build()
            .execute(&mut **transaction)
            .await
            .map_err(ExpirePreviousPasswordError::Database)?;

        Ok(())
    }
}

fn query<'q>(user_credential_pk: UserCredentialPrimaryKey, now: DateTime<Utc>) -> QueryBuilder<'q> {
    QueryBuilder::new()
        .update(UserCredentialHasHasher)
        .set(user_credential__has__hasher::columns::EXPIRED_AT, now)
        .where_(|builder| {
            builder
                .condition(|builder| {
                    let user_credential_pk: Vec<_> = user_credential_pk.into();

                    builder
                        .column(user_credential__has__hasher::columns::USER_CREDENTIAL_PK)
                        .eq()
                        .value(user_credential_pk)
                })
                .and()
                .condition(|builder| {
                    builder
                        .column(user_credential__has__hasher::columns::EXPIRED_AT)
                        .ge()
                        .value(now)
                })
        })
}
