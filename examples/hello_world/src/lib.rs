use abi_stable::{export_root_module, prefix_type::PrefixTypeTrait, sabi_extern_fn, std_types::{RSlice, RStr}};
use carrion_api::{Action, Plugin, PluginInfo, PluginRef, event::Event};

#[export_root_module]
fn instantiate() -> PluginRef {
    Plugin {
        info: PluginInfo {
            id: RStr::from_str("hello_world"),
            description: RStr::from_str("Print 'Hello, World!' before loading"),
            author: RStr::from_str("serioscat"),
            version: RStr::from_str("0.1.0"),
            api_version: 1,
            priority: 0
        },
        on_event,
        dependencies: RSlice::empty()
    }
    .leak_into_prefix()
}

#[sabi_extern_fn]
fn on_event(event: &Event) -> Action {
    match event {
        Event::PluginEnter => {
            println!("Hello, World!");
            Action::Handled
        }
        Event::PluginLeave => {
            println!("Bye!");
            Action::Handled
        }
        _ => Action::Handled
    }
}