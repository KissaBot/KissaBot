pub use kissa_topic as topic;
mod event_ext;
pub mod event {
    pub use crate::event_ext::*;
    pub use kissa_satori::event::SatoriEvent as SEventContent;
    pub struct SEvent {
        pub content: SEventContent,
        pub from_adapter: u64,
    }
    pub use kissa_satori::event::SatoriEventType as SEventType;
}
pub mod api {
    pub use kissa_satori::api::*;
}
pub mod resources {
    pub use kissa_satori::resources::*;
}
