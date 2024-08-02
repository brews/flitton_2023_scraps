use crate::config::Config;
use diesel::pg::PgConnection;
use diesel::prelude::*;

// TODO: Use a connection pool instead.
pub fn establish_connection() -> PgConnection {
    let database_url = Config::new().database_url;

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
