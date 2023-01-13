use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Database {
    url: String,
    password: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Sparkpost {
    key: String,
    token: String,
    url: String,
    version: u8,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Twitter {
    consumer_token: String,
    consumer_secret: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Braintree {
    merchant_id: String,
    public_key: String,
    private_key: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    debug: bool,
    database: Database,
    sparkpost: Sparkpost,
    twitter: Twitter,
    braintree: Braintree,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let profile = env::var("PROFILE").unwrap_or_else(|_| "local".into());

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            // .add_source(File::with_name("src/configs/config_local"))
            .add_source(File::with_name("src/configs/config_local"))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(File::with_name(&format!("src/configs/config_{}", profile)).required(false), )
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(config::File::with_name("src/configs/config_local").required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("app"))
            // .add_source(Environment::with_prefix())
            // You may also programmatically change settings
            // .set_override("database.url", "postgres://")?
            .set_override_option("database.password", env::var("DB_PASSWORD").ok())?
            // .set_override("database.password", std::env::var("DB_PASSWORD").unwrap())?
            .build()?;

        // Now that we're done, let's access our configuration
        println!("debug: {:?}", s.get_bool("debug"));
        println!("database: {:?}", s.get::<String>("database.url"));

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}