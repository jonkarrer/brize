use std::collections::HashMap;

use config::{Config, Environment, File};

pub struct Settings {
    pub postgres_url: String,
    pub stripe_secret_key: String,
    pub stripe_webhook_secret: String,
    pub base_url: String,
    pub auth_secret: String,
    // Add to this and the return below
}

pub fn settings() -> Settings {
    let s = Config::builder()
        .add_source(File::with_name(".env").required(false)) // pulls from .env file
        .add_source(Environment::default()) // pick up envs that have been set in the shell
        .add_source(Environment::with_prefix("APP")) // change this to whatever prefix you use
        .build()
        .expect("Failed to get configs from .env");

    let configs: HashMap<String, String> = s
        .try_deserialize()
        .expect("Failed to deserialize configs from .env");

    Settings {
        postgres_url: configs
            .get("POSTGRES_URL")
            .expect("Failed to get POSTGRES_URL")
            .to_string(),
        stripe_secret_key: configs
            .get("STRIPE_SECRET_KEY")
            .expect("Failed to get STRIPE_SECRET_KEY")
            .to_string(),
        stripe_webhook_secret: configs
            .get("STRIPE_WEBHOOK_SECRET")
            .expect("Failed to get STRIPE_WEBHOOK_SECRET")
            .to_string(),
        base_url: configs
            .get("BASE_URL")
            .expect("Failed to get BASE_URL")
            .to_string(),
        auth_secret: configs
            .get("AUTH_SECRET")
            .expect("Failed to get AUTH_SECRET")
            .to_string(),
        // Add to this
    }
}
