use crate::avail::*;
use crate::context::Context;
use crate::kissa::Kissa;
use kokoro_neo::any::*;
use std::sync::Arc;

pub trait ContextExt {
    fn observe<
        Param: Params<Kissa, Arc<dyn KAny>> + 'static,
        Func: FnMut<Param, Output = ()> + 'static,
        A: Into<Availed<Param, Func>>,
    >(
        &self,
        f: A,
    ) -> AvailHandle<Param, Func>;
}

impl ContextExt for Context {
    fn observe<
        Param: Params<Kissa, Arc<dyn KAny>> + 'static,
        Func: FnMut<Param, Output = ()> + 'static,
        A: Into<Availed<Param, Func>>,
    >(
        &self,
        f: A,
    ) -> AvailHandle<Param, Func> {
        self.avails().add(f)
    }
}
