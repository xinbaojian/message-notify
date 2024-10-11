#[macro_use]
extern crate rocket;
mod entity;
mod utils;

use crate::entity::MarkdownMessage;
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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![index, message_send])
}
