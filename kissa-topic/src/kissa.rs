use crate::adapter::*;
use crate::channel::Sender;
use kokoro_neo::result::Result;
use std::sync::Arc;

/// kissabot 的主结构体，上下文中的 scope
#[derive(Clone)]
pub struct Kissa {
    sender: Sender,
    /// 适配器们
    pub adapters: Adapters,
}
impl Kissa {
    /// 创建一个新的 kissabot 实例，请自行使用 `Context` 包装。
    pub fn new(sender: Sender) -> Self {
        Self {
            sender,
            adapters: Adapters::new(),
        }
    }
    /// 发布一个事件。
    pub fn publish<T: Send + Sync + 'static, N: Into<Arc<T>>>(&self, src: N) -> Result<()> {
        self.sender.send(src.into())?;
        Ok(())
    }
}
