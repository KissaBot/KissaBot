use crate::adapter::AdapterCache;
use crate::channel::Sender;
use kokoro_neo::result::Result;
use std::sync::Arc;

pub struct Kissa {
    sender: Sender,
    pub adapters: Vec<AdapterCache>,
}
impl Kissa {
    pub fn new(sender: Sender) -> Self {
        Self {
            sender,
            adapters: Vec::new(),
        }
    }
    pub fn send<T: Send + Sync + 'static, N: Into<Arc<T>>>(&self, src: N) -> Result<()> {
        self.sender.send(src.into())?;
        Ok(())
    }
}
