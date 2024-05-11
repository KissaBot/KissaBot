use kissa_topic::adapter::CreateAdapterFn;
use kissa_topic::prelude::*;
use libloading::{Library, Symbol};
use std::fs;
use std::path;
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
        if let Some(name) = name.to_str() {
            let path = path::Path::new("./plugins");
            let path = path.join(name);
            let lib = unsafe { Library::new(path) }?;
            if name.starts_with("adapter") {
                let create_adapter: Symbol<CreateAdapterFn> =
                    unsafe { lib.get(b"__create_adapter__") }?;
                let adapter = create_adapter();
                ctx.adapters.insert(&adapter as *const _ as u64, adapter);
            }
            ctx.dyn_plug(lib)?;
        }
    }
    handle.join().expect("Error: Main Loop Join Failed");
    Ok(())
}
