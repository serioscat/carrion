use std::path::Path;

use abi_stable::library::RootModule;
use carrion_api::{PluginRef, event::Event};

mod overlay;
mod config;

// use winit::event_loop::{EventLoop, ControlFlow};

// use crate::overlay::Overlay;

fn main() {
    // plugins injecting
    let plugin = PluginRef::load_from_file(Path::new("target/debug/libhello_world.so"));
    let info = plugin.as_ref().unwrap().info();
    assert_eq!(info.api_version, 1);
    println!("\t- {} | {}", info.id, info.version);
    let on_event = plugin.as_ref().unwrap().on_event();
    on_event(&Event::PluginEnter);


    // building window
    // let event_loop = EventLoop::new().unwrap();
    // event_loop.set_control_flow(ControlFlow::Wait);

    // let _ = event_loop.run_app(&mut Overlay::default());

    // binding hotkeys
    
}
