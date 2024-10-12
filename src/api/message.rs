use crate::entity;
use crate::entity::{MarkdownMessage, MessageSend};
use crate::utils::http_utils;
use rocket::serde::json::{serde_json, Json};

/// 发送消息
///
/// 传入json数据
/// {"subject": "主题", "content": "内容"}
/// 返回消息内容
#[post("/message/send/<key>", data = "<message>")]
pub async fn message_send(key: String, message: Json<MessageSend>) -> String {
    println!("{:?}", message);
    let markdown_message = MarkdownMessage::from_message_send(&message);
    http_utils::post(
        format!("{}{}", entity::WECOM_BOT_URL, key).as_str(),
        serde_json::to_string(&markdown_message).unwrap().as_str(),
    )
    .await
    .unwrap()
}
