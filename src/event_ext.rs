use crate::event::SEvent;
use kissa_topic::{adapter::AdaptersExt, kokoro::result::anyhow, prelude::*};

pub trait SEventExt {
    fn try_reply<T: Send + Sync>(&self, ctx: &Context<T>, message: String) -> Result<()>;
}
impl SEventExt for SEvent {
    fn try_reply<T: Send + Sync>(&self, ctx: &Context<T>, message: String) -> Result<()> {
        if let Some(parent) = ctx.clone().into_raw().parent.upgrade() {
            let parent = unsafe { parent.downcast_unchecked::<Kissa>(None, None) };
            parent.adapters.message(&self.from_adapter)?.create(
                self.content
                    .channel
                    .clone()
                    .ok_or(anyhow!("Resource Channel Not Exist"))?
                    .id,
                message,
            )?;
            Ok(())
        } else {
            Err(anyhow!("Can Not Upgrade Parent"))
        }
    }
}
