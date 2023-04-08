use crate::{Config, routes};
use crate::utils::db::establish_connection;

pub fn launch(conf: &Config) {
    // Print welcome Message
    info!("Starting App in {}", conf.app.environment);

    // Connect to database
    let db_connection = establish_connection(&conf.database.connection_string).unwrap();

    // Start Rocket HTTP Server
    let web_server_result = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(routes::init(conf, db_connection));

    let _ = if let Ok(web_server) = web_server_result {
        web_server
    } else {
        panic!("Web server");
    };
}