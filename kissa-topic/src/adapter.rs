use kissa_satori::resources::*;
use kokoro_neo::result::Result;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Page<T> {
    pub data: Vec<T>,
    pub next: Option<String>,
}

pub struct AdapterCache {
    pub raw: Box<dyn Adapter + Send + Sync + 'static>,
}
pub trait Adapter: ChannelAPI + GuildAPI + GuildRoleAPI {}
pub trait ChannelAPI {
    fn get(&self, channel_id: String) -> Result<Channel>;
    fn list(&self, guild_id: String, next: Option<String>) -> Result<Page<Channel>>;
    fn create(&self, guild_id: String, data: Channel) -> Result<Channel>;
    fn update(&self, channel_id: String, data: Channel) -> Result<()>;
    fn delete(&self, channel_id: String) -> Result<()>;
    fn mute(&self, channel_id: String, duration: u64) -> Result<()>;
    fn user_channel_create(&self, user_id: String, guild_id: Option<String>) -> Result<Channel>;
}
pub trait GuildAPI {
    fn get(&self, guild_id: String) -> Result<Guild>;
    fn list(&self, next: Option<String>) -> Result<Page<Guild>>;
    fn approve(&self, message_id: String, approve: bool, comment: String) -> Result<()>;
}
pub trait GuildMemberAPI {
    fn get(&self, guild_id: String, user_id: String) -> Result<GuildMember>;
    fn list(&self, guild_id: String, next: Option<String>) -> Result<Page<GuildMember>>;
    fn kick(&self, guild_id: String, user_id: String, permanent: bool) -> Result<()>;
    fn mute(&self, guild_id: String, user_id: String, duration: u64) -> Result<()>;
    fn approve(&self, message_id: String, approve: bool, comment: Option<String>) -> Result<()>;
}
pub trait GuildRoleAPI {
    fn set(&self, guild_id: String, user_id: String, role_id: String) -> Result<()>;
    fn unset(&self, guild_id: String, user_id: String, role_id: String) -> Result<()>;
    fn list(&self, guild_id: String, next: Option<String>) -> Result<Page<GuildRole>>;
    fn create(&self, guild_id: String, role: GuildRole) -> Result<GuildRole>;
    fn update(&self, guild_id: String, role_id: String, role: GuildRole) -> Result<()>;
    fn delete(&self, guild_id: String, role_id: String) -> Result<()>;
}
