use abi_stable::{
    export_root_module, prefix_type::PrefixTypeTrait, sabi_extern_fn, std_types::{
        RResult, RStr, RString
    },
};
use carrion_api::{
    GuiEvent::Visibility, Plugin, PluginInfo, PluginRef, core_api::CorePlugin, event::Event,
};
use math_provider_api::*;

#[allow(dead_code)]
struct MathImpl;

impl MathApi for MathImpl {
    fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

#[export_root_module]
fn instantiate() -> PluginRef {
    Plugin {
        info: PluginInfo {
            id: RStr::from_str(env!("CARGO_PKG_NAME")),
            description: RStr::from_str(env!("CARGO_PKG_DESCRIPTION")),
            author: RStr::from_str(env!("CARGO_PKG_AUTHORS")),
            version: RStr::from_str(env!("CARGO_PKG_VERSION")),
            api_version: 1,
        },
        init,
        priority: -100_000_000
    }
    .leak_into_prefix()
}

#[sabi_extern_fn]
fn init(core: &CorePlugin) -> RResult<(), RString> {
    let math_plugin = match core.import::<Math>(PLUGIN_MATH_ID) {
        Ok(p) => p,
        Err(e) => return RResult::RErr(RString::from(e))
    };

    let result = math_plugin.add(5, 10);
    println!("5 + 10 = {result}");

    core.on_event(|event| {
        match event {
            Event::Gui(Visibility(true)) => println!("Hello, Carrion!"),
            Event::Gui(Visibility(false)) => println!("Bye, Carrion."),
            _ => {}
        }
    });

    RResult::ROk(())
}
