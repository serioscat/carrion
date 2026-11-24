mod config;
mod overlay;
mod core;
mod types;
mod load_plugins;

use std::{collections::HashMap, sync::LazyLock};
use parking_lot::Mutex;
use abi_stable::std_types::RArc;
use clap::Parser;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState, hotkey::{Code, HotKey, Modifiers}};

use time::macros::format_description;
use tracing_subscriber::{EnvFilter, fmt::time::LocalTime};

use winit::{event_loop::{ControlFlow, EventLoop}};
use carrion_api::{
    PluginRef, core::{event::EventFnObj, feature::Feature, window::RenderFnObj}, event::Event
};
use crate::{
    config::Config, core::window::WindowRef, overlay::state::OverlayState
};

type Lock<T> = LazyLock<Mutex<T>>;

static FEATURES: Lock<HashMap<String, HashMap<String, RArc<Feature>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
static EVENT_HANDLERS: Lock<Vec<EventFnObj>> = LazyLock::new(|| Mutex::new(Vec::new()));
static RENDER_HANDLERS: Lock<Vec<RenderFnObj>> = LazyLock::new(|| Mutex::new(Vec::new()));
static WINDOWS: Lock<HashMap<String, HashMap<String, WindowRef>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
static PLUGINS: Lock<Vec<PluginRef>> = LazyLock::new(|| Mutex::new(Vec::new()));
static CONFIG: Lock<Config> = LazyLock::new(|| {
    let config = Config::get();

    if let Ok(c) = config {
        Mutex::new(c)
    } else {
        tracing::error!("Error: {:?}", config);
        Mutex::new(Config::default())
    }
});
static OVERLAY_STATE: Lock<OverlayState> = LazyLock::new(|| Mutex::new(OverlayState::default()));

pub fn send_event(event: Event) {
    tracing::debug!("Sending event: {event:?}");

    let handlers = EVENT_HANDLERS.lock();

    for handler in handlers.iter() {
        handler._call(event.clone());
    }
}

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(long)]
    debug: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let debug = if !args.debug {
        "debug"
    } else { "info" };

    let timer = LocalTime::new(format_description!(
        "[year]-[month]-[day]T[hour repr:24]:[minute]:[second].[subsecond digits:2]Z"
    ));

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
    load_plugins::load()?;

    tracing::info!("Plugins loaded");

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
