use core::panic;
use std::env;

use diesel::{Connection, SqliteConnection};
use diesel_logger::LoggingConnection;
use dotenvy::dotenv;
use log::{trace};

pub fn establish_connection() -> LoggingConnection<SqliteConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    trace!("opening database connection to {:?}", database_url);
    if database_url == "" {
        panic!("DATABASE_URL is empty");
    }
    let con = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    return LoggingConnection::new(con);
}