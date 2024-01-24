use std::env::var;

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        let database_url = var("FIN_DATABASE_URL").expect("FIN_DATABASE_URL must be set");
        Self { database_url }
    }
}
