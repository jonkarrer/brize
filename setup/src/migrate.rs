use colored::*;
use sea_orm::{ConnectionTrait, Database};
use std::fs;

use crate::SetupError;

pub async fn run(database_url: &str) -> Result<(), SetupError> {
    println!("{}\n", "   Running Migrations   ".on_white().black().bold());
    let db = Database::connect(database_url)
        .await
        .map_err(|_| SetupError("Failed to connect to database".to_string()))?;
    println!("{}", "✅ Connected to database".green());
    let sql = fs::read_to_string("setup/migrations/0000_initialize.sql")
        .map_err(|_| SetupError("Failed to read migration files".to_string()))?;
    db.execute_unprepared(&sql)
        .await
        .map_err(|_| SetupError("Failed to run migrations".to_string()))?;
    println!("{}", "✅ Migrations completed \n".green());
    Ok(())
}
