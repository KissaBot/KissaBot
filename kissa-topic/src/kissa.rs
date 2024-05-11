use crate::adapter::*;
use crate::channel::Sender;
use kokoro_neo::result::Result;
use std::sync::Arc;

pub struct Kissa {
    sender: Sender,
    pub adapters: Adapters,
}
impl Kissa {
    pub fn new(sender: Sender) -> Self {
        Self {
            sender,
            adapters: Adapters::new(),
        }
    }
    pub fn publish<T: Send + Sync + 'static, N: Into<Arc<T>>>(&self, src: N) -> Result<()> {
        self.sender.send(src.into())?;
        Ok(())
    }
}
