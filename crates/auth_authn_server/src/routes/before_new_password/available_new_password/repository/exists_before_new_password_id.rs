use auth_database::before_new_password::{
    self, columns::BeforeNewPasswordIdentity, BeforeNewPassword,
};
use database_toolkit::{DatabaseConnection, QueryBuilder};
use sqlx::FromRow;

pub trait ExistsBeforeNewPasswordIdContract {
    async fn exists_before_new_password_id(
        &self,
        connection: DatabaseConnection,
        before_new_password_id: BeforeNewPasswordIdentity,
    ) -> Result<Existence, ExistsBeforeNewPasswordIdError>;
}

pub enum Existence {
    Yes,
    No,
}

#[derive(Debug)]
pub enum ExistsBeforeNewPasswordIdError {
    Database(sqlx::Error),
    Row(sqlx::Error),
    Unknown,
}

impl ExistsBeforeNewPasswordIdContract for super::Repository {
    async fn exists_before_new_password_id(
        &self,
        mut connection: DatabaseConnection,
        before_new_password_id: BeforeNewPasswordIdentity,
    ) -> Result<Existence, ExistsBeforeNewPasswordIdError> {
        let row = query(before_new_password_id)
            .build()
            .fetch_one(&mut *connection)
            .await
            .map_err(ExistsBeforeNewPasswordIdError::Database)?;

        #[derive(FromRow)]
        struct Row {
            count: i8,
        }

        let Row { count } = Row::from_row(&row).map_err(ExistsBeforeNewPasswordIdError::Row)?;

        match count {
            0 => Ok(Existence::No),
            1 => Ok(Existence::Yes),
            _ => Err(ExistsBeforeNewPasswordIdError::Unknown),
        }
    }
}

fn query<'q>(before_new_password_id: BeforeNewPasswordIdentity) -> QueryBuilder<'q> {
    QueryBuilder::new()
        .select(BeforeNewPassword, |builder| {
            builder.write("COUNT(*)").alias("count")
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
