use std::env;

pub struct Config {
    pub addr: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        let addr = env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

        Config { addr }
    }
}