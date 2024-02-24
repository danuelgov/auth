use auth_database::{
    credential::columns::CredentialPrimaryKey,
    user::columns::UserPrimaryKey,
    user_credential::{self, columns::UserCredentialPrimaryKey, UserCredential},
};
use database_toolkit::{QueryBuilder, Transaction};
use new_type::EmailAddress;

pub trait CreateUserCredentialContract {
    async fn create_user_credential(
        &self,
        transaction: &mut Transaction,
        user_credential_pk: UserCredentialPrimaryKey,
        user_pk: UserPrimaryKey,
        email_address: EmailAddress,
    ) -> Result<(), CreateUserCredentialError>;
}

#[derive(Debug)]
pub enum CreateUserCredentialError {
    Database(sqlx::Error),
}

impl CreateUserCredentialContract for super::Repository {
    async fn create_user_credential(
        &self,
        transaction: &mut Transaction,
        user_credential_pk: UserCredentialPrimaryKey,
        user_pk: UserPrimaryKey,
        email_address: EmailAddress,
    ) -> Result<(), CreateUserCredentialError> {
        query(user_credential_pk, user_pk, email_address)
            .build()
            .execute(&mut **transaction)
            .await
            .map_err(CreateUserCredentialError::Database)?;

        Ok(())
    }
}

fn query<'args>(
    user_credential_pk: UserCredentialPrimaryKey,
    user_pk: UserPrimaryKey,
    email_address: EmailAddress,
) -> QueryBuilder<'args> {
    QueryBuilder::new()
        .insert_into(
            UserCredential,
            &[
                user_credential::columns::USER_CREDENTIAL_PK,
                user_credential::columns::USER_PK,
                user_credential::columns::CREDENTIAL_PK,
                user_credential::columns::EXTERNAL_ID,
            ],
        )
        .values(|builder| {
            builder.nested(|builder| {
                let user_credential_pk: Vec<u8> = user_credential_pk.into();
                let user_pk: Vec<u8> = user_pk.into();
                let credential_pk: Vec<u8> = CredentialPrimaryKey::EMAIL.into();
                let email_address: String = email_address.as_str().to_owned();

                builder
                    .value(user_credential_pk)
                    .comma()
                    .value(user_pk)
                    .comma()
                    .value(credential_pk)
                    .comma()
                    .value(email_address)
            })
        })
}
