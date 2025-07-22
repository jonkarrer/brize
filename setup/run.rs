use config::Config;
use dialoguer::Input;
use std::collections::HashMap;
use std::process::Command;
use uuid::Uuid;

fn main() {
    check_stripe_cli();

    let postgres_url = get_db_url();
    let stripe_secret_key = get_stripe_secret_key();
    let stripe_webhook_secret = get_stripe_webhook_secret();
    let ui_port = get_ui_port();
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

    println!("Done! Your .env file has been created!");
}

fn check_stripe_cli() {
    // Check stripe cli install
    println!("1.Checking stripe cli install");
    if Command::new("stripe").arg("--version").output().is_err() {
        println!("Stripe cli is not installed, please install stripe cli and try again");
        return;
    }
    println!("Stripe cli is installed");

    // Check stripe login
    println!("2.Checking stripe login");
    if Command::new("stripe")
        .arg("config")
        .arg("--list")
        .output()
        .is_err()
    {
        println!("Not logged into Stripe, please login and try again");
        return;
    }
    println!("Logged into Stripe");
}

fn get_db_url() -> String {
    println!("3.Select database connection type");
    let type_of_conn: String = Input::new()
        .with_prompt("Do you want to use a local Postgres instance with Docker (L) or a remote Postgres instance (R)? (L/R)")
        .interact_text()
        .expect("Failed to read input");

    // Check if local
    if type_of_conn.to_lowercase() == "l" {
        println!("4.Starting local Postgres instance");
        return start_local_db().expect("Failed to start local Postgres instance");
    }

    // Get remote url
    let remote_url: String = Input::new()
        .with_prompt("Please enter your remote Postgres instance url")
        .interact_text()
        .expect("Failed to read input");

    // Check if valid remote url
    if !remote_url.contains("postgres://") {
        println!("Missing postgres://, please try again");
        return get_db_url();
    }
    println!("4.Using remote Postgres instance");
    return remote_url;
}

fn start_local_db() -> Result<String, std::io::Error> {
    // Get Configs
    let configs: HashMap<String, String> = Config::builder()
        .add_source(config::File::with_name("./setup/config.toml"))
        .build()
        .unwrap()
        .try_deserialize()
        .expect("Failed to deserialize configs");

    let postgres_user = configs.get("POSTGRES_USER").unwrap();
    let postgres_password = configs.get("POSTGRES_PASSWORD").unwrap();
    let postgres_db = configs.get("POSTGRES_DB").unwrap();
    let postgres_port = configs.get("POSTGRES_PORT").unwrap();

    // Check docker install
    println!("Checking docker install");
    if Command::new("docker").arg("--version").output().is_err() {
        println!("Docker is not installed, please install docker and try again");
        println!("See https://docs.docker.com/get-docker/");
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Docker is not installed",
        ));
    };
    println!("Docker is installed");

    // Check docker-compose install
    println!("Checking docker-compose install");
    if Command::new("docker")
        .arg("compose")
        .arg("version")
        .output()
        .is_err()
    {
        println!("Docker-compose is not installed, please install docker-compose and try again");
        println!("See https://docs.docker.com/compose/install/");
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Docker-compose is not installed",
        ));
    };
    println!("Docker-compose is installed");

    // Write docker-compose.yml for postgres
    println!("Writing docker-compose.yml for postgres");

    let yaml_file = format!(
        r#"version: '3'
services:
  postgres:
    image: postgres:17.5-alpine3.22
    restart: unless-stopped
    environment:
      POSTGRES_USER: {} 
      POSTGRES_PASSWORD: {} 
      POSTGRES_DB: {}
    ports:
      - {}:5432
    volumes:
      - postgres_data:/var/lib/postgresql/data
volumes:
  postgres_data:
"#,
        postgres_user, postgres_password, postgres_db, postgres_port,
    );
    let _ = std::fs::write("docker-compose.yml", yaml_file);
    println!("Wrote docker-compose.yml for postgres");

    // Run docker-compose.yml
    println!("Running docker-compose.yml for postgres");
    if Command::new("docker")
        .arg("compose")
        .arg("-f")
        .arg("docker-compose.yml")
        .arg("up")
        .arg("-d")
        .output()
        .is_err()
    {
        println!("Failed to run docker-compose.yml for postgres");
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to run docker-compose.yml for postgres",
        ));
    };
    println!("Ran docker-compose.yml for postgres");

    Ok(format!(
        "postgres://{}:{}@localhost:{}/postgres",
        postgres_user, postgres_password, postgres_port
    ))
}

fn get_stripe_secret_key() -> String {
    println!("5.Getting Stripe secret key");
    let stripe_secret_key: String = Input::new()
        .with_prompt("Please enter your Stripe secret key")
        .interact_text()
        .expect("Failed to read input");
    return stripe_secret_key;
}

fn get_stripe_webhook_secret() -> String {
    println!("6.Getting Stripe webhook secret");
    let secret = Command::new("stripe")
        .arg("listen")
        .arg("--print-secret")
        .output();
    if secret.is_err() {
        println!("Failed to get Stripe webhook secret, you can fill it in later in the .env file");
        return String::new();
    }
    let secret = String::from_utf8(secret.unwrap().stdout)
        .expect("Failed to get Stripe webhook secret, you can fill it in later in the .env file");
    println!("Got Stripe webhook secret");
    return secret;
}

fn get_ui_port() -> String {
    println!("7.Getting UI port");
    let ui_port: String = Input::new()
        .with_prompt("Please enter the port you want the UI to run on, e.g. 3000")
        .interact_text()
        .expect("Failed to read input");
    return ui_port;
}

fn generate_auth_secret() -> String {
    println!("8.Generating auth secret");
    let auth_secret = uuid::Uuid::new_v4().to_string();
    println!("Generated auth secret");
    return auth_secret;
}
