use super::MessageSend;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MarkdownMessage {
    msgtype: String,
    markdown: MarkdownMessageContent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarkdownMessageContent {
    content: String,
}

/// MarkdownMessage
/// {
///     "msgtype": "markdown",
///     "markdown": {
///         "content": "#### 杭州天气 @156xxxx8827\n" +
///                     "> 9:00 am @ wxbot\n" +
///                     "> :sunny:\n" +
///                     " > :umbrella:\n" +
///                     " > [-4, 18] :thermometer:\n" +
///                     " > [](/home/qywx/jenkins/awesome.jpg)\n"
///     }
/// }
impl Default for MarkdownMessage {
    fn default() -> Self {
        Self {
            msgtype: "markdown".to_string(),
            markdown: MarkdownMessageContent {
                content: "".to_string(),
            },
        }
    }
}

impl MarkdownMessage {
    pub fn from_message_send(message_send: &MessageSend) -> Self {
        Self {
            msgtype: "markdown".to_string(),
            markdown: MarkdownMessageContent {
                content: format!("# {}\n{}", message_send.subject, message_send.content),
            },
        }
    }
}
