#![deny(missing_docs)]
use serde::{Deserialize, Serialize};
use serde_json::Value;

// ===Channel==
/// 频道
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Channel {
    /// 频道 ID
    pub id: String,
    /// 频道类型
    #[serde(rename = "type")]
    pub ty: ChannelType,
    /// 频道名称
    pub name: Option<String>,
    /// 父频道 ID
    pub parent_id: Option<String>,
}
/// 频道类型
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(into = "isize", from = "isize")]
pub enum ChannelType {
    /// 文本频道
    Text = 0,
    /// 私聊频道
    Direct = 1,
    /// 分类频道
    Category = 2,
    /// 语音频道
    Voice = 3,
    /// 未知类型
    Unknown = -1,
}
impl From<isize> for ChannelType {
    fn from(value: isize) -> Self {
        match value {
            0 => ChannelType::Text,
            1 => ChannelType::Direct,
            2 => ChannelType::Category,
            3 => ChannelType::Voice,
            _ => ChannelType::Unknown,
        }
    }
}
impl From<ChannelType> for isize {
    fn from(value: ChannelType) -> Self {
        value as isize
    }
}

// ===Guild===
/// 群组
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Guild {
    /// 群组ID
    pub id: String,
    /// 群组名称
    pub name: Option<String>,
    /// 群组头像
    pub avatar: Option<String>,
}

// ===GuildMember===
/// 群组成员
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildMember {
    /// 用户对象
    pub user: Option<User>,
    /// 用户在群组中的名称
    pub nick: Option<String>,
    /// 用户在群组中的头像
    pub avatar: Option<String>,
    /// 加入时间
    pub joined_at: Option<u64>,
}

// ===GuildRole===
/// 群组角色
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildRole {
    /// 角色ID
    pub id: String,
    /// 角色名称
    pub name: Option<String>,
}

// ===Interaction===
/// 交互
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Interaction {
    /// 指令名称
    pub name: String,
    /// 参数
    pub arguments: Vec<Value>,
    /// 选项
    pub options: Value,
}

// ===Login===
/// 登录信息
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Login {
    /// 用户对象
    pub user: Option<User>,
    /// 平台账号
    pub self_id: Option<String>,
    /// 平台名称
    pub platform: Option<String>,
    /// 登录状态
    pub status: Status,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(into = "isize", from = "isize")]
/// 登录状态
pub enum Status {
    /// 离线
    Offline = 0,
    /// 在线
    Online = 1,
    /// 连接中
    Connect = 2,
    /// 断开连接
    Disconnect = 3,
    /// 重新连接
    Reconnect = 4,
    /// 未知
    Unknown = -1,
}
impl From<isize> for Status {
    fn from(value: isize) -> Self {
        match value {
            0 => Status::Offline,
            1 => Status::Online,
            2 => Status::Connect,
            3 => Status::Disconnect,
            4 => Status::Reconnect,
            _ => Status::Unknown,
        }
    }
}
impl From<Status> for isize {
    fn from(value: Status) -> Self {
        value as isize
    }
}

// ===Message===
/// 消息
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    /// 消息ID
    pub id: String,
    /// 消息内容
    pub content: String,
    /// 消息对象
    pub channel: Option<Channel>,
    /// 群组对象
    pub guild: Option<Guild>,
    /// 成员对象
    pub member: Option<GuildMember>,
    /// 用户对象
    pub user: Option<User>,
    /// 消息发送的时间戳
    pub created_at: Option<u64>,
    /// 消息修改的时间戳
    pub updated_at: Option<u64>,
}

// ===User===
/// 用户
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    /// 用户ID
    pub id: String,
    /// 用户名称
    pub name: Option<String>,
    /// 用户昵称
    pub nick: Option<String>,
    /// 用户头像
    pub avatar: Option<String>,
    /// 是否为机器人
    pub is_bot: Option<bool>,
}
