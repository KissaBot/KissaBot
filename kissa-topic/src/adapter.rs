use std::ops::Deref;
use std::sync::Arc;

use dashmap::DashMap;
use kissa_satori::api::*;
use kissa_satori::resources::*;
use kokoro_neo::result::anyhow;
use kokoro_neo::result::Result;

pub type Adapters = DashMap<u64, Arc<dyn Adapter + Send + Sync + 'static>>;

pub trait Adapter:
    ChannelAPI
    + GuildAPI
    + GuildRoleAPI
    + GuildMemberAPI
    + LoginAPI
    + MessageAPI
    + ReactionAPI
    + UserAPI
{
}
pub trait AdaptersExt {
    fn channel(&self, adapter_id: &u64) -> Result<Arc<dyn ChannelAPI>>;
    fn guild(&self, adapter_id: &u64) -> Result<Arc<dyn GuildAPI>>;
    fn guild_member(&self, adapter_id: &u64) -> Result<Arc<dyn GuildMemberAPI>>;
    fn guild_role(&self, adapter_id: &u64) -> Result<Arc<dyn GuildRoleAPI>>;
    fn login(&self, adapter_id: &u64) -> Result<Arc<dyn LoginAPI>>;
    fn message(&self, adapter_id: &u64) -> Result<Arc<dyn MessageAPI>>;
    fn reaction(&self, adapter_id: &u64) -> Result<Arc<dyn ReactionAPI>>;
    fn user(&self, adapter_id: &u64) -> Result<Arc<dyn UserAPI>>;
}
impl AdaptersExt for Adapters {
    fn channel(&self, adapter_id: &u64) -> Result<Arc<dyn ChannelAPI>> {
        self.get(adapter_id)
            .ok_or(anyhow!("Adapter Not Exist"))
            .map(|v| Arc::clone(&*v) as Arc<dyn ChannelAPI>)
    }
    fn guild(&self, adapter_id: &u64) -> Result<Arc<dyn GuildAPI>> {
        self.get(adapter_id)
            .ok_or(anyhow!("Adapter Not Exist"))
            .map(|v| Arc::clone(&*v) as Arc<dyn GuildAPI>)
    }
    fn guild_member(&self, adapter_id: &u64) -> Result<Arc<dyn GuildMemberAPI>> {
        self.get(adapter_id)
            .ok_or(anyhow!("Adapter Not Exist"))
            .map(|v| Arc::clone(&*v) as Arc<dyn GuildMemberAPI>)
    }
    fn guild_role(&self, adapter_id: &u64) -> Result<Arc<dyn GuildRoleAPI>> {
        self.get(adapter_id)
            .ok_or(anyhow!("Adapter Not Exist"))
            .map(|v| Arc::clone(&*v) as Arc<dyn GuildRoleAPI>)
    }
    fn login(&self, adapter_id: &u64) -> Result<Arc<dyn LoginAPI>> {
        self.get(adapter_id)
            .ok_or(anyhow!("Adapter Not Exist"))
            .map(|v| Arc::clone(&*v) as Arc<dyn LoginAPI>)
    }
    fn message(&self, adapter_id: &u64) -> Result<Arc<dyn MessageAPI>> {
        self.get(adapter_id)
            .ok_or(anyhow!("Adapter Not Exist"))
            .map(|v| Arc::clone(&*v) as Arc<dyn MessageAPI>)
    }
    fn reaction(&self, adapter_id: &u64) -> Result<Arc<dyn ReactionAPI>> {
        self.get(adapter_id)
            .ok_or(anyhow!("Adapter Not Exist"))
            .map(|v| Arc::clone(&*v) as Arc<dyn ReactionAPI>)
    }
    fn user(&self, adapter_id: &u64) -> Result<Arc<dyn UserAPI>> {
        self.get(adapter_id)
            .ok_or(anyhow!("Adapter Not Exist"))
            .map(|v| Arc::clone(&*v) as Arc<dyn UserAPI>)
    }
}
