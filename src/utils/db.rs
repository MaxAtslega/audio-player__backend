use diesel::prelude::*;
use diesel::pg::PgConnection;
use log::{error, debug};

pub fn establish_connection(database_url: &str) -> Result<PgConnection, ConnectionError> {
    match PgConnection::establish(database_url) {
        Ok(connection) => {
            debug!("Connected to database");
            Ok(connection)
        },
        Err(error) => {
            error!("Could not connect to database");
            Err(error)
        }
    }
}