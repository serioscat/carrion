use abi_stable::{
    std_types::{
        RResult::{self, RErr, ROk}, RStr, RString
    }
};
use carrion_api::{
    Service, LogLevel, core_api::{
        CorePluginApi, CORE_ID, Handler
    }
};

use crate::{HANDLERS, SERVICES, config::Config};

#[macro_export]
macro_rules! rresult_try {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => return RErr(format!("{e:#}").into()),
        }
    };
}

pub fn find_service(service_id: &str) -> Result<*const Service, String> {
    let services = crate::SERVICES.lock().unwrap();
    let result = services.get(service_id)
        .ok_or_else(|| String::from("Service not found"));

    Ok(result? as *const Service)
}

pub struct CorePluginImpl;

impl CorePluginApi for CorePluginImpl {
    fn get_config(&self, id: RStr) -> RResult<RString, RString> {
        let id = id.as_str();
        if id.is_empty() || !id.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
            return RErr("Invalid plugin id".into());
        }

        let dir = rresult_try!(dirs::config_dir().ok_or("Failed get config dir")).join("carrion");
        let path = dir.join("carrion.toml");

        let text = if path.exists() {
            rresult_try!(std::fs::read_to_string(&path))
        } else {
            String::new()
        };

        let mut root: toml::Table = if text.trim().is_empty() {
            toml::Table::new()
        } else {
            rresult_try!(toml::from_str(&text))
        };

        let raw = match root.get(id) {
            Some(v) => v.clone(),
            None if id == "carrion" => {
                let defaults = toml::Value::Table(
                    rresult_try!(toml::Table::try_from(&Config::default()))
                );

                if root.is_empty() {
                    root.insert(id.to_string(), defaults.clone());
                    rresult_try!(std::fs::create_dir_all(&dir));
                    rresult_try!(std::fs::write(&path, rresult_try!(toml::to_string_pretty(&root))));
                }

                defaults
            }
            None => return RErr(RString::from(format!("No config section for {id}")))
        };

        let result = rresult_try!(toml::to_string(&raw));
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

    fn _log(&self, level: LogLevel, target: RStr<'_>, message: RStr<'_>) {
        let (t, m) = (target.as_str(), message.as_str());

        // im mb replace this
        let message = format!("\x1b[1D\x1b[2m{t}:\x1b[0m {m}");

        match level {
            LogLevel::Debug => tracing::debug!(target: "plugin:", "{message}"),
            LogLevel::Info  => tracing::info!( target: "plugin:", "{message}"),
            LogLevel::Warn  => tracing::warn!( target: "plugin:", "{message}"),
            LogLevel::Error => tracing::error!(target: "plugin:", "{message}"),
        }
    }

    fn _find_service(&self, service: RStr) -> RResult<*const Service, RString> {
        find_service(&service).map_err(RString::from).into()
    }

    fn _add_event_handler(&self, handler: Handler) {
        if let Ok(mut h) = HANDLERS.lock() {
            h.push(handler);
        }
    }
}
