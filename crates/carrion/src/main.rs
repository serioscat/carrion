mod config;
mod core;
mod load_plugins;
mod overlay;
mod types;

use clap::Parser;
use global_hotkey::{
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
    hotkey::{Code, HotKey, Modifiers},
};
use parking_lot::Mutex;
use std::sync::LazyLock;

use time::macros::format_description;
use tracing_subscriber::{EnvFilter, fmt::time::LocalTime};

use carrion_api::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};

use crate::{config::Config, types::OverlayData};

type Lock<T> = LazyLock<Mutex<T>>;

static OVERLAY_DATA: Lock<OverlayData> = LazyLock::new(|| Mutex::new(OverlayData::default()));
static CONFIG: Lock<Config> = LazyLock::new(|| {
    let config = Config::get();

    if let Ok(c) = config {
        Mutex::new(c)
    } else {
        tracing::error!("Error: {:?}", config);
        Mutex::new(Config::default())
    }
});

pub fn send_event(event: Event) {
    tracing::debug!("Sending event: {event:?}");

    let handlers = OVERLAY_DATA.lock().event_handlers.clone();

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

    let debug = if !args.debug { "debug" } else { "info" };

    let timer = LocalTime::new(format_description!(
        "[year]-[month]-[day]T[hour repr:24]:[minute]:[second].[subsecond digits:2]Z"
    ));

    let env_filter = EnvFilter::new(format!("carrion={debug},plugin={debug}"));

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
            if let Err(err) = proxy.send_event(()) {
                tracing::error!("send_event failed: {err}");
            }
        }
    }));

    event_loop.run_app(&mut overlay::Overlay::new())?;

    Ok(())
}
