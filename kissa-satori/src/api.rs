#![deny(missing_docs)]
use crate::resources::*;
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// 分页数据
#[derive(Serialize, Deserialize, Clone)]
pub struct Page<T: Clone> {
    /// 数据
    pub data: Vec<T>,
    /// 下一页的令牌
    pub next: Option<String>,
}
/// 频道API
pub trait ChannelAPI {
    /// ### 获取群组频道
    ///
    /// ID: `channel.get`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | channel_id | `String` | 频道 ID |
    ///
    /// 根据 ID 获取频道。返回一个 `Channel`。
    fn get(&self, channel_id: String) -> Result<Channel>;
    /// ### 获取群组频道列表
    ///
    /// ID: `channel.list`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | guild_id | `String` | 群组 ID |
    /// | next | `Option<String>` | 分页令牌 |
    ///
    /// 获取群组中的全部频道。返回一个 `Page<Channel>`。
    fn list(&self, guild_id: String, next: Option<String>) -> Result<Page<Channel>>;
    /// ### 创建群组频道
    ///
    /// ID: `channel.create`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | guild_id | `String` | 群组 ID |
    /// | data | `Channel` | 频道数据 |
    ///
    /// 创建群组频道。返回一个 `Channel`。
    fn create(&self, guild_id: String, data: Channel) -> Result<Channel>;
    /// ### 修改群组频道
    ///
    /// ID: `channel.update`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | channel_id | `String` | 频道 ID |
    /// | data | `Channel` | 频道数据 |
    ///
    /// 修改群组频道。
    fn update(&self, channel_id: String, data: Channel) -> Result<()>;
    /// ### 删除群组频道
    ///
    /// ID: `channel.delete`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | channel_id | `String` | 频道 ID |
    ///
    /// 删除群组频道。
    fn delete(&self, channel_id: String) -> Result<()>;
    /// ### 禁言群组频道
    ///
    /// ID: `channel.mute`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | channel_id | `String` | 频道 ID |
    /// | duration | `u64` | 禁言时长 (毫秒) |
    ///
    /// 禁言群组频道。如果传入的禁言时长为 `0` 则表示解除禁言。
    fn mute(&self, channel_id: String, duration: u64) -> Result<()>;
    /// ### 创建私聊频道
    ///
    /// ID: `user.channel.create`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | user_id | `String` | 用户 ID |
    /// | guild_id | `Option<String>` | 群组 ID |
    ///
    /// 创建一个私聊频道。返回一个 `Channel`。
    fn user_channel_create(&self, user_id: String, guild_id: Option<String>) -> Result<Channel>;
}
/// 群组API
pub trait GuildAPI {
    /// ### 获取群组
    ///
    /// ID: `guild.get`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | guild_id | `String` | 群组 ID |
    ///
    /// 根据 ID 获取。返回一个 `Guild` 对象。
    fn get(&self, guild_id: String) -> Result<Guild>;
    /// ### 获取群组列表
    ///
    /// ID: `guild.list`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | next | `Option<String>` | 分页令牌 |
    ///
    /// 获取当前用户加入的全部群组。返回一个 `Page<Guild>`。
    fn list(&self, next: Option<String>) -> Result<Page<Guild>>;
    /// ### 处理群组邀请
    ///
    /// ID: `guild.approve`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | message_id | `String` | 请求 ID |
    /// | approve | `bool` | 是否通过请求 |
    /// | comment | `String` | 备注信息 |
    ///
    /// 处理来自群组的邀请。
    fn approve(&self, message_id: String, approve: bool, comment: String) -> Result<()>;
}
/// 群组成员API
pub trait GuildMemberAPI {
    /// ### 获取群组成员
    ///
    /// ID: `guild.member.get`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | guild_id | `String` | 群组 ID |
    /// | user_id | `String` | 用户 ID |
    ///
    /// 获取群成员信息。返回一个 `GuildMember`。
    fn get(&self, guild_id: String, user_id: String) -> Result<GuildMember>;
    /// ### 获取群组成员列表
    ///
    /// ID: `guild.member.list`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | guild_id | `String` | 群组 ID |
    /// | next | `Option<String>` | 分页令牌 |
    ///
    /// 获取群成员列表。返回一个 `Page<GuildMember>`。
    fn list(&self, guild_id: String, next: Option<String>) -> Result<Page<GuildMember>>;
    /// ### 踢出群组成员
    ///
    /// ID: `guild.member.kick`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | guild_id | `String` | 群组 ID |
    /// | user_id | `String` | 用户 ID |
    /// | permanent | `Option<bool>` | 是否永久踢出 (无法再次加入群组) |
    ///
    /// 将某个用户踢出群组。
    fn kick(&self, guild_id: String, user_id: String, permanent: Option<bool>) -> Result<()>;
    /// ### 禁言群组成员
    ///
    /// ID: `guild.member.mute`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | guild_id | `String` | 群组 ID |
    /// | user_id | `String` | 用户 ID |
    /// | duration | `u64` | 禁言时长 (毫秒) |
    ///
    /// 将某个用户禁言。如果传入的禁言时长为 `0` 则表示解除禁言。
    fn mute(&self, guild_id: String, user_id: String, duration: u64) -> Result<()>;
    /// ### 通过群组成员申请
    ///
    /// ID: `guild.member.approve`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | message_id | `String` | 请求 ID |
    /// | approve | `bool` | 是否通过请求 |
    /// | comment | `Option<String>` | 备注信息 |
    ///
    /// 处理加群请求。
    fn approve(&self, message_id: String, approve: bool, comment: Option<String>) -> Result<()>;
}
/// 群组角色API
pub trait GuildRoleAPI {
    /// ### 设置群组成员角色
    ///
    /// ID: `guild.member.role.set`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | guild_id | `String` | 群组 ID |
    /// | user_id | `String` | 用户 ID |
    /// | role_id | `String` | 角色 ID |
    ///
    /// 设置群组内用户的角色。
    fn set(&self, guild_id: String, user_id: String, role_id: String) -> Result<()>;
    /// ### 取消群组成员角色
    ///
    /// ID: `guild.member.role.unset`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | guild_id | `String` | 群组 ID |
    /// | user_id | `String` | 用户 ID |
    /// | role_id | `String` | 角色 ID |
    ///
    /// 取消群组内用户的角色。
    fn unset(&self, guild_id: String, user_id: String, role_id: String) -> Result<()>;
    /// ### 获取群组角色列表
    ///
    /// ID: `guild.role.list`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | guild_id | `String` | 群组 ID |
    /// | next | `Option<String>` | 分页令牌 |
    ///
    /// 获取群组角色列表。返回一个 `Page<GuildRole>`。
    fn list(&self, guild_id: String, next: Option<String>) -> Result<Page<GuildRole>>;
    /// ### 创建群组角色
    ///
    /// ID: `guild.role.create`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | guild_id | `String` | 群组 ID |
    /// | role | `GuildRole` | 角色数据 |
    ///
    /// 创建群组角色。返回一个 `GuildRole` 对象。
    fn create(&self, guild_id: String, role: GuildRole) -> Result<GuildRole>;
    /// ### 修改群组角色
    ///
    /// ID: `guild.role.update`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | guild_id | `String` | 群组 ID |
    /// | role_id | `String` | 角色 ID |
    /// | role | `GuildRole` | 角色数据 |
    ///
    /// 修改群组角色。
    fn update(&self, guild_id: String, role_id: String, role: GuildRole) -> Result<()>;
    /// ### 删除群组角色
    ///
    /// ID: `guild.role.delete`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | guild_id | `String` | 群组 ID |
    /// | role_id | `String` | 角色 ID |
    ///
    /// 删除群组角色。
    fn delete(&self, guild_id: String, role_id: String) -> Result<()>;
}
/// 登录信息API
pub trait LoginAPI {
    /// ### 获取登录信息
    ///
    /// ID: `login.get`
    ///
    /// 获取登录信息。返回一个 `Login` 对象。
    fn get(&self) -> Result<Login>;
}
/// 消息API
pub trait MessageAPI {
    /// ### 发送消息
    ///
    /// ID: `message.create`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | channel_id | `String` | 频道 ID |
    /// | content | `String` | 消息内容 |
    ///
    /// 发送消息。返回一个 `Message` 对象构成的数组。
    fn create(&self, channel_id: String, content: String) -> Result<Vec<Message>>;
    /// ### 获取消息
    ///
    /// ID: `message.get`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | channel_id | `String` | 频道 ID |
    /// | message_id | `String` | 消息 ID |
    ///
    /// 获取特定消息。返回一个 `Message` 对象。必需资源：`channel`，`user`。
    fn get(&self, channel_id: String, message_id: String) -> Result<Message>;
    /// ### 撤回消息
    ///
    /// ID: `message.delete`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | channel_id | `String` | 频道 ID |
    /// | message_id | `String` | 消息 ID |
    ///
    /// 撤回特定消息。
    fn delete(&self, channel_id: String, message_id: String) -> Result<()>;
    /// ### 编辑消息
    ///
    /// ID: `message.update`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | channel_id | `String` | 频道 ID |
    /// | message_id | `String` | 消息 ID |
    /// | content | `String` | 消息内容 |
    ///
    /// 编辑特定消息。
    fn update(&self, channel_id: String, message_id: String, content: String) -> Result<()>;
    /// ### 获取消息列表
    ///
    /// ID: `message.list`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | channel_id | `String` | 频道 ID |
    /// | next | `Option<String>` | 分页令牌 |
    ///
    /// 获取频道消息列表。返回一个 `Page<Message>`。必需资源：`channel`，`user`。
    fn list(&self, channel_id: String, next: Option<String>) -> Result<Page<Message>>;
}
/// 表态API
pub trait ReactionAPI {
    /// ### 添加表态
    ///
    /// ID: `reaction.create`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | channel_id | `String` | 频道 ID |
    /// | message_id | `String` | 消息 ID |
    /// | emoji | `String` | 表态名称 |
    ///
    /// 向特定消息添加表态。
    fn create(&self, channel_id: String, message_id: String, emoji: String) -> Result<()>;
    /// ### 删除表态
    ///
    /// ID: `reaction.delete`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | channel_id | `String` | 频道 ID |
    /// | message_id | `String` | 消息 ID |
    /// | emoji | `String` | 表态名称 |
    /// | user_id | `Option<String>` | 用户 ID |
    ///
    /// 从特定消息删除某个用户添加的特定表态。如果没有传入用户 ID 则表示删除自己的表态。
    fn delete(
        &self,
        channel_id: String,
        message_id: String,
        emoji: String,
        user_id: Option<String>,
    ) -> Result<()>;
    /// ### 清除表态
    ///
    /// ID: `reaction.clear`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | channel_id | `String` | 频道 ID |
    /// | message_id | `String` | 消息 ID |
    /// | emoji | `Option<String>` | 表态名称 |
    ///
    /// 从特定消息清除某个特定表态。如果没有传入表态名称则表示清除所有表态。
    fn clear(&self, channel_id: String, message_id: String, emoji: Option<String>) -> Result<()>;
    /// ### 获取表态列表
    ///
    /// ID: `reaction.list`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | channel_id | `String` | 频道 ID |
    /// | message_id | `String` | 消息 ID |
    /// | emoji | `String` | 表态名称 |
    /// | next | `Option<String>` | 分页令牌 |
    ///
    /// 获取添加特定消息的特定表态的用户列表。返回一个 `Page<User>`。
    fn list(
        &self,
        channel_id: String,
        message_id: String,
        emoji: String,
        next: Option<String>,
    ) -> Result<Page<User>>;
}
/// 用户API
pub trait UserAPI {
    /// ### 获取用户信息
    ///
    /// ID: `user.get`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | user_id | `String` | 用户 ID |
    ///
    /// 获取用户信息。返回一个 `User`。
    fn get(&self, user_id: String) -> Result<User>;
    /// ### 获取好友列表
    ///
    /// ID: `friend.list`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | next | `String` | 分页令牌 |
    ///
    /// 获取好友列表。返回一个 `Page<User>`。
    fn list(&self, next: Option<String>) -> Result<Page<User>>;
    /// ### 处理好友申请
    ///
    /// ID: `friend.approve`
    ///
    /// | 字段 | 类型 | 描述 |
    /// | --- | --- | --- |
    /// | message_id | `String` | 请求 ID |
    /// | approve | `bool` | 是否通过请求 |
    /// | comment | `Option<String>` | 备注信息 |
    ///
    /// 处理好友申请。
    fn approve(&self, message_id: String, approve: bool, comment: Option<String>) -> Result<()>;
}
