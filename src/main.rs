#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

mod app;
mod config;
mod log;
mod routes;
mod api;

use config::Config;

fn main() {
    let conf = Config::from_any().unwrap();

    // Setup simplelog
    log::setup(&conf.log);

    // Launch App
    app::launch(&conf);

}