use dotenv::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub secret_key: String,
    pub expire_minutes: i64,
    pub redis_url: String,
}

impl Config {
    #[cfg(not(test))]
    pub fn new() -> Self {
        // doing this in new() doesn't feel right because construction could throw an error.
        dotenv().ok();

        return Config {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            secret_key: env::var("SECRET_KEY").expect("SECRET_KEY must be set"),
            expire_minutes: env::var("EXPIRE_MINUTES")
                .expect("EXPIRE_MINUTES must be set")
                .parse::<i64>()
                .expect("Cannot parse EXPIRE_MINUTES string as i64"),
            redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set"),
        };
    }

    #[cfg(test)]
    pub fn new() -> Self {
        return Config {
            database_url: String::from("postgres://username:password@localhost:5433/to_do"),
            secret_key: String::from("secret"),
            expire_minutes: 120,
            redis_url: String::from("redis://127.0.0.1/"),
        };
    }
}
