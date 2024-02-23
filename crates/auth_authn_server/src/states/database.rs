use database_toolkit::DatabaseConnectionPool;
use rocket::{Build, Rocket};

#[derive(Debug)]
pub enum DatabaseError {
    DatabaseUrl(std::env::VarError),
    ConnectionPool(sqlx::Error),
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DatabaseError {
    //
}

pub async fn manage(rocket: Rocket<Build>) -> Result<Rocket<Build>, DatabaseError> {
    let database_url = std::env::var("DATABASE_URL").map_err(DatabaseError::DatabaseUrl)?;
    let pool = DatabaseConnectionPool::builder()
        .build(&database_url)
        .await
        .map_err(DatabaseError::ConnectionPool)?;
    let rocket = rocket.manage(pool);

    Ok(rocket)
}
