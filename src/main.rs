use kissa_topic::prelude::*;
use kissabot::event::*;
use kissabot::resources::Message;
use std::fs;
use std::thread;

struct EOP;
fn main() -> Result<()> {
    let (tx, rx) = channel::unbounded();
    let ctx = DefaultContext::new(Kissa::new(tx));
    let ctx_ = ctx.clone();
    let handle = thread::spawn(move || {
        for event in rx {
            if event.is::<EOP>() {
                break;
            }
            ctx_(event);
        }
    });
    let ctx_ = ctx.clone();
    ctrlc::set_handler(move || {
        println!("END");
        ctx_.publish(EOP).expect("Error: Can Not to Signal EOP");
    })?;
    fs::create_dir_all("./plugins")?;
    let plugins = fs::read_dir("./plugins")?;
    for plugin_path in plugins {
        let name = plugin_path?.file_name();
        ctx.dyn_plug(
            format!(
                "./plugins/{}",
                name.to_str().expect("Error: Can Not Read File Name")
            )
            .as_str(),
        )?;
    }
    ctx.publish(SEvent {
        content: SEventContent {
            ty: SEventType::MessageCreated,
            message: Some(Message {
                id: "".to_string(),
                content: "Hello".to_string(),
                channel: None,
                guild: None,
                member: None,
                user: None,
                created_at: None,
                updated_at: None,
            }),
            ..Default::default()
        },
        from_adapter: 0,
    })?;
    handle.join().expect("Error: Main Loop Join Failed");
    Ok(())
}
