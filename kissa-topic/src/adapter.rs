use std::sync::Arc;

use crate::context::Context;
use dashmap::DashMap;
use kissa_satori::api::*;
use kokoro_neo::result::anyhow;
use kokoro_neo::result::Result;

/// 多适配器
pub type Adapters = DashMap<u64, Arc<dyn Adapter + Send + Sync + 'static>>;

/// 适配器需要实现的 trait
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

/// 对多适配的扩展方法
pub trait AdaptersExt {
    /// 获取一个适配器的频道API
    fn channel(&self, adapter_id: &u64) -> Result<Arc<dyn ChannelAPI>>;
    /// 获取一个适配器的群组API
    fn guild(&self, adapter_id: &u64) -> Result<Arc<dyn GuildAPI>>;
    /// 获取一个适配器的群组成员API
    fn guild_member(&self, adapter_id: &u64) -> Result<Arc<dyn GuildMemberAPI>>;
    /// 获取一个适配器的群组角色API
    fn guild_role(&self, adapter_id: &u64) -> Result<Arc<dyn GuildRoleAPI>>;
    /// 获取一个适配器的登录信息API
    fn login(&self, adapter_id: &u64) -> Result<Arc<dyn LoginAPI>>;
    /// 获取一个适配器的消息API
    fn message(&self, adapter_id: &u64) -> Result<Arc<dyn MessageAPI>>;
    /// 获取一个适配器的表态API
    fn reaction(&self, adapter_id: &u64) -> Result<Arc<dyn ReactionAPI>>;
    /// 获取一个适配器的用户API
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

/// 将适配器注册进父上下文
pub fn add_adapter<T: Send + Sync, A: Adapter + Send + Sync + 'static>(
    ctx: &Context<T>,
    adapter: A,
) -> u64 {
    let id = &adapter as *const _ as u64;
    ctx.global.adapters.insert(id, Arc::new(adapter));
    id
}
