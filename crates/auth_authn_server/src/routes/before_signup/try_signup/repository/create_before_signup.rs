use auth_database::{
    agreement::columns::AgreementPrimaryKey,
    before_signup::{
        self,
        columns::{BeforeSignupIdentity, BeforeSignupPrimaryKey},
    },
    user_profile::columns::UserProfileName,
};
use database_toolkit::{DatabaseConnection, QueryBuilder};
use new_type::{EmailAddress, Password};

pub trait CreateBeforeSignupContract {
    async fn create_before_signup(
        &self,
        connection: DatabaseConnection,
        email_address: EmailAddress,
        password: Password,
        name: UserProfileName,
        agreements: Vec<AgreementPrimaryKey>,
    ) -> Result<BeforeSignupIdentity, CreateBeforeSignupError>;
}

#[derive(Debug)]
pub enum CreateBeforeSignupError {
    Database(sqlx::Error),
    Payload(serde_json::Error),
}

impl CreateBeforeSignupContract for super::Repository {
    async fn create_before_signup(
        &self,
        mut connection: DatabaseConnection,
        email_address: EmailAddress,
        password: Password,
        name: UserProfileName,
        agreements: Vec<AgreementPrimaryKey>,
    ) -> Result<BeforeSignupIdentity, CreateBeforeSignupError> {
        let before_signup_pk = BeforeSignupPrimaryKey::new();
        let before_signup_id = BeforeSignupIdentity::new();
        query(
            before_signup_pk,
            before_signup_id,
            email_address,
            password,
            name,
            agreements,
        )
        .build()
        .execute(&mut *connection)
        .await
        .map_err(CreateBeforeSignupError::Database)?;

        Ok(before_signup_id)
    }
}

fn query<'q>(
    before_signup_pk: BeforeSignupPrimaryKey,
    before_signup_id: BeforeSignupIdentity,
    email_address: EmailAddress,
    password: Password,
    name: UserProfileName,
    agreements: Vec<AgreementPrimaryKey>,
) -> QueryBuilder<'q> {
    #[derive(Serialize)]
    struct Payload {
        email_address: EmailAddress,
        password: Password,
        name: UserProfileName,
        agreements: Vec<AgreementPrimaryKey>,
    }

    let before_signup_pkp: Vec<u8> = before_signup_pk.into();
    let before_signup_id: Vec<u8> = before_signup_id.into();
    let payload = serde_json::to_string(&Payload {
        email_address,
        password,
        name,
        agreements,
    })
    .map_err(CreateBeforeSignupError::Payload)
    .unwrap();
    let now = chrono::Utc::now().naive_utc();
    let expired_at = now + chrono::Duration::days(1);

    QueryBuilder::new()
        .insert_into(
            before_signup::TABLE_NAME,
            &[
                before_signup::columns::BEFORE_SIGNUP_PK,
                before_signup::columns::ID,
                before_signup::columns::PAYLOAD,
                before_signup::columns::EXPIRED_AT,
            ],
        )
        .values(|builder| {
            builder.nested(|builder| {
                builder
                    .value(before_signup_pkp)
                    .comma()
                    .value(before_signup_id)
                    .comma()
                    .value(payload)
                    .comma()
                    .value(expired_at)
            })
        })
}
