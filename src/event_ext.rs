use crate::event::SEvent;
use kissa_topic::{adapter::AdaptersExt, kokoro::result::anyhow, prelude::*};

pub trait SEventExt {
    fn reply<T: Send + Sync>(&self, ctx: &Context<T>, message: String) -> Result<()>;
}
impl SEventExt for SEvent {
    fn reply<T: Send + Sync>(&self, ctx: &Context<T>, message: String) -> Result<()> {
        ctx.global.adapters.message(&self.from_adapter)?.create(
            self.content
                .channel
                .clone()
                .ok_or(anyhow!("频道资源不存在"))?
                .id,
            message,
        )?;
        Ok(())
    }
}
