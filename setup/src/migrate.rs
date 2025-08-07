use colored::*;
use std::process::Command;

use crate::SetupError;

pub fn run(database_url: &str) -> Result<(), SetupError> {
    println!("{}\n", "   Running Migrations   ".on_white().black().bold());

    let output = Command::new("diesel")
        .args(&["migration", "run", "--migration-dir", "setup/migrations"])
        .env("DATABASE_URL", database_url)
        .output()
        .map_err(|e| SetupError(format!("Failed to execute diesel command: {}", e)))?;

    if output.status.success() {
        println!("{}", "✅ Migrations completed \n".green());
        Ok(())
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        Err(SetupError(format!(
            "Diesel migration failed: {}",
            error_msg
        )))
    }
}
