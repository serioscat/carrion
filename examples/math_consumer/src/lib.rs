use abi_stable::{
    export_root_module, prefix_type::PrefixTypeTrait, sabi_extern_fn, std_types::{
        RResult, RStr, RString
    },
};
use carrion_api::{Plugin, PluginInfo, PluginRef, core_api::CorePlugin, event::Event,
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
            version: RStr::from_str(env!("CARGO_PKG_VERSION"))
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

    let result = math_plugin.add(10, 5);
    assert_eq!(10 + 5, result);

    core.on_event(|plugin, event| {
        match event {
            Event::Visible(true) => carrion_api::info!(plugin, "Hello, Carrion!"),
            Event::Visible(false) => carrion_api::info!(plugin, "Bye, Carrion."),
            _ => {}
        }
    });

    RResult::ROk(())
}
