use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

use kokoro_neo::result::Result;
use rayon::iter::Either;
use serde::{Deserialize, Serialize};

use crate::{adapter::Bot, message::Message};

/// 基本事件类型
#[derive(Clone)]
pub struct KEvent<C: Clone> {
    /// 机器人
    pub bot: Arc<Mutex<dyn Bot>>,
    /// 适配器 ID
    pub adapter_id: u64,
    /// 事件本体
    pub inner: C,
}
impl<T: Clone> Deref for KEvent<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
/// 频道
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Channel(pub Either<User, Guild>);
impl From<Either<User, Guild>> for Channel {
    fn from(value: Either<User, Guild>) -> Self {
        Channel(value)
    }
}
impl Deref for Channel {
    type Target = Either<User, Guild>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
/// 基本用户类型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    /// 用户 ID
    pub id: String,
    /// 用户名
    pub name: String,
    /// 用户昵称
    pub nick: Option<String>,
    /// 用户头像 url
    pub avatar_url: Option<String>,
    /// 是否为机器人
    pub is_bot: bool,
}

/// 基本群聊类型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Guild {}

/// 消息被创建。接收到消息会触发的事件。
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageCreated {
    /// 来自于
    pub from: Channel,
    /// 消息
    pub message: Message,
    /// 消息 ID
    pub message_id: String,
    /// 消息被创建的时间
    pub time: u128,
}

/// 事件扩展方法
pub trait MessageCreatedExt {
    /// 回复此条消息
    fn reply(&self, message: Message) -> Result<()>;
}

impl MessageCreatedExt for KEvent<MessageCreated> {
    fn reply(&self, message: Message) -> Result<()> {
        self.bot
            .lock()
            .unwrap()
            .create_message(message, self.from.clone())?;
        Ok(())
    }
}
