#[macro_use]
extern crate rocket;
mod consul;
mod entity;
mod utils;

use crate::entity::MarkdownMessage;
use rocket::log::private::{log, Level};
use rocket::serde::json::{serde_json, Json};
use rocket::Request;
use utils::http_utils;

#[get("/index")]
fn index() -> &'static str {
    "Hello, World!"
}

/// 发送消息
///
/// 传入json数据
/// {"subject": "主题", "content": "内容"}
/// 返回消息内容
#[post("/message/send/<key>", data = "<message>")]
async fn message_send(key: String, message: Json<entity::MessageSend>) -> String {
    println!("{:?}", message);
    let markdown_message = MarkdownMessage::from_message_send(&message);
    http_utils::post(
        format!("{}{}", entity::WECOM_BOT_URL, key).as_str(),
        serde_json::to_string(&markdown_message).unwrap().as_str(),
    )
    .await
    .unwrap()
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[get("/health")]
fn health() -> &'static str {
    "OK"
}

#[launch]
async fn rocket() -> _ {
    let rocket = rocket::build();
    let config: consul::ConsulConfig = rocket.figment().extract().unwrap();
    println!("{:?}", config);
    // 在应用启动时注册服务
    if let Err(err) = consul::register_service(&config).await {
        log!(Level::Error, "Failed to register service: {}", err);
    }
    rocket
        .register("/", catchers![not_found])
        .mount("/", routes![index, health, message_send])
}
