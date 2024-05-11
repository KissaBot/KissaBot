use kissa_satori::api::*;

pub struct AdapterCache {
    pub raw: Box<dyn Adapter + Send + Sync + 'static>,
}
pub trait Adapter: ChannelAPI + GuildAPI + GuildRoleAPI {}
