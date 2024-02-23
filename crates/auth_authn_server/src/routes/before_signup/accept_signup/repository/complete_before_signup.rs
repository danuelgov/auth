use auth_database::before_signup::{self, columns::BeforeSignupPrimaryKey};
use database_toolkit::{QueryBuilder, Transaction};

pub trait CompleteBeforeSignupContract {
    async fn complete_before_signup(
        &self,
        transaction: &mut Transaction,
        before_signup_pk: BeforeSignupPrimaryKey,
    ) -> Result<(), CompleteBeforeSignupError>;
}

#[derive(Debug)]
pub enum CompleteBeforeSignupError {
    Database(sqlx::Error),
}

impl CompleteBeforeSignupContract for super::Repository {
    async fn complete_before_signup(
        &self,
        transaction: &mut Transaction,
        before_signup_pk: BeforeSignupPrimaryKey,
    ) -> Result<(), CompleteBeforeSignupError> {
        query(before_signup_pk)
            .build()
            .execute(&mut **transaction)
            .await
            .map_err(CompleteBeforeSignupError::Database)?;

        Ok(())
    }
}

fn query<'q>(before_signup_pk: BeforeSignupPrimaryKey) -> QueryBuilder<'q> {
    let now = chrono::Utc::now().naive_utc();
    let before_signup_pk: Vec<u8> = before_signup_pk.into();

    QueryBuilder::new()
        .update(before_signup::TABLE_NAME)
        .set()
        .assign_value(before_signup::columns::COMPLETED_AT, now)
        .where_(|builder| {
            builder.condition(|builder| {
                builder
                    .column(before_signup::columns::BEFORE_SIGNUP_PK)
                    .eq()
                    .value(before_signup_pk)
            })
        })
}
