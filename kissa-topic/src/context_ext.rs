use crate::avail::*;
use crate::context::Context;
use kokoro_neo::any::*;
use std::sync::Arc;

/// 扩展上下文的方法
pub trait ContextExt<T: Send + Sync> {
    /// 新增一个观察者，当有事件就会被触发
    fn observe<
        Param: Params<T, Arc<dyn KAny>> + 'static,
        Func: FnMut<Param, Output = ()> + 'static,
        A: Into<Availed<T, Param, Func>>,
    >(
        &self,
        f: A,
    ) -> AvailHandle<T, Param, Func>;
}

impl<T: Send + Sync> ContextExt<T> for Context<T> {
    fn observe<
        Param: Params<T, Arc<dyn KAny>> + 'static,
        Func: FnMut<Param, Output = ()> + 'static,
        A: Into<Availed<T, Param, Func>>,
    >(
        &self,
        f: A,
    ) -> AvailHandle<T, Param, Func> {
        self.avails().add(f)
    }
}
