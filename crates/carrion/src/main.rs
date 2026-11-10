mod config;
mod overlay;
mod core_plugin;
mod load_plugins;

use std::{collections::HashMap, sync::{LazyLock, Mutex}};

use abi_stable::{sabi_trait::TD_Opaque, std_types::RStr};
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState, hotkey::{Code, HotKey, Modifiers}};

use tracing::info;
use time::macros::format_description;
use tracing_subscriber::{EnvFilter, fmt::time::LocalTime};

use winit::{event_loop::{ControlFlow, EventLoop}};
use carrion_api::{event::Event, PluginRef, Service, core_api::{CorePlugin, CorePluginApi_TO, Handler}};

use crate::{
    config::Config,
    core_plugin::CorePluginImpl,
    overlay::state::OverlayState
};


static SERVICES: LazyLock<Mutex<HashMap<String, Service>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
static HANDLERS: LazyLock<Mutex<Vec<Handler>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static PLUGINS: LazyLock<Mutex<Vec<PluginRef>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static CONFIG: LazyLock<Mutex<Config>> = LazyLock::new(|| Mutex::new(Config::default()));
static OVERLAY_STATE: LazyLock<Mutex<OverlayState>> = LazyLock::new(|| Mutex::new(OverlayState::default()));


pub fn send_event(event: Event) {
    tracing::debug!("Sending event: {event:?}");

    let core_plugin: CorePlugin = CorePluginApi_TO::from_value(CorePluginImpl, TD_Opaque);
    let handlers = HANDLERS.lock().unwrap();

    for handler in handlers.iter() {
        handler.call(&core_plugin, event.clone());
    }
}

fn main() -> anyhow::Result<()> {
    let timer = LocalTime::new(format_description!(
        "[year]-[month]-[day]T[hour repr:24]:[minute]:[second].[subsecond digits:2]Z"
    ));


    let core_plugin: CorePlugin = CorePluginApi_TO::from_value(CorePluginImpl, TD_Opaque);
    let mut config = CONFIG.lock().unwrap();
    *config = toml::from_str::<config::Config>(
        core_plugin
            .get_config(
                RStr::from("carrion")
            )
        .into_result()
        .map_err(String::from)
        .unwrap()
        .as_str()
    )?;

    let debug = if config.debug {
        "debug"
    } else { "info" };

    drop(config);

    let env_filter = EnvFilter::new(format!(
        "carrion={debug},plugin={debug}"
    ));

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_ansi_sanitization(false)
        .with_timer(timer)
        .with_level(true)
        .init();

    // plugins injecting
    load_plugins::load(&core_plugin)?;

    info!("Plugins loaded");

    // building window
    let event_loop = EventLoop::<()>::with_user_event().build()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let proxy = event_loop.create_proxy();

    let manager = GlobalHotKeyManager::new()?;
    let toggle_hotkey = HotKey::new(Some(Modifiers::META), Code::KeyG);

    manager.register(toggle_hotkey)?;

    let toggle_id = toggle_hotkey.id();

    GlobalHotKeyEvent::set_event_handler(Some(move |e: GlobalHotKeyEvent| {
        if e.id == toggle_id && e.state == HotKeyState::Released {
            let _ = proxy.send_event(());
        }
    }));

    event_loop.run_app(&mut overlay::Overlay::new())?;

    Ok(())
}
