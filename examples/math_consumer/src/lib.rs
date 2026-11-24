use abi_stable::{
    export_root_module, prefix_type::PrefixTypeTrait, sabi_extern_fn, std_types::{
        RResult, RStr, RString
    },
};
use carrion_api::{Plugin, PluginInfo, PluginRef, core::feature::FeatureApiObj, event::{Event, KeyboardEvent, SystemEvent},
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
fn init(f: &FeatureApiObj) -> RResult<(), RString> {
    let logger = match f.import(carrion_api::core::logger::FEATURE_KEY) {
        Ok(p) => p,
        Err(e) => return RResult::RErr(RString::from(e))
    };

    carrion_api::core::logger::init_logger(logger);

    let math_plugin = match f.import(math_provider_api::FEATURE_KEY) {
        Ok(p) => p,
        Err(e) => return RResult::RErr(RString::from(e))
    };

    let result = math_plugin.add(10, 5);
    assert_eq!(10 + 5, result);

    let event_plugin = match f.import(carrion_api::core::event::FEATURE_KEY) {
        Ok(p) => p,
        Err(e) => return RResult::RErr(RString::from(e))
    };

    event_plugin.on_event(|event| {
        match event {
            Event::System(SystemEvent::Keyboard(KeyboardEvent::Grabbed)) => carrion_api::debug!("Keyboard grabbed"),
            Event::System(SystemEvent::Keyboard(KeyboardEvent::Ungrabbed)) => carrion_api::debug!("Keyboard ungrabbed"),
            e => carrion_api::debug!("Unhandled {e:?}")
        }
    });

    RResult::ROk(())
}
