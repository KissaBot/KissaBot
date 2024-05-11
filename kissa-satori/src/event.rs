#![deny(missing_docs)]
/// 严格类型
#[deprecated]
pub mod strict {
    use crate::resources::*;
    use serde::{Deserialize, Serialize};
    /// satori 的基本事件
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct SatoriEvent {
        /// satori 的事件ID，注意与 KAny 的 ID 区分。
        id: u64,
        /// 事件类型
        #[serde(flatten)]
        ty: SatoriEventType,
        /// 接收者的平台名称
        platfrom: String,
        /// 接收者的平台账号
        self_id: String,
        /// 事件的时间戳
        timestamp: u64,
    }
    /// 事件类型
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(tag = "type")]
    pub enum SatoriEventType {
        /// 加入群组
        #[serde(rename = "guild-added")]
        GuildAdded {
            /// 交互指令
            argv: Option<Argv>,
            /// 交互按钮
            button: Option<Button>,
            /// 事件所属频道
            channel: Option<Channel>,
            /// 事件所属群组
            guild: Guild,
            /// 事件的登录信息
            login: Option<Login>,
            /// 事件的目标成员
            member: Option<GuildMember>,
            /// 事件的消息
            message: Option<Message>,
            /// 事件的操作者
            operator: Option<User>,
            /// 事件的目标角色
            role: Option<GuildRole>,
            /// 事件的目标用户
            user: Option<User>,
        },
        /// 群组被修改
        #[serde(rename = "guild-updated")]
        GuildUpdated {
            /// 交互指令
            argv: Option<Argv>,
            /// 交互按钮
            button: Option<Button>,
            /// 事件所属频道
            channel: Option<Channel>,
            /// 事件所属群组
            guild: Guild,
            /// 事件的登录信息
            login: Option<Login>,
            /// 事件的目标成员
            member: Option<GuildMember>,
            /// 事件的消息
            message: Option<Message>,
            /// 事件的操作者
            operator: Option<User>,
            /// 事件的目标角色
            role: Option<GuildRole>,
            /// 事件的目标用户
            user: Option<User>,
        },
        /// 退出群组
        #[serde(rename = "guild-removed")]
        GuildRemoved {
            /// 交互指令
            argv: Option<Argv>,
            /// 交互按钮
            button: Option<Button>,
            /// 事件所属频道
            channel: Option<Channel>,
            /// 事件所属群组
            guild: Guild,
            /// 事件的登录信息
            login: Option<Login>,
            /// 事件的目标成员
            member: Option<GuildMember>,
            /// 事件的消息
            message: Option<Message>,
            /// 事件的操作者
            operator: Option<User>,
            /// 事件的目标角色
            role: Option<GuildRole>,
            /// 事件的目标用户
            user: Option<User>,
        },
        /// 接收到新的入群邀请
        #[serde(rename = "guild-request")]
        GuildRequest {
            /// 交互指令
            argv: Option<Argv>,
            /// 交互按钮
            button: Option<Button>,
            /// 事件所属频道
            channel: Option<Channel>,
            /// 事件所属群组
            guild: Guild,
            /// 事件的登录信息
            login: Option<Login>,
            /// 事件的目标成员
            member: Option<GuildMember>,
            /// 事件的消息
            message: Option<Message>,
            /// 事件的操作者
            operator: Option<User>,
            /// 事件的目标角色
            role: Option<GuildRole>,
            /// 事件的目标用户
            user: Option<User>,
        },
        /// 群组成员增加
        #[serde(rename = "guild-member-added")]
        GuildMemberAdded {
            /// 交互指令
            argv: Option<Argv>,
            /// 交互按钮
            button: Option<Button>,
            /// 事件所属频道
            channel: Option<Channel>,
            /// 事件所属群组
            guild: Guild,
            /// 事件的登录信息
            login: Option<Login>,
            /// 事件的目标成员
            member: GuildMember,
            /// 事件的消息
            message: Option<Message>,
            /// 事件的操作者
            operator: User,
            /// 事件的目标角色
            role: Option<GuildRole>,
            /// 事件的目标用户
            user: Option<User>,
        },
        /// 群组成员信息更新
        #[serde(rename = "guild-member-updated")]
        GuildMemberUpdated {
            /// 交互指令
            argv: Option<Argv>,
            /// 交互按钮
            button: Option<Button>,
            /// 事件所属频道
            channel: Option<Channel>,
            /// 事件所属群组
            guild: Guild,
            /// 事件的登录信息
            login: Option<Login>,
            /// 事件的目标成员
            member: GuildMember,
            /// 事件的消息
            message: Option<Message>,
            /// 事件的操作者
            operator: User,
            /// 事件的目标角色
            role: Option<GuildRole>,
            /// 事件的目标用户
            user: Option<User>,
        },
        /// 群组成员移除
        #[serde(rename = "guild-member-removed")]
        GuildMemberRemoved {
            /// 交互指令
            argv: Option<Argv>,
            /// 交互按钮
            button: Option<Button>,
            /// 事件所属频道
            channel: Option<Channel>,
            /// 事件所属群组
            guild: Guild,
            /// 事件的登录信息
            login: Option<Login>,
            /// 事件的目标成员
            member: GuildMember,
            /// 事件的消息
            message: Option<Message>,
            /// 事件的操作者
            operator: User,
            /// 事件的目标角色
            role: Option<GuildRole>,
            /// 事件的目标用户
            user: Option<User>,
        },
        /// 接收到新的加群请求
        #[serde(rename = "guild-member-request")]
        GuildMemberRequest {
            /// 交互指令
            argv: Option<Argv>,
            /// 交互按钮
            button: Option<Button>,
            /// 事件所属频道
            channel: Option<Channel>,
            /// 事件所属群组
            guild: Guild,
            /// 事件的登录信息
            login: Option<Login>,
            /// 事件的目标成员
            member: GuildMember,
            /// 事件的消息
            message: Option<Message>,
            /// 事件的操作者
            operator: User,
            /// 事件的目标角色
            role: Option<GuildRole>,
            /// 事件的目标用户
            user: Option<User>,
        },
        /// 群组角色被创建
        #[serde(rename = "guild-role-created")]
        GuildRoleCreated {
            /// 交互指令
            argv: Option<Argv>,
            /// 交互按钮
            button: Option<Button>,
            /// 事件所属频道
            channel: Option<Channel>,
            /// 事件所属群组
            guild: Guild,
            /// 事件的登录信息
            login: Option<Login>,
            /// 事件的目标成员
            member: Option<GuildMember>,
            /// 事件的消息
            message: Option<Message>,
            /// 事件的操作者
            operator: Option<User>,
            /// 事件的目标角色
            role: GuildRole,
            /// 事件的目标用户
            user: Option<User>,
        },
        /// 群组角色被修改
        #[serde(rename = "guild-role-updated")]
        GuildRoleUpdated {
            /// 交互指令
            argv: Option<Argv>,
            /// 交互按钮
            button: Option<Button>,
            /// 事件所属频道
            channel: Option<Channel>,
            /// 事件所属群组
            guild: Guild,
            /// 事件的登录信息
            login: Option<Login>,
            /// 事件的目标成员
            member: Option<GuildMember>,
            /// 事件的消息
            message: Option<Message>,
            /// 事件的操作者
            operator: Option<User>,
            /// 事件的目标角色
            role: GuildRole,
            /// 事件的目标用户
            user: Option<User>,
        },
        /// 群组角色被删除
        #[serde(rename = "guild-role-deleted")]
        GuildRoleDeleted {
            /// 交互指令
            argv: Option<Argv>,
            /// 交互按钮
            button: Option<Button>,
            /// 事件所属频道
            channel: Option<Channel>,
            /// 事件所属群组
            guild: Guild,
            /// 事件的登录信息
            login: Option<Login>,
            /// 事件的目标成员
            member: Option<GuildMember>,
            /// 事件的消息
            message: Option<Message>,
            /// 事件的操作者
            operator: Option<User>,
            /// 事件的目标角色
            role: GuildRole,
            /// 事件的目标用户
            user: Option<User>,
        },
        /// 类型为 action 的按钮被点击
        #[serde(rename = "interaction/button")]
        InteractionButton {
            /// 交互指令
            argv: Option<Argv>,
            /// 交互按钮
            button: Button,
            /// 事件所属频道
            channel: Option<Channel>,
            /// 事件所属群组
            guild: Option<Guild>,
            /// 事件的登录信息
            login: Option<Login>,
            /// 事件的目标成员
            member: Option<GuildMember>,
            /// 事件的消息
            message: Option<Message>,
            /// 事件的操作者
            operator: Option<User>,
            /// 事件的目标角色
            role: Option<GuildRole>,
            /// 事件的目标用户
            user: Option<User>,
        },
        /// 调用斜线指令
        #[serde(rename = "interaction/command")]
        InteractionCommand {
            /// 交互指令
            argv: Option<Argv>,
            /// 交互按钮
            button: Option<Button>,
            /// 事件所属频道
            channel: Option<Channel>,
            /// 事件所属群组
            guild: Option<Guild>,
            /// 事件的登录信息
            login: Option<Login>,
            /// 事件的目标成员
            member: Option<GuildMember>,
            /// 事件的消息
            message: Option<Message>,
            /// 事件的操作者
            operator: Option<User>,
            /// 事件的目标角色
            role: Option<GuildRole>,
            /// 事件的目标用户
            user: Option<User>,
        },
        /// 登录被创建
        #[serde(rename = "login-added")]
        LoginAdded {
            /// 交互指令
            argv: Option<Argv>,
            /// 交互按钮
            button: Option<Button>,
            /// 事件所属频道
            channel: Option<Channel>,
            /// 事件所属群组
            guild: Option<Guild>,
            /// 事件的登录信息
            login: Login,
            /// 事件的目标成员
            member: Option<GuildMember>,
            /// 事件的消息
            message: Option<Message>,
            /// 事件的操作者
            operator: Option<User>,
            /// 事件的目标角色
            role: Option<GuildRole>,
            /// 事件的目标用户
            user: Option<User>,
        },
        /// 登录被删除
        #[serde(rename = "login-removed")]
        LoginRemoved {
            /// 交互指令
            argv: Option<Argv>,
            /// 交互按钮
            button: Option<Button>,
            /// 事件所属频道
            channel: Option<Channel>,
            /// 事件所属群组
            guild: Option<Guild>,
            /// 事件的登录信息
            login: Login,
            /// 事件的目标成员
            member: Option<GuildMember>,
            /// 事件的消息
            message: Option<Message>,
            /// 事件的操作者
            operator: Option<User>,
            /// 事件的目标角色
            role: Option<GuildRole>,
            /// 事件的目标用户
            user: Option<User>,
        },
        /// 登录信息更新
        #[serde(rename = "login-updated")]
        LoginUpdated {
            /// 交互指令
            argv: Option<Argv>,
            /// 交互按钮
            button: Option<Button>,
            /// 事件所属频道
            channel: Option<Channel>,
            /// 事件所属群组
            guild: Option<Guild>,
            /// 事件的登录信息
            login: Login,
            /// 事件的目标成员
            member: Option<GuildMember>,
            /// 事件的消息
            message: Option<Message>,
            /// 事件的操作者
            operator: Option<User>,
            /// 事件的目标角色
            role: Option<GuildRole>,
            /// 事件的目标用户
            user: Option<User>,
        },
        /// 消息被创建
        #[serde(rename = "message-created")]
        MessageCreated {
            /// 交互指令
            argv: Option<Argv>,
            /// 交互按钮
            button: Option<Button>,
            /// 事件所属频道
            channel: Channel,
            /// 事件所属群组
            guild: Option<Guild>,
            /// 事件的登录信息
            login: Option<Login>,
            /// 事件的目标成员
            member: Option<GuildMember>,
            /// 事件的消息
            message: Message,
            /// 事件的操作者
            operator: User,
            /// 事件的目标角色
            role: Option<GuildRole>,
            /// 事件的目标用户
            user: Option<User>,
        },
        /// 消息被编辑
        #[serde(rename = "message-updated")]
        MessageUpdated {
            /// 交互指令
            argv: Option<Argv>,
            /// 交互按钮
            button: Option<Button>,
            /// 事件所属频道
            channel: Channel,
            /// 事件所属群组
            guild: Option<Guild>,
            /// 事件的登录信息
            login: Option<Login>,
            /// 事件的目标成员
            member: Option<GuildMember>,
            /// 事件的消息
            message: Message,
            /// 事件的操作者
            operator: User,
            /// 事件的目标角色
            role: Option<GuildRole>,
            /// 事件的目标用户
            user: Option<User>,
        },
        /// 消息被编辑
        #[serde(rename = "message-deleted")]
        MessageDeleted {
            /// 交互指令
            argv: Option<Argv>,
            /// 交互按钮
            button: Option<Button>,
            /// 事件所属频道
            channel: Channel,
            /// 事件所属群组
            guild: Option<Guild>,
            /// 事件的登录信息
            login: Option<Login>,
            /// 事件的目标成员
            member: Option<GuildMember>,
            /// 事件的消息
            message: Message,
            /// 事件的操作者
            operator: User,
            /// 事件的目标角色
            role: Option<GuildRole>,
            /// 事件的目标用户
            user: Option<User>,
        },
        /// 表态被添加
        #[serde(rename = "reaction-added")]
        ReactionAdded {
            /// 交互指令
            argv: Option<Argv>,
            /// 交互按钮
            button: Option<Button>,
            /// 事件所属频道
            channel: Option<Channel>,
            /// 事件所属群组
            guild: Option<Guild>,
            /// 事件的登录信息
            login: Option<Login>,
            /// 事件的目标成员
            member: Option<GuildMember>,
            /// 事件的消息
            message: Option<Message>,
            /// 事件的操作者
            operator: Option<User>,
            /// 事件的目标角色
            role: Option<GuildRole>,
            /// 事件的目标用户
            user: Option<User>,
        },
        /// 表态被移除
        #[serde(rename = "reaction-removed")]
        ReactionRemoved {
            /// 交互指令
            argv: Option<Argv>,
            /// 交互按钮
            button: Option<Button>,
            /// 事件所属频道
            channel: Option<Channel>,
            /// 事件所属群组
            guild: Option<Guild>,
            /// 事件的登录信息
            login: Option<Login>,
            /// 事件的目标成员
            member: Option<GuildMember>,
            /// 事件的消息
            message: Option<Message>,
            /// 事件的操作者
            operator: Option<User>,
            /// 事件的目标角色
            role: Option<GuildRole>,
            /// 事件的目标用户
            user: Option<User>,
        },
        /// 接收到新的好友申请
        #[serde(rename = "friend-request")]
        FriendRequest {
            /// 交互指令
            argv: Option<Argv>,
            /// 交互按钮
            button: Option<Button>,
            /// 事件所属频道
            channel: Option<Channel>,
            /// 事件所属群组
            guild: Option<Guild>,
            /// 事件的登录信息
            login: Option<Login>,
            /// 事件的目标成员
            member: Option<GuildMember>,
            /// 事件的消息
            message: Option<Message>,
            /// 事件的操作者
            operator: Option<User>,
            /// 事件的目标角色
            role: Option<GuildRole>,
            /// 事件的目标用户
            user: User,
        },
    }
}

use std::time::{SystemTime, UNIX_EPOCH};

use crate::resources::*;
use serde::{Deserialize, Serialize};

/// satori 的基本事件
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SatoriEvent {
    /// satori 的事件ID，注意与 KAny 的 ID 区分。
    pub id: u64,
    /// 事件类型
    #[serde(rename = "type")]
    pub ty: SatoriEventType,
    /// 接收者的平台名称
    pub platfrom: String,
    /// 接收者的平台账号
    pub self_id: String,
    /// 事件的时间戳
    pub timestamp: u128,
    /// 交互指令
    pub argv: Option<Argv>,
    /// 交互按钮
    pub button: Option<Button>,
    /// 事件所属频道
    pub channel: Option<Channel>,
    /// 事件所属群组
    pub guild: Option<Guild>,
    /// 事件的登录信息
    pub login: Option<Login>,
    /// 事件的目标成员
    pub member: Option<GuildMember>,
    /// 事件的消息
    pub message: Option<Message>,
    /// 事件的操作者
    pub operator: Option<User>,
    /// 事件的目标角色
    pub role: Option<GuildRole>,
    /// 事件的目标用户
    pub user: Option<User>,
}
impl Default for SatoriEvent {
    fn default() -> Self {
        // 获取当前时间
        let now = SystemTime::now();
        // 计算当前时间与 UNIX 纪元（1970-01-01 00:00:00 UTC）之间的时间间隔
        let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let time = since_epoch.as_millis();
        SatoriEvent {
            id: 0,
            ty: SatoriEventType::Unknown,
            platfrom: "".to_string(),
            self_id: "".to_string(),
            timestamp: time,
            argv: None,
            button: None,
            channel: None,
            guild: None,
            login: None,
            member: None,
            message: None,
            operator: None,
            role: None,
            user: None,
        }
    }
}
/// 事件类型
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum SatoriEventType {
    /// 加入群组
    #[serde(rename = "guild-added")]
    GuildAdded,
    /// 群组被修改
    #[serde(rename = "guild-updated")]
    GuildUpdated,
    /// 退出群组
    #[serde(rename = "guild-removed")]
    GuildRemoved,
    /// 接收到新的入群邀请
    #[serde(rename = "guild-request")]
    GuildRequest,
    /// 群组成员增加
    #[serde(rename = "guild-member-added")]
    GuildMemberAdded,
    /// 群组成员信息更新
    #[serde(rename = "guild-member-updated")]
    GuildMemberUpdated,
    /// 群组成员移除
    #[serde(rename = "guild-member-removed")]
    GuildMemberRemoved,
    /// 接收到新的加群请求
    #[serde(rename = "guild-member-request")]
    GuildMemberRequest,
    /// 群组角色被创建
    #[serde(rename = "guild-role-created")]
    GuildRoleCreated,
    /// 群组角色被修改
    #[serde(rename = "guild-role-updated")]
    GuildRoleUpdated,
    /// 群组角色被删除
    #[serde(rename = "guild-role-deleted")]
    GuildRoleDeleted,
    /// 类型为 action 的按钮被点击
    #[serde(rename = "interaction/button")]
    InteractionButton,
    /// 调用斜线指令
    #[serde(rename = "interaction/command")]
    InteractionCommand,
    /// 登录被创建
    #[serde(rename = "login-added")]
    LoginAdded,
    /// 登录被删除
    #[serde(rename = "login-removed")]
    LoginRemoved,
    /// 登录信息更新
    #[serde(rename = "login-updated")]
    LoginUpdated,
    /// 消息被创建
    #[serde(rename = "message-created")]
    MessageCreated,
    /// 消息被编辑
    #[serde(rename = "message-updated")]
    MessageUpdated,
    /// 消息被编辑
    #[serde(rename = "message-deleted")]
    MessageDeleted,
    /// 表态被添加
    #[serde(rename = "reaction-added")]
    ReactionAdded,
    /// 表态被移除
    #[serde(rename = "reaction-removed")]
    ReactionRemoved,
    /// 接收到新的好友申请
    #[serde(rename = "friend-request")]
    FriendRequest,
    /// 未知类型
    #[serde(rename = "unknown")]
    Unknown,
}
