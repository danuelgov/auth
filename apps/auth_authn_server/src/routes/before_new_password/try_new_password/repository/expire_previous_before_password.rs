use auth_database::{
    before_new_password::{self, BeforeNewPassword},
    user_credential::columns::UserCredentialPrimaryKey,
};
use chrono::{DateTime, Utc};
use database_toolkit::{QueryBuilder, Transaction};

pub trait ExpirePreviousBeforePasswordContract {
    async fn expire_previous_before_password(
        &self,
        transaction: &mut Transaction,
        user_credential_pk: UserCredentialPrimaryKey,
        now: DateTime<Utc>,
    ) -> Result<(), ExpirePreviousBeforePasswordError>;
}

#[derive(Debug)]
pub enum ExpirePreviousBeforePasswordError {
    Database(sqlx::Error),
}

impl ExpirePreviousBeforePasswordContract for super::Repository {
    async fn expire_previous_before_password(
        &self,
        transaction: &mut Transaction,
        user_credential_pk: UserCredentialPrimaryKey,
        now: DateTime<Utc>,
    ) -> Result<(), ExpirePreviousBeforePasswordError> {
        query(user_credential_pk, now)
            .build()
            .execute(&mut **transaction)
            .await
            .map_err(ExpirePreviousBeforePasswordError::Database)?;

        Ok(())
    }
}

fn query<'q>(user_credential_pk: UserCredentialPrimaryKey, now: DateTime<Utc>) -> QueryBuilder<'q> {
    QueryBuilder::new()
        .update(BeforeNewPassword)
        .set(before_new_password::columns::EXPIRED_AT, now)
        .where_(|builder| {
            builder
                .condition(|builder| {
                    let user_credential_pk: Vec<_> = user_credential_pk.into();

                    builder
                        .column(before_new_password::columns::USER_CREDENTIAL_PK)
                        .eq()
                        .value(user_credential_pk)
                })
                .and()
                .condition(|builder| {
                    builder
                        .column(before_new_password::columns::EXPIRED_AT)
                        .ge()
                        .value(now)
                })
        })
}
