#![feature(unboxed_closures, trait_upcasting, trait_alias)]
#![warn(missing_docs)]
#![doc = "该包是 kissabot 的基础包。"]

/// kokoro 是一个元框架，它是 kissabot 的基础
pub use kokoro_neo as kokoro;

pub use log;

/// 上下文概念来自于 kokoro
pub mod context {
    use crate::kissa::Kissa;
    use std::sync::Arc;
    /// 携带类型的上下文
    pub type Context<T> = kokoro_neo::context::Context<T, Arc<dyn kokoro_neo::any::KAny>, Kissa>;
    pub use kokoro_neo::context::{ChildHandle, Children, RawContext, RawContextExt};
}

/// 事件通道
pub mod channel {
    pub use flume::*;
    use kokoro_neo::any::KAny;
    use std::sync::Arc;
    /// kissabot 的发送者
    pub type Sender = flume::Sender<Arc<dyn KAny>>;
    /// kissabot 的接收者
    pub type Receiver = flume::Receiver<Arc<dyn KAny>>;
}

/// 可用性概念来自于 kokoro
pub mod avail {
    use kokoro_neo::any::KAny;
    use std::sync::Arc;

    /// 事件基本类型
    pub type Event = Arc<dyn KAny>;
    /// kissabot 的可用例
    pub type Availed<T, Param, Func> =
        kokoro_neo::avail::Availed<T, Param, Func, Arc<dyn KAny>, Kissa>;
    /// kissabot 的可用例句柄
    pub type AvailHandle<T, Param, Func> =
        kokoro_neo::avail::AvailHandle<T, Param, Func, Arc<dyn KAny>, Kissa>;
    /// 参数特征
    pub trait Params<T: KAny> = kokoro_neo::avail::Params<T, Arc<dyn KAny>, Kissa>;
    pub use kokoro_neo::avail::Avail;
    pub use kokoro_neo::avail::Avails;

    use crate::prelude::Kissa;
}

/// kissabot 的适配器
pub mod adapter;
/// 对 kokoro 上下文的扩展
pub mod context_ext;
/// kissabot 的主结构体
pub mod kissa;

/// 主要模块
pub mod prelude {
    pub use crate::adapter::*;
    pub use crate::avail::*;
    pub use crate::channel;
    pub use crate::context::*;
    pub use crate::context_ext::*;
    pub use crate::export_plugin;
    pub use crate::kissa::*;
    pub use kokoro_neo::any::*;
    pub use kokoro_neo::plugin::dynamic::*;
    pub use kokoro_neo::plugin::*;
    pub use kokoro_neo::result::Result;
    pub use log::{debug, error, info, trace, warn};
}

/// 订阅常规事件
#[macro_export]
macro_rules! subscribe {
    ($ctx:expr,$event:ty,$subscriber:expr) => {
        $crate::context_ext::ContextExt::observe(
            &$ctx,
            |ctx: $crate::context::Context<_>, event: $crate::avail::Event| {
                if let Some(se) = <dyn $crate::kokoro::any::KAny>::downcast_ref::<$event>(&event) {
                    $subscriber(ctx, se);
                }
            },
        );
    };
}

pub use kokoro_neo::export_plugin as export_plugin_;
/// 导出插件
#[macro_export]
macro_rules! export_plugin {
    ($plugin_type:ty,$plugin:expr) => {
        $crate::export_plugin_!($plugin_type, $plugin);
        #[no_mangle]
        pub extern "Rust" fn __setup_logger__(
            logger: &'static dyn $crate::log::Log,
            level: $crate::log::LevelFilter,
        ) -> $crate::kokoro::result::Result<()> {
            $crate::log::set_max_level(level);
            $crate::log::set_logger(logger)?;
            Ok(())
        }
    };
}

use crate::kokoro::result::Result;
/// 用于初始化动态链接库的 logger 的函数
pub type SetupLoggerFn =
    extern "Rust" fn(logger: &'static dyn log::Log, level: log::LevelFilter) -> Result<()>;

use kokoro::plugin::dynamic::{Library, Symbol};
/// 初始化动态链接库的 logger
pub fn setup_logger(lib: &Library) -> Result<()> {
    let setup: Symbol<SetupLoggerFn> = unsafe { lib.get(b"__setup_logger__") }?;
    setup(log::logger(), log::max_level())?;
    Ok(())
}
