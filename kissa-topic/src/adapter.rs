use std::sync::{Arc, Mutex};

use crate::{context::Context, event::Channel, message::Message, prelude::Event};
use dashmap::DashMap;
use kokoro_neo::result::Result;

/// 多适配器
pub type Adapters = DashMap<u64, Arc<dyn Adapter + Send + Sync + 'static>>;

/// 适配器需要实现的 trait
pub trait Adapter {
    /// 获取机器人
    fn bot(&self) -> Arc<Mutex<dyn Bot>>;
}

/// 机器人
pub trait Bot {
    /// 发送消息
    fn create_message(&mut self, message: Message, channel: Channel) -> Result<u32>;
    /// 撤回消息
    fn delete_message(&mut self, message_id: String) -> Result<()>;
    /// 获取消息
    fn get_message(&mut self, channel: Channel, message_id: String);
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
