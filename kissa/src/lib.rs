use crossbeam_channel::Sender;
pub use crossbeam_channel::{self, SendError};
use std::any::Any;
#[derive(Debug)]
pub enum EventType {
    Message(String),
    QuitKissa(i32),
    Unknown(String, Box<dyn Any>),
}
unsafe impl Send for EventType {}

pub type KissaResult<T> = Result<(), SendError<(u32, T)>>;

pub struct KissaSender<T>(u32, Sender<(u32, T)>);

impl<T> KissaSender<T> {
    pub fn send(&self, msg: T) -> KissaResult<T> {
        self.1.send((self.0, msg))
    }
    pub fn from_sender(id: u32, sender: Sender<(u32, T)>) -> Self {
        KissaSender(id, sender)
    }
    pub unsafe fn get_sender(&self) -> Sender<(u32, T)> {
        self.1.clone()
    }
}

pub trait KissaPlugin: Any + Send + Sync {
    fn name(&self) -> &'static str;
    fn load(&self, _sender: KissaSender<EventType>) -> KissaResult<EventType> {
        Ok(())
    }
    fn on_event(
        &self,
        event: &EventType,
        _sender: KissaSender<EventType>,
    ) -> KissaResult<EventType>;
    fn drop(&self) {}
}

pub type PluginCreater = fn() -> Box<dyn KissaPlugin>;

pub struct TestEvent<'a> {
    pub message: &'a str,
}
pub enum TestEventType<'a> {
    Message(&'a str),
}
