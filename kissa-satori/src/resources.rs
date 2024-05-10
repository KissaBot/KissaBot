use serde::{Deserialize, Serialize};

// ===Channel==
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Channel {
    /// 频道 ID
    pub id: String,
    /// 频道类型
    #[serde(rename = "type")]
    pub ty: u8,
    /// 频道名称
    pub name: Option<String>,
    /// 父频道 ID
    pub parent_id: Option<String>,
}
impl Channel {
    /// 获取频道类型
    pub fn ty(&self) -> Result<ChannelType, String> {
        match self.ty {
            0 => Ok(ChannelType::Text),
            1 => Ok(ChannelType::Direct),
            2 => Ok(ChannelType::Category),
            3 => Ok(ChannelType::Voice),
            _ => Err("Unknown Type".to_string()),
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum ChannelType {
    /// 文本频道
    Text = 0,
    /// 私聊频道
    Direct = 1,
    /// 分类频道
    Category = 2,
    /// 语音频道
    Voice = 3,
}

// ===Guild===
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Guild {
    pub id: String,
    pub name: Option<String>,
    pub avatar: Option<String>,
}

// ===GuildMember===
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildMember {
    pub user: Option<User>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub joined_at: Option<u64>,
}

// ===GuildRole===
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildRole {
    pub id: String,
    pub name: Option<String>,
}

// ===Interaction===
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Interaction {
    pub name: String,
    pub arguments: Vec<String>,
    pub options: bool,
}

// ===Login===
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Login {
    pub user: Option<User>,
    pub self_id: Option<String>,
    pub platform: Option<String>,
    pub status: u8,
}
impl Login {
    pub fn status(&self) -> Result<Status, String> {
        match self.status {
            0 => Ok(Status::Offline),
            1 => Ok(Status::Online),
            2 => Ok(Status::Connect),
            3 => Ok(Status::Disconnect),
            4 => Ok(Status::Reconnect),
            _ => Err("Unknown Status".to_string()),
        }
    }
}
#[derive(Clone, Debug)]
pub enum Status {
    Offline = 0,
    Online = 1,
    Connect = 2,
    Disconnect = 3,
    Reconnect = 4,
}

// ===Message===
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub id: String,
    pub content: String,
    pub channel: Option<Channel>,
    pub guild: Option<Guild>,
    pub member: Option<GuildMember>,
    pub user: Option<User>,
    pub created_at: Option<u64>,
    pub updated_at: Option<u64>,
}

// ===Reaction===
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Reaction {
    pub channel_id: String,
    pub message_id: String,
    pub emoji: String,
}

// ===User===
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub name: Option<String>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub is_bot: Option<bool>,
}
