use rocket::get;
use rocket::serde::{json::Json, Serialize};
#[allow(unused_imports)]
use rocket_codegen::routes;
#[allow(unused_imports)]
use rocket_okapi::settings::UrlObject;
#[allow(unused_imports)]
use rocket_okapi::{openapi, openapi_get_routes, JsonSchema};

#[derive(Serialize, JsonSchema)]
pub struct InfoResponse {
    health: String,
    version: String,
    app_short: String,
    app_name: String,
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NAME: &'static str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

#[openapi]
#[get("/info")]
pub fn get_info() -> Json<InfoResponse> {

    Json(InfoResponse {
        version: VERSION.to_string(),
        health: "healthy".to_string(),
        app_name: DESCRIPTION.to_string(),
        app_short: NAME.to_string(),
    })
}
