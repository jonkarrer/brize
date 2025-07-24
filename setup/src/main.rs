use colored::*;
use config::Config;
use dialoguer::Input;
use std::collections::HashMap;
use std::default;
use std::io::{self, Write};
use std::process::Command;
use uuid::Uuid;

#[derive(Debug)]
struct SetupError(String);
impl std::fmt::Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.red())
    }
}
impl std::error::Error for SetupError {}

fn main() {
    println!("{}\n", "   Stripe Setup   ".on_white().black().bold());
    match check_stripe_cli() {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            return;
        }
    }
    let stripe_secret_key = match get_stripe_secret_key() {
        Ok(key) => {
            println!("{}", "✅ Stripe secret key is set".green());
            key
        }
        Err(e) => {
            println!(
                "{}",
                "Failed to read Stripe secret key, you can fill it in later in the .env file".red()
            );
            return;
        }
    };
    let stripe_webhook_secret = match get_stripe_webhook_secret() {
        Ok(key) => {
            println!("{}\n", "✅ Stripe webhook secret is set".green());
            key
        }
        Err(e) => {
            println!(
                "{}",
                "Failed to get Stripe webhook secret, you can fill it in later in the .env file"
                    .red()
            );
            return;
        }
    };

    println!("{}\n", "  Database Setup  ".on_white().black().bold());
    let postgres_url = match get_db_url() {
        Ok(url) => {
            println!("{}", "✅ Database URL is set\n".green());
            url
        }
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    println!("{}\n", "  Application Setup  ".on_white().black().bold());
    let ui_port = match get_ui_port() {
        Ok(port) => {
            println!("{}", "✅ UI port is set".green());
            port
        }
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let auth_secret = generate_auth_secret();

    let env_file = format!(
        r#"POSTGRES_URL={}
STRIPE_SECRET_KEY={}
STRIPE_WEBHOOK_SECRET={}
BASE_URL=http://localhost:{}
AUTH_SECRET={}
"#,
        postgres_url, stripe_secret_key, stripe_webhook_secret, ui_port, auth_secret
    );
    let _ = std::fs::write(".env", env_file);
    println!("{}\n", "✅ Environment variables are set in .env".green());

    println!(
        "{}",
        "🎉 Setup Is Complete! You are ready to migrate and seed your database."
    );
}

fn check_stripe_cli() -> Result<(), SetupError> {
    // Check stripe cli install
    Command::new("stripe")
        .arg("--version")
        .output()
        .map_err(|_| {
            SetupError("Stripe CLI is not installed, please install and try again".to_string())
        })?;
    println!("{}", "✅ Stripe CLI is installed".green());

    // Check stripe login
    Command::new("stripe")
        .arg("config")
        .arg("--list")
        .output()
        .map_err(|_| {
            SetupError("Not logged into Stripe CLI, please log in and try again".to_string())
        })?;
    println!("{}", "✅ Authenticated with Stripe CLI".green());
    Ok(())
}

fn get_db_url() -> Result<String, SetupError> {
    let type_of_conn: String = Input::new()
        .with_prompt("Do you want to use a local Postgres instance with Docker (L) or a remote Postgres instance (R)? (L/R)")
        .interact_text()
        .map_err(|_| SetupError("Failed to read input".to_string()))?;
    clear_line();

    // Check if local argument
    if type_of_conn.to_lowercase() == "l" {
        let url = start_local_db()?;
        return Ok(url);
    }

    // Get remote url
    let remote_url: String = Input::new()
        .with_prompt("Please enter your remote Postgres instance url")
        .interact_text()
        .map_err(|_| SetupError("Failed to read input".to_string()))?;

    // Check if valid remote url
    if !remote_url.contains("postgres://") {
        println!("Missing postgres://, please try again");
        return get_db_url();
    }
    return Ok(remote_url);
}

fn start_local_db() -> Result<String, SetupError> {
    let configs: HashMap<String, String> = Config::builder()
        .add_source(config::File::with_name("./setup/config.toml"))
        .build()
        .map_err(|_| SetupError("Failed to get configs from config.toml".to_string()))?
        .try_deserialize()
        .map_err(|_| SetupError("Failed to get configs from config.toml".to_string()))?;

    let postgres_user = configs
        .get("POSTGRES_USER")
        .expect("Failed to get POSTGRES_USER");
    let postgres_password = configs
        .get("POSTGRES_PASSWORD")
        .expect("Failed to get POSTGRES_PASSWORD");
    let postgres_db = configs
        .get("POSTGRES_DB")
        .expect("Failed to get POSTGRES_DB");
    let postgres_port = configs
        .get("POSTGRES_PORT")
        .expect("Failed to get POSTGRES_PORT");
    let default_schema = configs
        .get("DEFAULT_SCHEMA")
        .expect("Failed to get DEFAULT_SCHEMA");

    // Check docker install
    Command::new("docker")
        .arg("--version")
        .output()
        .map_err(|_| {
            SetupError("Docker is not installed, please install and try again".to_string())
        })?;
    println!("{}", "✅ Docker is installed".green());

    // Check docker compose install
    Command::new("docker")
        .arg("compose")
        .arg("version")
        .output()
        .map_err(|_| {
            SetupError("Docker-compose is not installed, please install and try again".to_string())
        })?;
    println!("{}", "✅ Docker compose is installed".green());

    // Write docker-compose.yml
    let yaml_file = format!(
        r#"version: '3'
services:
  postgres:
    image: postgres:17.5-alpine3.22
    container_name: brize_postgres
    restart: unless-stopped
    environment:
      POSTGRES_USER: {} 
      POSTGRES_PASSWORD: {} 
      POSTGRES_DB: {}
      DEFAULT_SCHEMA: {}
    ports:
      - {}:5432
    volumes:
      - postgres_data:/var/lib/postgresql/data
volumes:
  postgres_data:
"#,
        postgres_user, postgres_password, postgres_db, default_schema, postgres_port,
    );
    std::fs::write("docker-compose.yml", yaml_file);
    println!("{}", "✅ Wrote docker-compose.yml".green());

    // Run docker-compose.yml
    Command::new("docker")
        .arg("compose")
        .arg("-f")
        .arg("docker-compose.yml")
        .arg("up")
        .arg("-d")
        .output()
        .map_err(|_| SetupError("Failed to start local Postgres instance".to_string()))?;
    println!("{}", "✅ Started local Postgres container".green());
    Ok(format!(
        "postgres://{}:{}@localhost:{}/postgres?currentSchema={}",
        postgres_user, postgres_password, postgres_port, default_schema
    ))
}

fn get_stripe_secret_key() -> Result<String, SetupError> {
    let input: String = Input::new()
        .with_prompt("Please enter your Stripe secret key")
        .interact_text()
        .map_err(|_| SetupError("Failed to read input".to_string()))?;
    clear_line();
    Ok(input)
}

fn get_stripe_webhook_secret() -> Result<String, SetupError> {
    let secret = Command::new("stripe")
        .arg("listen")
        .arg("--print-secret")
        .output()
        .map_err(|_| SetupError("Failed to get Stripe webhook secret".to_string()))?;
    String::from_utf8(secret.stdout)
        .map_err(|_| SetupError("Failed to get Stripe webhook secret".to_string()))
}

fn get_ui_port() -> Result<String, SetupError> {
    let input: String = Input::new()
        .with_prompt("Please enter the port you want the UI to run on, e.g. 3000")
        .interact_text()
        .map_err(|_| SetupError("Failed to read input".to_string()))?;
    clear_line();
    Ok(input)
}

fn generate_auth_secret() -> String {
    let auth_secret = uuid::Uuid::new_v4().to_string();
    println!("{}", "✅ Generated auth secret".green());
    return auth_secret;
}

fn clear_line() {
    print!("\x1B[1A"); // Move cursor up one line
    print!("\r\x1B[2K"); // Clear the entire line
    std::io::stdout().flush().unwrap();
}
