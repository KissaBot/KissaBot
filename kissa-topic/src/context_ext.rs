use crate::avail::*;
use crate::channel::Sender;
use kokoro_neo::any::*;
use std::sync::Arc;

use kokoro_neo::result::Result;

use crate::context::Context;

pub trait ContextExt {
    fn send<T: Send + Sync + 'static, N: Into<Arc<T>>>(&self, src: N) -> Result<()>;
    fn observe<
        Param: Params<Sender, Arc<dyn KAny>> + 'static,
        Func: FnMut<Param, Output = ()> + 'static,
        A: Into<Availed<Param, Func>>,
    >(
        &self,
        f: A,
    ) -> AvailHandle<Param, Func>;
}

impl ContextExt for Context {
    fn send<T: Send + Sync + 'static, N: Into<Arc<T>>>(&self, src: N) -> Result<()> {
        self.scope().send(src.into())?;
        Ok(())
    }
    fn observe<
        Param: Params<Sender, Arc<dyn KAny>> + 'static,
        Func: FnMut<Param, Output = ()> + 'static,
        A: Into<Availed<Param, Func>>,
    >(
        &self,
        f: A,
    ) -> AvailHandle<Param, Func> {
        self.avails().add(f)
    }
}
