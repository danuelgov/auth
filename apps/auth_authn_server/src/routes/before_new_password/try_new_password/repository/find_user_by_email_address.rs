use auth_database::user_credential::{self, columns::UserCredentialPrimaryKey, UserCredential};
use database_toolkit::{DatabaseConnection, QueryBuilder};
use new_type::EmailAddress;
use sqlx::FromRow;

pub trait FindUserByEmailAddressContract {
    async fn find_user_by_email_address(
        &self,
        connection: DatabaseConnection,
        email_address: EmailAddress,
    ) -> Result<FindUserByEmailAddressEntity, FindUserByEmailAddressError>;
}

pub struct FindUserByEmailAddressEntity {
    pub user_credential_pk: UserCredentialPrimaryKey,
}

#[derive(Debug)]
pub enum FindUserByEmailAddressError {
    Database(sqlx::Error),
    NotFound,
    Row(sqlx::Error),
    UserCredentialPrimaryKey,
}

impl FindUserByEmailAddressContract for super::Repository {
    async fn find_user_by_email_address(
        &self,
        mut connection: DatabaseConnection,
        email_address: EmailAddress,
    ) -> Result<FindUserByEmailAddressEntity, FindUserByEmailAddressError> {
        let Some(row) = query(email_address)
            .build()
            .fetch_optional(&mut *connection)
            .await
            .map_err(FindUserByEmailAddressError::Database)?
        else {
            return Err(FindUserByEmailAddressError::NotFound);
        };

        #[derive(FromRow)]
        struct Row {
            user_credential_pk: Vec<u8>,
        }

        let Row { user_credential_pk } =
            Row::from_row(&row).map_err(FindUserByEmailAddressError::Row)?;
        let user_credential_pk = user_credential_pk
            .try_into()
            .map_err(|_| FindUserByEmailAddressError::UserCredentialPrimaryKey)?;

        Ok(FindUserByEmailAddressEntity { user_credential_pk })
    }
}

fn query<'q>(email_address: EmailAddress) -> QueryBuilder<'q> {
    QueryBuilder::new()
        .select(UserCredential, |builder| {
            builder.columns(&[user_credential::columns::USER_CREDENTIAL_PK])
        })
        .where_(|builder| {
            builder.condition(|builder| {
                let email_address: String = email_address.to_string();

                builder
                    .column(user_credential::columns::EXTERNAL_ID)
                    .eq()
                    .value(email_address)
            })
        })
}
