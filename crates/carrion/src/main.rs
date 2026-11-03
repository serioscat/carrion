use std::{collections::HashMap, path::Path, sync::{LazyLock, Mutex}};

use abi_stable::{library::{RootModule, lib_header_from_path}, std_types::{RResult::{self, RErr, ROk}, RStr, RString}};
use carrion_api::{HostApi, PluginRef, Service};

mod overlay;
mod config;

// use winit::event_loop::{EventLoop, ControlFlow};

// use crate::overlay::Overlay;

static SERVICES: LazyLock<Mutex<HashMap<String, HashMap<String, Service>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

extern "C" fn find_service(plugin_id: RStr, service: RStr) -> RResult<*const Service, RString> {
    let services = SERVICES.lock().unwrap();
    let result = services
        .get(plugin_id.into())
        .ok_or_else(|| RString::from("Plugin not found"))
        .and_then(|services| {
            services
                .get(service.into())
                .ok_or_else(|| RString::from("Service not found"))
        });

    match result {
        Ok(service) => ROk(service as *const Service),
        Err(error) => RErr(error),
    }
}

fn process_new_service(plugins: Vec<&PluginRef>) {
    for plugin in plugins {
        let info = plugin.info();
        println!("\t- {} | {}", info.id, info.version);

        let services = plugin.services()();
        let mut services_map = HashMap::new();

        for service in services {
            services_map.insert(service.id.to_string(), service);
        }

        SERVICES.lock().unwrap().insert(info.id.into(), services_map);
    }
}

fn load_plugin(path: &Path) -> anyhow::Result<PluginRef> {
    let header = lib_header_from_path(path)?;
    Ok(header.init_root_module::<PluginRef>()?)
}

fn main() -> anyhow::Result<()> {
    // plugins injecting
    let provider_bind = load_plugin(Path::new("target/debug/libmath_provider.so"));
    let consumer_bind = load_plugin(Path::new("target/debug/libmath_consumer.so"));

    let provider = provider_bind.as_ref().unwrap();
    let consumer = consumer_bind.as_ref().unwrap();

    process_new_service(vec![
        provider, consumer
    ]);

    provider.init()(HostApi { find_service });
    consumer.init()(HostApi { find_service });




    // building window
    // let event_loop = EventLoop::new().unwrap();
    // event_loop.set_control_flow(ControlFlow::Wait);

    // let _ = event_loop.run_app(&mut Overlay::default());

    // binding hotkeys

    Ok(())
}
