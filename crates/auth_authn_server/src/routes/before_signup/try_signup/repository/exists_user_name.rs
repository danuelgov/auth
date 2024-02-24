use auth_database::user_profile::{self, columns::UserProfileName, UserProfile};
use database_toolkit::{DatabaseConnection, QueryBuilder};
use sqlx::FromRow;

pub trait ExistsUserNameContract {
    async fn exists_user_name(
        &self,
        connection: DatabaseConnection,
        user_name: UserProfileName,
    ) -> Result<bool, ExistsUserNameError>;
}

#[derive(Debug)]
pub enum ExistsUserNameError {
    Database(sqlx::Error),
    Row(sqlx::Error),
}

impl ExistsUserNameContract for super::Repository {
    async fn exists_user_name(
        &self,
        mut connection: DatabaseConnection,
        user_name: UserProfileName,
    ) -> Result<bool, ExistsUserNameError> {
        let row = query(user_name)
            .build()
            .fetch_one(&mut *connection)
            .await
            .map_err(ExistsUserNameError::Database)?;

        #[derive(Debug, FromRow)]
        struct Row {
            count: i32,
        }

        let Row { count } = Row::from_row(&row).map_err(ExistsUserNameError::Row)?;

        Ok(count != 0)
    }
}

fn query<'q>(user_name: UserProfileName) -> QueryBuilder<'q> {
    let user_name = user_name.as_str().to_owned();

    QueryBuilder::new()
        .select(UserProfile, |builder| {
            builder.write("COUNT(*)").alias("count")
        })
        .where_(|builder| {
            builder.condition(|builder| {
                builder
                    .column(user_profile::columns::NAME)
                    .eq()
                    .value(user_name)
            })
        })
}
