use auth_database::{
    agreement::columns::AgreementPrimaryKey,
    before_signup::{
        self,
        columns::{BeforeSignupIdentity, BeforeSignupPrimaryKey},
        BeforeSignup,
    },
    hasher::columns::HasherPrimaryKey,
    user_profile::columns::UserProfileName,
};
use database_toolkit::{DatabaseConnection, QueryBuilder};
use new_type::{EmailAddress, Hash};
use sqlx::FromRow;

pub trait FindBeforeSignupContract {
    async fn find_before_signup(
        &self,
        connection: DatabaseConnection,
        before_signup_id: BeforeSignupIdentity,
    ) -> Result<BeforeSignupData, FindBeforeSignupError>;
}

#[derive(Debug)]
pub struct BeforeSignupData {
    pub before_signup_pk: BeforeSignupPrimaryKey,
    pub email_address: EmailAddress,
    pub hasher_pk: HasherPrimaryKey,
    pub hash: Hash,
    pub name: UserProfileName,
    pub agreements: Vec<AgreementPrimaryKey>,
}

#[derive(Debug)]
pub enum FindBeforeSignupError {
    Database(sqlx::Error),
    NotFound,
    Row(sqlx::Error),
    Expired,
    Payload(serde_json::Error),
    BeforeSignupPrimaryKey,
}

impl FindBeforeSignupContract for super::Repository {
    async fn find_before_signup(
        &self,
        mut connection: DatabaseConnection,
        before_signup_id: BeforeSignupIdentity,
    ) -> Result<BeforeSignupData, FindBeforeSignupError> {
        #[derive(Debug, FromRow)]
        struct Row {
            before_signup_pk: Vec<u8>,
            payload: String,
            expired_at: chrono::NaiveDateTime,
            completed_at: chrono::NaiveDateTime,
        }

        match query(before_signup_id)
            .build()
            .fetch_optional(&mut *connection)
            .await
            .map_err(FindBeforeSignupError::Database)?
            .map(|row| Row::from_row(&row).map_err(FindBeforeSignupError::Row))
        {
            Some(Ok(row)) => {
                let now = chrono::Utc::now().naive_utc();
                if row.completed_at <= now || row.expired_at <= now {
                    return Err(FindBeforeSignupError::Expired);
                }

                #[derive(Deserialize)]
                struct Payload {
                    credential: CredentialPayload,
                    hasher: HasherPayload,
                    profile: ProfilePayload,
                    agreements: Vec<AgreementPrimaryKey>,
                }

                #[derive(Deserialize)]
                struct CredentialPayload {
                    email_address: EmailAddress,
                }

                #[derive(Deserialize)]
                struct HasherPayload {
                    hasher_pk: HasherPrimaryKey,
                    hash: Hash,
                }

                #[derive(Deserialize)]
                struct ProfilePayload {
                    name: UserProfileName,
                }

                let Payload {
                    credential: CredentialPayload { email_address },
                    hasher: HasherPayload { hasher_pk, hash },
                    profile: ProfilePayload { name },
                    agreements,
                } = serde_json::from_str(&row.payload).map_err(FindBeforeSignupError::Payload)?;
                let before_signup_pk: BeforeSignupPrimaryKey = row
                    .before_signup_pk
                    .try_into()
                    .map_err(|_| FindBeforeSignupError::BeforeSignupPrimaryKey)?;

                Ok(BeforeSignupData {
                    before_signup_pk,
                    email_address,
                    hasher_pk,
                    hash,
                    name,
                    agreements,
                })
            }
            Some(Err(error)) => Err(error),
            None => Err(FindBeforeSignupError::NotFound),
        }
    }
}

fn query<'q>(before_signup_id: BeforeSignupIdentity) -> QueryBuilder<'q> {
    let before_signup_id: Vec<u8> = before_signup_id.into();

    QueryBuilder::new()
        .select(BeforeSignup, |builder| {
            builder.columns(&[
                before_signup::columns::BEFORE_SIGNUP_PK,
                before_signup::columns::PAYLOAD,
                before_signup::columns::EXPIRED_AT,
                before_signup::columns::COMPLETED_AT,
            ])
        })
        .where_(|builder| {
            builder.condition(|builder| {
                builder
                    .column(before_signup::columns::ID)
                    .eq()
                    .value(before_signup_id)
            })
        })
        .limit_offset(1, 0)
}
