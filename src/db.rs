use sea_orm::{Database as SeaDatabase, DatabaseConnection};
use std::sync::Arc;

#[derive(Clone)]
pub struct Database {
    pub conn: Arc<DatabaseConnection>,
}

impl Database {
    pub async fn connect(database_url: &str) -> Result<Self, sea_orm::DbErr> {
        let conn = SeaDatabase::connect(database_url).await?;
        Ok(Self {
            conn: Arc::new(conn),
        })
    }
}
