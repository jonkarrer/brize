use sea_orm::{Database, DatabaseConnection};

use crate::tools::settings;

pub async fn db() -> DatabaseConnection {
    let url = settings().postgres_url;
    Database::connect(url)
        .await
        .expect("Failed to connect to database")
}
