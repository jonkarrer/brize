mod migrate;
mod schema;
mod seed;
mod setup;

use colored::*;

#[derive(Debug)]
struct SetupError(String);
impl std::fmt::Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.red())
    }
}
impl std::error::Error for SetupError {}

fn main() {
    let postgres_url = match setup::run() {
        Ok(url) => url,
        Err(e) => {
            println!("{}", e.to_string().red());
            std::process::exit(1);
        }
    };
    println!("🎉 Setup Is Complete! Creating Users and Teams schema...\n");

    match migrate::run(&postgres_url) {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e.to_string().red());
            std::process::exit(1);
        }
    };

    println!("🎉 Users and Teams created! Seeding database...");

    match seed::run(&postgres_url) {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e.to_string().red());
            std::process::exit(1);
        }
    }
}
