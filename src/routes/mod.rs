pub mod catcher;

use rocket::{Error, Ignite, Rocket};
use rocket::catchers;

use crate::config::Config;
use crate::api;

use rocket_okapi::openapi_get_routes;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

pub async fn init(conf: &Config) -> Result<Rocket<Ignite>, Error> {
    let conf = &conf.webserver;

    let figment = rocket::Config::figment()
        .merge(("port", &conf.port))
        .merge(("address", &conf.address))
        .merge(("ident", &conf.ident))
        .merge(("log_level", "Off"));

    let rocket = rocket::custom(figment)
        .mount("/api/", openapi_get_routes![api::info::get_info])
        .register("/", catchers![
            catcher::bad_request,
            catcher::unauthorized,
            catcher::forbidden,
            catcher::not_found,
            catcher::not_implemented,
            catcher::internal_error,
            catcher::unprocessable_entity,
        ])
        .mount(
            "/api/swagger/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "/api/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .ignite()
        .await?;

    let result= rocket.launch().await;
    println!("The server shutdown: {:?}", result);

    result
}