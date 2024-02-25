use auth_database::{
    hasher::columns::HasherPrimaryKey,
    user::columns::UserPrimaryKey,
    user_credential::{self, UserCredential},
    user_credential__has__hasher::{
        self,
        columns::{UserCredentialHasHasherExpiredAt, UserCredentialHasHasherHash},
        UserCredentialHasHasher,
    },
};
use database_toolkit::{DatabaseConnection, QueryBuilder};
use new_type::EmailAddress;
use sqlx::FromRow;

pub trait FindUserByEmailAddressContract {
    async fn find_password_by_email_address(
        &self,
        connection: DatabaseConnection,
        email_address: EmailAddress,
    ) -> Result<FindUserByEmailAddressEntity, FindUserByEmailAddressError>;
}

pub struct FindUserByEmailAddressEntity {
    pub user_pk: UserPrimaryKey,
    pub hasher_pk: HasherPrimaryKey,
    pub hash: UserCredentialHasHasherHash,
    pub expired_at: UserCredentialHasHasherExpiredAt,
}

#[derive(Debug)]
pub enum FindUserByEmailAddressError {
    Database(sqlx::Error),
    Row(sqlx::Error),
    EmailAddressNotFound,
    UserPrimaryKey,
    HasherPrimaryKey,
}

impl FindUserByEmailAddressContract for super::Repository {
    async fn find_password_by_email_address(
        &self,
        mut connection: DatabaseConnection,
        email_address: EmailAddress,
    ) -> Result<FindUserByEmailAddressEntity, FindUserByEmailAddressError> {
        #[derive(FromRow)]
        struct Row {
            user_pk: Vec<u8>,
            hasher_pk: Vec<u8>,
            hash: String,
            expired_at: chrono::NaiveDateTime,
        }

        let row = query(email_address)
            .build()
            .fetch_optional(&mut *connection)
            .await
            .map_err(FindUserByEmailAddressError::Database)?
            .ok_or(FindUserByEmailAddressError::EmailAddressNotFound)?;
        let row = Row::from_row(&row).map_err(FindUserByEmailAddressError::Row)?;
        let user_pk = row
            .user_pk
            .try_into()
            .map_err(|_| FindUserByEmailAddressError::UserPrimaryKey)?;
        let hasher_pk = HasherPrimaryKey::try_from(row.hasher_pk)
            .map_err(|_| FindUserByEmailAddressError::HasherPrimaryKey)?;
        let hash = row.hash.into();
        let expired_at = row.expired_at.into();

        Ok(FindUserByEmailAddressEntity {
            user_pk,
            hasher_pk,
            hash,
            expired_at,
        })
    }
}

fn query<'q>(email_address: EmailAddress) -> QueryBuilder<'q> {
    QueryBuilder::new()
        .select(UserCredential, |builder| {
            builder
                .column(user_credential::columns::USER_PK)
                .comma()
                .column(user_credential__has__hasher::columns::HASHER_PK)
                .comma()
                .column(user_credential__has__hasher::columns::HASH)
                .comma()
                .column(user_credential__has__hasher::columns::EXPIRED_AT)
        })
        .join(UserCredentialHasHasher, |builder| {
            builder
                .column(user_credential__has__hasher::columns::USER_CREDENTIAL_PK)
                .eq()
                .column(user_credential::columns::USER_CREDENTIAL_PK)
        })
        .where_(|builder| {
            let email_address = email_address.as_str().to_owned();

            builder.condition(|builder| {
                builder
                    .column(user_credential::columns::EXTERNAL_ID)
                    .eq()
                    .value(email_address)
            })
        })
}
