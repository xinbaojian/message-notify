mod markdown_message;
mod message_send;

pub use message_send::MessageSend;

pub use markdown_message::MarkdownMessage;

pub const WECOM_BOT_URL: &str = "https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key=";
