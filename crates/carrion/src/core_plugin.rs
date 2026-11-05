// The first one is always loaded

use std::{collections::HashMap, str::FromStr};

use abi_stable::{
    sabi_extern_fn,
    prefix_type::PrefixTypeTrait,
    sabi_trait::TD_Opaque,
    std_types::{
        RResult::{self, RErr, ROk},
        RStr, RString
    }
};
use carrion_api::{
    Plugin, PluginInfo, PluginRef, Service,
    core_api::{
        CorePluginApi_TO, CorePlugin, CorePluginApi,
        CORE_ID, Handler
    }
};

use crate::{HANDLERS, SERVICES};

#[macro_export]
macro_rules! rresult_try {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => return abi_stable::std_types::RErr(format!("{e:#}").into()),
        }
    };
}

pub fn find_service(service_id: &str) -> Result<*const Service, String> {
    let services = crate::SERVICES.lock().unwrap();
    let result = services.get(service_id)
        .ok_or_else(|| String::from("Service not found"));

    Ok(result? as *const Service)
}

type Root = HashMap<String, toml::Table>;
const CARRION_CONFIG: &str = r#"
[carrion]
debug = false
toggle_overlay_bind = "Super+G"
exit_on_error = true
"#;
struct CorePluginImpl;

impl CorePluginApi for CorePluginImpl {
    fn get_config(&self, id: RStr) -> RResult<RString, RString> {
        let id = id.as_str();
        if id.is_empty() || !id.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
            return RErr("Invalid plugin id".into());
        }

        let dir = rresult_try!(dirs::config_dir().ok_or("Failed get config dir")).join("carrion");
        let path = dir.join("carrion.toml");

        if !path.exists() {
            rresult_try!(std::fs::create_dir_all(&dir));
            let mut root = Root::new();

            let content = rresult_try!(toml::Table::from_str(CARRION_CONFIG));
            root.insert("carrion".into(), content);
            rresult_try!(std::fs::write(&path, rresult_try!(toml::to_string_pretty(&root))));
        }

        let text = rresult_try!(std::fs::read_to_string(&path));
        let root: Root = rresult_try!(toml::from_str(&text));
        let raw = rresult_try!(root.get(id).ok_or(format!("No config section for {id}")));
        let result = rresult_try!(toml::to_string(raw));

        ROk(RString::from(result))
    }

    fn add_service(&self, service: Service) -> RResult<(), RString> {
        let id = service.id.to_string();
        if id == CORE_ID {
            return RErr("Registering the core service is forbidden".into());
        }

        let mut services = rresult_try!(SERVICES.lock().map_err(|e| e.to_string()));
        if services.contains_key(&id) {
            return RErr(format!("Service already registered: {id}").into());
        }

        services.insert(service.id.to_string(), service);

        ROk(())
    }

    fn remove_service(&self, service_id: RStr) -> RResult<(), RString> {
        let id = service_id.as_str();
        if id == CORE_ID {
            return RErr("Removing the core service is forbidden".into());
        }

        let mut services = rresult_try!(SERVICES.lock().map_err(|e| e.to_string()));
        if services.remove(id).is_none() {
            return RErr(format!("Service not found: {id}").into());
        }

        services.remove(service_id.as_str());

        ROk(())
    }

    fn find_service(&self, service: RStr) -> RResult<*const Service, RString> {
        find_service(&service).map_err(RString::from).into()
    }

    fn add_event_handler(&self, handler: Handler) {
        if let Ok(mut h) = HANDLERS.lock() {
            h.push(handler);
        }
    }
}

pub fn instantiate() -> PluginRef {
    let core_plugin: CorePlugin = CorePluginApi_TO::from_value(CorePluginImpl, TD_Opaque);
    SERVICES.lock().unwrap().insert(CORE_ID.to_string(), Service::new(CORE_ID, core_plugin));


    Plugin {
        info: PluginInfo {
            id: RStr::from_str(env!("CARGO_PKG_NAME")),
            description: RStr::from_str(env!("CARGO_PKG_DESCRIPTION")),
            author: RStr::from_str(env!("CARGO_PKG_AUTHORS")),
            version: RStr::from_str(env!("CARGO_PKG_VERSION")),
            api_version: 1
        },
        init,
        priority: 0
    }
    .leak_into_prefix()
}

#[sabi_extern_fn]
fn init(_core: &CorePlugin) -> RResult<(), RString> {
    RResult::ROk(())
}
