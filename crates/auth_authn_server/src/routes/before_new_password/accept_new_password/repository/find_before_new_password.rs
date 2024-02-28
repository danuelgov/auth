use auth_database::{
    before_new_password::{
        self,
        columns::{BeforeNewPasswordIdentity, BeforeNewPasswordPrimaryKey},
        BeforeNewPassword,
    },
    user_credential::{self, columns::UserCredentialPrimaryKey, UserCredential},
};
use chrono::{DateTime, Utc};
use database_toolkit::{DatabaseConnection, QueryBuilder};
use new_type::EmailAddress;
use sqlx::FromRow;

pub trait FindBeforeNewPasswordContract {
    async fn find_before_new_password(
        &self,
        connection: DatabaseConnection,
        before_new_password_id: BeforeNewPasswordIdentity,
        now: DateTime<Utc>,
    ) -> Result<FindBeforeNewPasswordEntity, FindBeforeNewPasswordError>;
}

pub struct FindBeforeNewPasswordEntity {
    pub before_new_password_pk: BeforeNewPasswordPrimaryKey,
    pub user_credential_pk: UserCredentialPrimaryKey,
    pub email_address: EmailAddress,
}

#[derive(Debug)]
pub enum FindBeforeNewPasswordError {
    Database(sqlx::Error),
    Used,
    Expired,
    NotFound,
    Row(sqlx::Error),
    BeforeNewPasswordPrimaryKey,
    UserCredentialPrimaryKey,
    EmailAddress,
}

impl FindBeforeNewPasswordContract for super::Repository {
    async fn find_before_new_password(
        &self,
        mut connection: DatabaseConnection,
        before_new_password_id: BeforeNewPasswordIdentity,
        now: DateTime<Utc>,
    ) -> Result<FindBeforeNewPasswordEntity, FindBeforeNewPasswordError> {
        let row = query(before_new_password_id)
            .build()
            .fetch_optional(&mut *connection)
            .await
            .map_err(FindBeforeNewPasswordError::Database)?
            .ok_or(FindBeforeNewPasswordError::NotFound)?;

        #[derive(FromRow)]
        struct Row {
            before_new_password_pk: Vec<u8>,
            user_credential_pk: Vec<u8>,
            expired_at: DateTime<Utc>,
            completed_at: DateTime<Utc>,
            external_id: String,
        }

        let Row {
            before_new_password_pk,
            user_credential_pk,
            expired_at,
            completed_at,
            external_id: email_address,
        } = Row::from_row(&row).map_err(FindBeforeNewPasswordError::Row)?;
        if completed_at < now {
            return Err(FindBeforeNewPasswordError::Used);
        }
        if expired_at < now {
            return Err(FindBeforeNewPasswordError::Expired);
        }

        let before_new_password_pk = before_new_password_pk
            .try_into()
            .map_err(|_| FindBeforeNewPasswordError::BeforeNewPasswordPrimaryKey)?;
        let user_credential_pk = user_credential_pk
            .try_into()
            .map_err(|_| FindBeforeNewPasswordError::UserCredentialPrimaryKey)?;
        let email_address = email_address
            .parse()
            .map_err(|_| FindBeforeNewPasswordError::EmailAddress)?;

        Ok(FindBeforeNewPasswordEntity {
            before_new_password_pk,
            user_credential_pk,
            email_address,
        })
    }
}

fn query<'q>(before_new_password_id: BeforeNewPasswordIdentity) -> QueryBuilder<'q> {
    QueryBuilder::new()
        .select(BeforeNewPassword, |builder| {
            builder
                .column(before_new_password::columns::BEFORE_NEW_PASSWORD_PK)
                .comma()
                .column(before_new_password::columns::USER_CREDENTIAL_PK)
                .comma()
                .column(before_new_password::columns::EXPIRED_AT)
                .comma()
                .column(before_new_password::columns::COMPLETED_AT)
                .comma()
                .column(user_credential::columns::EXTERNAL_ID)
        })
        .join(UserCredential, |builder| {
            builder.condition(|builder| {
                builder
                    .column(before_new_password::columns::USER_CREDENTIAL_PK)
                    .eq()
                    .column(user_credential::columns::USER_CREDENTIAL_PK)
            })
        })
        .where_(|builder| {
            builder.condition(|builder| {
                let before_new_password_id: Vec<_> = before_new_password_id.into();

                builder
                    .column(before_new_password::columns::ID)
                    .eq()
                    .value(before_new_password_id)
            })
        })
}
