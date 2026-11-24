use abi_stable::{
    export_root_module, prefix_type::PrefixTypeTrait, sabi_extern_fn, std_types::{RResult, RStr, RString}
};
use carrion_api::{
    Plugin, PluginInfo, PluginRef, core::feature::FeatureApiObj, types::{math::vec2, window::Label}
};

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
        priority: 12_345
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

    let win_api = match f.import(carrion_api::core::window::FEATURE_KEY) {
        Ok(p) => p,
        Err(e) => return RResult::RErr(RString::from(e))
    };

    let RResult::ROk(mut window) = win_api.create_window(
        RStr::from_str(env!("CARGO_PKG_NAME")),
        RStr::from("example"),
        RStr::from("Example window")
    ) else {
        return RResult::RErr(RString::from("Failed create window"))
    };

    carrion_api::info!("Window created!");

    window.set_position(vec2(250., 300.));
    window.show(|ui| {
        let label = Label::new("Label!");
        if ui.label(label).clicked() {
            carrion_api::info!("Label clicked");
        }
    });


    RResult::ROk(())
}