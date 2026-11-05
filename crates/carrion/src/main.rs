use std::{collections::HashMap, sync::{LazyLock, Mutex}, thread::sleep, time::Duration};


use carrion_api::{Event, GuiEvent, PluginRef, Service, core_api::Handler};

mod overlay;
mod core_plugin;
mod load_plugins;

// use winit::event_loop::{EventLoop, ControlFlow};

static SERVICES: LazyLock<Mutex<HashMap<String, Service>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
static HANDLERS: LazyLock<Mutex<Vec<Handler>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static PLUGINS: LazyLock<Mutex<Vec<PluginRef>>> = LazyLock::new(|| Mutex::new(Vec::new()));

fn main() -> anyhow::Result<()> {
    // plugins injecting
    load_plugins::load()?;

    println!("\nEvents test:");

    let handlers = HANDLERS.lock().unwrap();

    for handler in handlers.iter() {
        handler.call(Event::Gui(GuiEvent::Visibility(true)));
    }
    sleep(Duration::from_secs(2));

    for handler in handlers.iter() {
        handler.call(Event::Gui(GuiEvent::Visibility(false)));
    }


    // building window
    // let event_loop = EventLoop::new().unwrap();
    // event_loop.set_control_flow(ControlFlow::Wait);

    // let _ = event_loop.run_app(&mut Overlay::default());

    // binding hotkeys

    Ok(())
}
