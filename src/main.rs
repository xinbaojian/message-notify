#[macro_use]
extern crate rocket;
mod api;
mod consul;
mod entity;
mod utils;

use api::*;
use rocket::log::private::{log, Level};

#[launch]
async fn rocket() -> _ {
    let rocket = rocket::build();
    let config: consul::ConsulConfig = rocket.figment().extract().unwrap();
    // 在应用启动时注册服务
    if let Err(err) = consul::register_service(&config).await {
        log!(Level::Error, "Failed to register service: {}", err);
    }
    rocket
        .register("/", catchers![catch::not_found, catch::internal_error])
        .mount("/", routes![health::health, message::message_send])
}
