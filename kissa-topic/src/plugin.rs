use std::sync::{Arc, OnceLock};

use kokoro_neo::{
    any::KAny,
    avail::Avails,
    context::{ChildHandle, Children},
    plugin::dynamic::{Library, Symbol},
    result::anyhow,
};
use mlua::prelude::*;

use crate::{
    context::RawContext,
    kissa::Kissa,
    prelude::{Context, Result},
};
use serde::Deserialize;

/// kissabot 插件特征
pub trait Plugin: KAny + Sized {
    /// 配置类型
    type Config: Deserialize<'static>;
    /// 加载插件
    fn load(ctx: Context<Self>) -> Result<()>;
    /// 创建插件
    fn create(config: Self::Config) -> Result<Self>;
}
/// 可动态插件的
pub trait DyPluggable {
    /// 动态插件插入
    fn dyn_plug(
        &self,
        lib: Library,
        lua: &Lua,
        value: LuaValue,
    ) -> Result<ChildHandle<Arc<dyn KAny>>>;
}
/// 加载插件
pub type LoadFn = extern "Rust" fn(ctx: Arc<RawContext>, global: Kissa) -> Result<()>;
/// 创建插件
pub type CreateFn = extern "Rust" fn(lua: &Lua, value: LuaValue) -> Result<Arc<dyn KAny>, String>;
/// 用于初始化动态链接库的 logger 的函数
pub type SetupLoggerFn = extern "Rust" fn(logger: &'static dyn log::Log, level: log::LevelFilter);
/// 初始化动态链接库的 logger
pub fn setup_logger(lib: &Library) -> Result<()> {
    let setup: Symbol<SetupLoggerFn> = unsafe { lib.get(b"__setup_logger__") }?;
    setup(log::logger(), log::max_level());
    Ok(())
}
impl<T: KAny + 'static + ?Sized> DyPluggable for Context<T> {
    fn dyn_plug(
        &self,
        lib: Library,
        lua: &Lua,
        value: LuaValue,
    ) -> Result<ChildHandle<Arc<dyn KAny>>> {
        setup_logger(&lib)?;
        let create_fn: Symbol<CreateFn> = unsafe { lib.get(b"__create__")? };
        let load_fn: Symbol<LoadFn> = unsafe { lib.get(b"__load__")? };
        let scope = create_fn(lua, value).map_err(|err| anyhow!("{}", err))?;
        let raw: Arc<RawContext> = RawContext {
            scope,
            children: Children::new(),
            parent: Arc::downgrade(self.raw_ref()),
            avails: Avails::new(),
            _effects: Box::new([OnceLock::new()]),
        }
        .into();
        let id = self.children_raw().add(Arc::clone(&raw));
        load_fn(raw.clone(), self.global.clone())?;
        let _ = raw._effects[0].set(Box::new(lib));
        Ok(ChildHandle::from(id))
    }
}
