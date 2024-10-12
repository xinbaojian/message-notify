#[macro_use]
extern crate rocket;
mod consul;
mod entity;
mod utils;
mod api;

use crate::entity::MarkdownMessage;
use api::*;
use rocket::log::private::{log, Level};
use rocket::serde::json::{serde_json, Json};
use rocket::Request;
use utils::http_utils;

#[launch]
async fn rocket() -> _ {
    let rocket = rocket::build();
    let config: consul::ConsulConfig = rocket.figment().extract().unwrap();
    // 在应用启动时注册服务
    if let Err(err) = consul::register_service(&config).await {
        log!(Level::Error, "Failed to register service: {}", err);
    }
    rocket
        .register("/", catchers![catch::not_found])
        .mount("/", routes![health::health, message::message_send])
}
