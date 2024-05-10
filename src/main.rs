use kissa_topic::prelude::*;
use std::fs;
use std::thread;

struct EOP;
fn main() -> Result<()> {
    let (tx, rx) = channel::unbounded();
    let ctx = Context::new(tx);
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
        ctx_.send(EOP).expect("Error: Can Not to Signal EOP");
    })?;
    fs::create_dir_all("./plugins")?;
    let plugins = fs::read_dir("./plugins")?;
    for plugin_path in plugins {
        let name = plugin_path?.file_name();
        ctx.dyn_plug(name.to_str().expect("Error: Can Not Read File Name"))?;
    }
    handle.join().expect("Error: Main Loop Join Failed");
    Ok(())
}
