#![feature(try_blocks)]
use kissa_topic::lua::prelude::*;
use kissa_topic::prelude::*;
use std::fs;
use std::path;
use std::thread;

struct EOP;
struct Main;
fn main() -> Result<()> {
    colog::init();
    let lua = Lua::new();
    let lua_globals = lua.globals();
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
    let path = path::Path::new("./plugins");
    let ctx_ = ctx.clone();
    let plugin_function = lua.create_function(move |lua, param: String| {
        let path = path.join(param);
        let ctx__ = ctx_.clone();
        let func = lua.create_function(move |lua, param: LuaValue| {
            let lib = unsafe { Library::new(&path) }
                .map_err(|err| LuaError::RuntimeError(err.to_string()))?;
            ctx__
                .dyn_plug(lib, &lua, param)
                .map_err(|err| LuaError::RuntimeError(err.to_string()))?;
            Ok(())
        });
        Ok(func)
    })?;
    lua_globals.set("Plugin", plugin_function)?;
    let lua_script = match fs::read("./init.lua") {
        Ok(file) => file,
        Err(_) => {
            fs::write("./init.lua", [])?;
            vec![]
        }
    };
    lua.load(lua_script).exec()?;
    handle.join().expect("Error: Main Loop Join Failed");
    Ok(())
}
