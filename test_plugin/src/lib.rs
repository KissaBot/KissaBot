use kissa::{EventType, KissaPlugin, KissaResult, KissaSender, TestEventType};
#[derive(Default)]
struct TestPlugin;

impl KissaPlugin for TestPlugin {
    fn name(&self) -> &'static str {
        "test_plugin"
    }
    fn load(&self, sender: KissaSender<EventType>) -> KissaResult<EventType> {
        sender.send(EventType::Message("hello world".to_string()))?;
        Ok(())
    }
    fn on_event(&self, e: &EventType, sender: KissaSender<EventType>) -> KissaResult<EventType> {
        match e {
            EventType::Unknown(s, e) => {
                println!("{}", s);
                match e.downcast_ref::<TestEventType>() {
                    Some(a) => {
                        let TestEventType::Message(b) = a;
                        println!("{}", b);
                    }
                    None => {
                        println!("没有数据？")
                    }
                }
            }
            _ => {}
        }

        sender.send(EventType::QuitKissa(10))?;
        Ok(())
    }
}

#[no_mangle]
fn _create() -> Box<dyn KissaPlugin> {
    Box::new(TestPlugin::default())
}
