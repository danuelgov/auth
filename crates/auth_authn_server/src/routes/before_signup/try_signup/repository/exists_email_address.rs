use auth_database::{credential::columns::CredentialPrimaryKey, user_credential};
use database_toolkit::{DatabaseConnection, QueryBuilder};
use new_type::EmailAddress;
use sqlx::FromRow;

pub trait ExistsEmailAddressContract {
    async fn exists_email_address(
        &self,
        connection: DatabaseConnection,
        email_address: EmailAddress,
    ) -> Result<bool, ExistsEmailAddressError>;
}

#[derive(Debug)]
pub enum ExistsEmailAddressError {
    Database(sqlx::Error),
    Row(sqlx::Error),
}

impl ExistsEmailAddressContract for super::Repository {
    async fn exists_email_address(
        &self,
        mut connection: DatabaseConnection,
        email_address: EmailAddress,
    ) -> Result<bool, ExistsEmailAddressError> {
        let row = query(email_address)
            .build()
            .fetch_one(&mut *connection)
            .await
            .map_err(ExistsEmailAddressError::Database)?;

        #[derive(Debug, FromRow)]
        struct Row {
            count: i32,
        }

        let Row { count } = Row::from_row(&row).map_err(ExistsEmailAddressError::Row)?;

        Ok(count != 0)
    }
}

fn query<'q>(email_address: EmailAddress) -> QueryBuilder<'q> {
    let credential_pk: Vec<u8> = CredentialPrimaryKey::EMAIL.into();
    let email_address = email_address.as_str().to_owned();

    QueryBuilder::new()
        .select()
        .write("COUNT(*)")
        .alias("count")
        .from(user_credential::TABLE_NAME)
        .where_(|builder| {
            builder
                .condition(|builder| {
                    builder
                        .column(user_credential::columns::CREDENTIAL_PK)
                        .eq()
                        .value(credential_pk)
                })
                .and()
                .condition(|builder| {
                    builder
                        .column(user_credential::columns::EXTERNAL_ID)
                        .eq()
                        .value(email_address)
                })
        })
}
