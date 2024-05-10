#![feature(unboxed_closures)]

pub use kokoro_neo as kokoro;

pub mod context {
    use std::sync::Arc;
    pub type Context = kokoro_neo::context::Context<
        flume::Sender<Arc<dyn kokoro_neo::any::KAny>>,
        Arc<dyn kokoro_neo::any::KAny>,
    >;
    pub use kokoro_neo::context::{ChildHandle, Children, RawContext, RawContextExt};
}

pub mod channel {
    pub use flume::*;
    use kokoro_neo::any::KAny;
    use std::sync::Arc;
    pub type Sender = flume::Sender<Arc<dyn KAny>>;
    pub type Receiver = flume::Receiver<Arc<dyn KAny>>;
}

pub mod avail {
    use kokoro_neo::any::KAny;
    use std::sync::Arc;

    pub type Availed<Param, Func> =
        kokoro_neo::avail::Availed<crate::channel::Sender, Param, Func, Arc<dyn KAny>>;
    pub type AvailHandle<Param, Func> =
        kokoro_neo::avail::AvailHandle<crate::channel::Sender, Param, Func, Arc<dyn KAny>>;
    pub use kokoro_neo::avail::Avail;
    pub use kokoro_neo::avail::Avails;
    pub use kokoro_neo::avail::Params;
}

pub mod context_ext;

pub mod prelude {
    pub use crate::avail::*;
    pub use crate::channel;
    pub use crate::context::*;
    pub use crate::context_ext::*;
    pub use kokoro_neo::any::*;
    pub use kokoro_neo::export_plugin;
    pub use kokoro_neo::plugin::dynamic::*;
    pub use kokoro_neo::plugin::*;
    pub use kokoro_neo::result::Result;
}
