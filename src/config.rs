use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub db: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();  // Load environment variables from .env file

        let server_host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let server_port = env::var("PORT").unwrap_or_else(|_| "8080".to_string())
            .parse::<i32>()
            .unwrap_or(8080);
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Config {
            server: ServerConfig {
                host: server_host,
                port: server_port,
            },
            db: db_url,
        }
    }
}
