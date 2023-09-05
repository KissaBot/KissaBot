use kissa::{EventType, KissaPlugin, KissaResult, KissaSender, TestEventType};
#[derive(Default)]
struct Plugin;

impl KissaPlugin for Plugin {
    fn name(&self) -> &'static str {
        "my-plugin"
    }

    fn load(&self, sender: KissaSender<EventType>) -> KissaResult<EventType> {
        sender.send(EventType::Unknown(
            "bye".to_string(),
            Box::new(TestEventType::Message("world")),
        ))?;
        Ok(())
    }

    fn on_event(
        &self,
        event: &EventType,
        _sender: KissaSender<EventType>,
    ) -> KissaResult<EventType> {
        match event {
            EventType::Message(s) => {
                println!("my-plugin {}", s);
            }
            _ => {}
        }
        Ok(())
    }
}

#[no_mangle]
fn _create() -> Box<dyn KissaPlugin> {
    Box::new(Plugin::default())
}


