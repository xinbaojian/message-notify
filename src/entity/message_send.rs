/// 定义消息发送实体
use serde::{Deserialize, Serialize};

/// 定义消息发送实体
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageSend {
    // 标题
    pub subject: String,
    // 内容
    pub content: String,
}
