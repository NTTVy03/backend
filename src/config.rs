use std::env;

pub struct Config {
    pub database_url: String,
    pub server_addr: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let server_addr = env::var("SERVER_ADDR").expect("SERVER_ADDR must be set");
    
        Config {
            database_url,
            server_addr
        }
    }
}
