#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

mod app;
mod config;
mod routes;
mod api;

use config::Config;

fn main() {
    let conf = Config::from_any().unwrap();

    //Launch App
    app::launch(&conf);

}