use dotenv::dotenv;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub gzip_response: bool,
    pub server: String,
}

// Add the Config struct into a CONFIG lazy_static for single loading
lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

/// Use envy to inject dotenv and env vars into the Config struct
fn get_config() -> Config {
    dotenv().ok();

    match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Configuration Error: {:#?}", error),
    }
}
