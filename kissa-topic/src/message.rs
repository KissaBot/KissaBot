use serde::{Deserialize, Serialize};

/// 消息节点
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MessageNode {
    /// 文本消息
    String(String),
}
/// 消息
pub type Message = Vec<MessageNode>;
