#![feature(try_blocks)]
use kissa_topic::prelude::*;
use kissa_topic::setup_logger;
use libloading::Library;
use std::fs;
use std::path;
use std::thread;

struct EOP;
struct Main;
fn main() -> Result<()> {
    colog::init();
    let (tx, rx) = channel::unbounded();
    let ctx = Context::new(Main, Kissa::new(tx));
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
        info!("被用户停止");
        ctx_.global
            .publish(EOP)
            .expect("Error: Can Not to Signal EOP");
    })?;
    fs::create_dir_all("./plugins")?;
    let plugins = fs::read_dir("./plugins")?;
    for plugin_path in plugins {
        let name = plugin_path?.file_name();
        let path = path::Path::new("./plugins");
        let path = path.join(name);
        let result: Result<()> = try {
            let lib = unsafe { Library::new(&path) }?;
            setup_logger(&lib)?;
            ctx.dyn_plug(lib)?;
        };
        match result {
            Ok(_) => info!(
                "已加载插件 {}",
                path.file_stem()
                    .map(|v| v.to_str().unwrap_or("unknown"))
                    .unwrap_or("unknown")
            ),
            Err(err) => error!(
                "加载插件 {} 失败: {}",
                path.file_stem()
                    .map(|v| v.to_str().unwrap_or("unknown"))
                    .unwrap_or("unknown"),
                err
            ),
        }
    }
    handle.join().expect("Error: Main Loop Join Failed");
    Ok(())
}
