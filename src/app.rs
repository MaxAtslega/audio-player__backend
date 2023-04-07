use crate::{Config, routes};

pub fn launch(conf: &Config) {
    //Print welcome Message
    info!("Starting App in {}", conf.app.environment);

    //Start Rocket HTTP Server
    let web_server_result = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(routes::init(conf));

    let _ = if let Ok(web_server) = web_server_result {
        web_server
    } else {
        panic!("Web server");
    };
}