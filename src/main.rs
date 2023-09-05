use crossbeam_channel::unbounded;
use dynamic_reload::DynamicReload;
use kissa::{EventType, KissaPlugin, KissaSender, PluginCreater};
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
type Plugin = (u32, Box<dyn KissaPlugin>);
type Plugins = Vec<Arc<Mutex<Plugin>>>;
type JoinHandles<T> = Vec<thread::JoinHandle<T>>;
fn main() {
    let mut reload_handler = DynamicReload::new(
        Some(vec!["./test_plugin/target/debug"]),
        Some(&std::env::temp_dir().to_str().unwrap()),
        dynamic_reload::Search::Default,
        Duration::from_secs(2),
    );

    let plugin_list: Vec<String> = vec!["test_plugin".to_string()];

    let mut plugins: Plugins = Vec::new();
    let mut id: u32 = 0;
    for plugin_name in plugin_list {
        match unsafe { reload_handler.add_library(&plugin_name, dynamic_reload::PlatformName::Yes) }
        {
            Ok(lib) => {
                let creator: Result<dynamic_reload::Symbol<PluginCreater>, _> =
                    unsafe { lib.lib.get(b"_create") };
                match creator {
                    Ok(creator) => {
                        let plugin = creator();
                        plugins.push(Arc::new(Mutex::new((id, plugin))));
                        id += 1;
                    }
                    Err(e) => println!("Err {}", e.to_string()),
                }
            }
            Err(e) => {
                println!("Err {}", e.to_string());
            }
        }
    }

    let (ms, mr) = unbounded::<(u32, EventType)>();

    let mut jhs: JoinHandles<_> = Vec::new();
    for plugin in plugins.clone() {
        let plugin_ = plugin.clone();
        let ms_ = ms.clone();
        let jh = thread::spawn(move || {
            let plugin = plugin_.lock().unwrap();
            plugin
                .1
                .load(KissaSender::from_sender(plugin.0, ms_))
                .unwrap();
        });
        jhs.push(jh);
    }
    join(jhs);

    loop {
        let event = Arc::new(Mutex::new(mr.recv().unwrap()));
        match &event.lock().unwrap().1 {
            EventType::QuitKissa(a) => {
                exit(*a);
            }
            _ => {}
        }
        for plugin in &plugins {
            let plugin_ = plugin.clone();
            let ms_ = ms.clone();
            let event_ = event.clone();
            let _ = thread::spawn(move || {
                let event = event_.lock().unwrap();
                let plugin = plugin_.lock().unwrap();
                if event.0 != plugin.0 {
                    plugin
                        .1
                        .on_event(&event.1, KissaSender::from_sender(plugin.0, ms_))
                        .unwrap();
                }
            })
            .join();
        }
    }
}
fn join<T>(jhs: JoinHandles<T>) {
    for jh in jhs {
        jh.join().unwrap();
    }
}
