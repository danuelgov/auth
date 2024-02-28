use auth_database::before_new_password::{
    self, columns::BeforeNewPasswordPrimaryKey, BeforeNewPassword,
};
use chrono::{DateTime, Utc};
use database_toolkit::{QueryBuilder, Transaction};

pub trait ExpireBeforeNewPasswordContract {
    async fn expire_before_new_password(
        &self,
        transaction: &mut Transaction,
        before_new_password_pk: BeforeNewPasswordPrimaryKey,
        now: DateTime<Utc>,
    ) -> Result<(), ExpireBeforeNewPasswordError>;
}

#[derive(Debug)]
pub enum ExpireBeforeNewPasswordError {
    Database(sqlx::Error),
}

impl ExpireBeforeNewPasswordContract for super::Repository {
    async fn expire_before_new_password(
        &self,
        transaction: &mut Transaction,
        before_new_password_pk: BeforeNewPasswordPrimaryKey,
        now: DateTime<Utc>,
    ) -> Result<(), ExpireBeforeNewPasswordError> {
        query(before_new_password_pk, now)
            .build()
            .execute(&mut **transaction)
            .await
            .map_err(ExpireBeforeNewPasswordError::Database)?;

        Ok(())
    }
}

fn query<'q>(
    before_new_password_pk: BeforeNewPasswordPrimaryKey,
    now: DateTime<Utc>,
) -> QueryBuilder<'q> {
    QueryBuilder::new()
        .update(BeforeNewPassword)
        .set(before_new_password::columns::EXPIRED_AT, now)
        .where_(|builder| {
            builder.condition(|builder| {
                let before_new_password_pk: Vec<_> = before_new_password_pk.into();

                builder
                    .column(before_new_password::columns::BEFORE_NEW_PASSWORD_PK)
                    .eq()
                    .value(before_new_password_pk)
            })
        })
}
