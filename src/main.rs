#![feature(try_blocks)]
use kissa_topic::lua::prelude::*;
use kissa_topic::lua::Variadic;
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
    init_lua(&lua, lua_globals, &ctx)?;
    let lua_script = match fs::read("./init.lua") {
        Ok(file) => file,
        Err(_) => {
            fs::write("./init.lua", [])?;
            vec![]
        }
    };
    lua.load(lua_script)
        .exec()
        .unwrap_or_else(|err| error!("Lua 脚本错误，实例将无法正常工作，修改后重新运行:\n{}", err));
    handle.join().expect("Error: Main Loop Join Failed");
    Ok(())
}

fn init_lua(lua: &Lua, globals: LuaTable, ctx: &Context<Main>) -> Result<()> {
    fs::create_dir_all("./plugins")?;
    let path = path::Path::new("./plugins");
    let ctx_ = ctx.clone();
    let plugin_function = lua.create_function(move |lua, param: String| {
        let path = path.join(param);
        let ctx__ = ctx_.clone();
        let func = lua.create_function(move |lua, param: LuaValue| {
            let result: Result<_> = try {
                let lib = unsafe { Library::new(&path) }?;
                ctx__.dyn_plug(lib, &lua, param)?
            };
            match result {
                Err(err) => Err(LuaError::RuntimeError(format!(
                    "插件加载失败\npath: {}\nerror: {}",
                    path.to_str().unwrap_or("unknown"),
                    err
                ))),
                Ok(handle) => Ok(handle.0),
            }
        });
        Ok(func)
    })?;
    let ctx_ = ctx.clone();
    let unplug_function = lua.create_function(move |_, param: u64| {
        ctx_.raw().children.remove(&param);
        Ok(())
    })?;
    let info_function = lua.create_function(lua_info)?;
    let warn_function = lua.create_function(lua_warn)?;
    let error_function = lua.create_function(lua_error)?;
    let debug_function = lua.create_function(lua_debug)?;
    globals.set("Plugin", plugin_function)?;
    globals.set("Unplug", unplug_function)?;
    globals.set("info", &info_function)?;
    globals.set("print", info_function)?;
    globals.set("warn", warn_function)?;
    globals.set("error", error_function)?;
    globals.set("debug", debug_function)?;
    Ok(())
}

fn lua_info(_: &Lua, strings: Variadic<String>) -> Result<(), LuaError> {
    let string = strings.join("");
    info!("{string}");
    Ok(())
}

fn lua_warn(_: &Lua, strings: Variadic<String>) -> Result<(), LuaError> {
    let string = strings.join("");
    warn!("{string}");
    Ok(())
}

fn lua_error(_: &Lua, strings: Variadic<String>) -> Result<(), LuaError> {
    let string = strings.join("");
    error!("{string}");
    Ok(())
}

fn lua_debug(_: &Lua, strings: Variadic<String>) -> Result<(), LuaError> {
    let string = strings.join("");
    debug!("{string}");
    Ok(())
}
