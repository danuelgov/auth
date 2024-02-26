use auth_database::{
    user::columns::UserPrimaryKey,
    user_profile::{self, UserProfile},
};
use database_toolkit::{DatabaseConnection, QueryBuilder};
use sqlx::FromRow;

pub trait FindUserProfileByPrimaryKeyContract {
    async fn find_user_profile_by_primary_key(
        &self,
        connection: DatabaseConnection,
        user_pk: UserPrimaryKey,
    ) -> Result<FindUserProfileByPrimaryKeyEntity, FindUserProfileByPrimaryKeyError>;
}

pub struct FindUserProfileByPrimaryKeyEntity {
    pub handle: String,
    pub name: String,
    pub bio: String,
    pub image: String,
}

#[derive(Debug)]
pub enum FindUserProfileByPrimaryKeyError {
    Database(sqlx::Error),
    Row(sqlx::Error),
}

impl FindUserProfileByPrimaryKeyContract for super::Repository {
    async fn find_user_profile_by_primary_key(
        &self,
        mut connection: DatabaseConnection,
        user_pk: UserPrimaryKey,
    ) -> Result<FindUserProfileByPrimaryKeyEntity, FindUserProfileByPrimaryKeyError> {
        let row = query(user_pk)
            .build()
            .fetch_one(&mut *connection)
            .await
            .map_err(FindUserProfileByPrimaryKeyError::Database)?;

        #[derive(FromRow)]
        struct Row {
            handle: String,
            name: String,
            bio: String,
            image: String,
        }

        let Row {
            handle,
            name,
            bio,
            image,
        } = Row::from_row(&row).map_err(FindUserProfileByPrimaryKeyError::Row)?;

        Ok(FindUserProfileByPrimaryKeyEntity {
            handle,
            name,
            bio,
            image,
        })
    }
}

fn query<'q>(user_pk: UserPrimaryKey) -> QueryBuilder<'q> {
    QueryBuilder::new()
        .select(UserProfile, |builder| {
            builder.columns(&[
                user_profile::columns::HANDLE,
                user_profile::columns::NAME,
                user_profile::columns::BIO,
                user_profile::columns::IMAGE,
            ])
        })
        .where_(|builder| {
            builder.condition(|builder| {
                let user_pk: Vec<u8> = user_pk.into();

                builder
                    .column(user_profile::columns::USER_PK)
                    .eq()
                    .value(user_pk)
            })
        })
        .limit_offset(1, 0)
}
