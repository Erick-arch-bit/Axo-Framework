// Fase 4 — Objeto global `Device` para el contexto QuickJS (reutiliza `axo_core`).
//
// Diseño: Rust expone primitivas ocultas (`__axo_device_ns`, solo String/bool/Option)
// y un shim JS ensambla `globalThis.Device`. Así se evita construir `Object`
// dentro de callbacks (lifetimes de rquickjs) y el contrato JS queda explícito.
use std::sync::{Arc, Mutex};
use rquickjs::{Ctx, Function, Object};
use axo_core::device::{
    BatteryInfo, DeviceInfo, DisplayInfo, GeoLocation, NetworkInfo, SensorData, Storage,
    SystemInfo,
};
use axo_core::permissions::{
    AndroidPermissionHandler, DesktopPermissionHandler, IosPermissionHandler, Permission,
    PermissionHandler, PermissionState, WindowsPermissionHandler,
};

// Fase 5: estado compartido del dispositivo (antes en device_api.rs; solo queda lo que usa el bridge JS).
pub enum PlatformPermissions {
    Desktop(DesktopPermissionHandler),
    Android(AndroidPermissionHandler),
    Ios(IosPermissionHandler),
    Windows(WindowsPermissionHandler),
}

impl PermissionHandler for PlatformPermissions {
    fn check_permission(&self, perm: Permission) -> PermissionState {
        match self {
            Self::Desktop(h) => h.check_permission(perm),
            Self::Android(h) => h.check_permission(perm),
            Self::Ios(h) => h.check_permission(perm),
            Self::Windows(h) => h.check_permission(perm),
        }
    }

    fn request_permission(&mut self, perm: Permission) -> PermissionState {
        match self {
            Self::Desktop(h) => h.request_permission(perm),
            Self::Android(h) => h.request_permission(perm),
            Self::Ios(h) => h.request_permission(perm),
            Self::Windows(h) => h.request_permission(perm),
        }
    }

    fn open_settings(&mut self) -> bool {
        match self {
            Self::Desktop(h) => h.open_settings(),
            Self::Android(h) => h.open_settings(),
            Self::Ios(h) => h.open_settings(),
            Self::Windows(h) => h.open_settings(),
        }
    }
}

pub struct DeviceBridge {
    pub permissions: Arc<Mutex<PlatformPermissions>>,
    pub storage: Storage,
    pub geo: GeoLocation,
    pub sensors: SensorData,
    pub info: DeviceInfo,
    pub battery: BatteryInfo,
    pub network: NetworkInfo,
    pub display: DisplayInfo,
    pub system: SystemInfo,
}

impl Default for DeviceBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceBridge {
    pub fn new() -> Self {
        let platform = if cfg!(target_os = "android") {
            PlatformPermissions::Android(AndroidPermissionHandler::new())
        } else if cfg!(target_os = "ios") {
            PlatformPermissions::Ios(IosPermissionHandler::new())
        } else if cfg!(target_os = "windows") {
            PlatformPermissions::Windows(WindowsPermissionHandler::new())
        } else {
            PlatformPermissions::Desktop(DesktopPermissionHandler::new())
        };

        Self {
            permissions: Arc::new(Mutex::new(platform)),
            storage: Storage::new("axo"),
            geo: GeoLocation::mock(),
            sensors: SensorData::current(),
            info: DeviceInfo::current(),
            battery: BatteryInfo::current(),
            network: NetworkInfo::current(),
            display: DisplayInfo::current(),
            system: SystemInfo::current(),
        }
    }
}

// Fase 4: mapeo de estados de permiso a string (granted/denied/... + "unsupported").
fn perm_state_str(state: PermissionState) -> &'static str {
    match state {
        PermissionState::Granted => "granted",
        PermissionState::Denied => "denied",
        PermissionState::NotRequested => "not_requested",
        PermissionState::Restricted => "restricted",
        PermissionState::Limited => "limited",
        PermissionState::PermanentlyDenied => "permanently_denied",
    }
}

// Fase 4: f64/f32 finitos para JSON (evita nulls por NaN/inf).
fn fin(v: f64) -> f64 {
    if v.is_finite() {
        v
    } else {
        0.0
    }
}

// Fase 4: shim JS que expone el contrato `Device` sobre las primitivas Rust.
const DEVICE_SHIM: &str = r#"
(function () {
  var ns = globalThis.__axo_device_ns;
  globalThis.Device = {
    info: function () { return JSON.parse(ns.__axo_device_info()); },
    checkPermission: function (name) { return ns.__axo_perm_check(String(name)); },
    requestPermission: function (name) { return ns.__axo_perm_request(String(name)); },
    getLocation: function () { return JSON.parse(ns.__axo_device_location()); },
    getSensors: function () { return JSON.parse(ns.__axo_device_sensors()); },
    readFile: function (path) {
      var s = ns.__axo_read_file(String(path));
      if (s === null || s === undefined) {
        throw new Error("Device.readFile: no se pudo leer '" + path + "'");
      }
      return s;
    },
    writeFile: function (path, content) {
      return !!ns.__axo_write_file(String(path), String(content));
    },
    deleteFile: function (path) {
      return !!ns.__axo_delete_file(String(path));
    },
    showNotification: function (title, body) {
      return !!ns.__axo_notify(String(title), String(body));
    },
    takePhoto: function () {
      console.log("[JS Device] not available: takePhoto");
      return null;
    }
  };
})();
"#;

// Fase 4: registra `globalThis.Device` con la Device API mínima en el contexto dado.
pub(crate) fn register_js_device(ctx: Ctx) -> Result<(), rquickjs::Error> {
    let bridge = Arc::new(Mutex::new(DeviceBridge::new()));
    let ns = Object::new(ctx.clone())?;

    // Fase 4 (real): JSON de DeviceInfo desde axo_core::device.
    {
        let b = Arc::clone(&bridge);
        ns.set(
            "__axo_device_info",
            Function::new(ctx.clone(), move || {
                let b = b.lock().unwrap();
                let info = &b.info;
                serde_json::to_string(&serde_json::json!({
                    "os_name": info.os_name,
                    "os": info.os_name,
                    "os_version": info.os_version,
                    "device_model": info.device_model,
                    "screen_width": fin(info.screen_width as f64),
                    "screen_height": fin(info.screen_height as f64),
                    "is_mobile": info.is_mobile,
                    "language": info.language,
                    "timezone": info.timezone,
                    "app_version": info.app_version,
                }))
                .unwrap_or_else(|_| "{}".to_string())
            })?,
        )?;
    }

    // Fase 4 (real): checkPermission reutilizando el handler de plataforma.
    {
        let b = Arc::clone(&bridge);
        ns.set(
            "__axo_perm_check",
            Function::new(ctx.clone(), move |name: String| {
                let b = b.lock().unwrap();
                name.parse::<Permission>()
                    .map(|p| b.permissions.lock().unwrap().check_permission(p))
                    .map(perm_state_str)
                    .unwrap_or("unsupported")
                    .to_string()
            })?,
        )?;
    }

    // Fase 4 (real donde la plataforma lo soporta, stub en caso contrario).
    {
        let b = Arc::clone(&bridge);
        ns.set(
            "__axo_perm_request",
            Function::new(ctx.clone(), move |name: String| {
                let b = b.lock().unwrap();
                name.parse::<Permission>()
                    .map(|p| b.permissions.lock().unwrap().request_permission(p))
                    .map(perm_state_str)
                    .unwrap_or("unsupported")
                    .to_string()
            })?,
        )?;
    }

    // Fase 4 (real): JSON de ubicación { lat, lng, accuracy }.
    {
        let b = Arc::clone(&bridge);
        ns.set(
            "__axo_device_location",
            Function::new(ctx.clone(), move || {
                let b = b.lock().unwrap();
                let geo = &b.geo;
                serde_json::to_string(&serde_json::json!({
                    "lat": fin(geo.latitude),
                    "lng": fin(geo.longitude),
                    "accuracy": fin(geo.accuracy),
                }))
                .unwrap_or_else(|_| "{}".to_string())
            })?,
        )?;
    }

    // Fase 4 (real): JSON de sensores, null donde no hay sensor.
    {
        let b = Arc::clone(&bridge);
        ns.set(
            "__axo_device_sensors",
            Function::new(ctx.clone(), move || {
                let b = b.lock().unwrap();
                let s = &b.sensors;
                let triple = |t: Option<(f32, f32, f32)>| match t {
                    Some((x, y, z)) => serde_json::json!({
                        "x": fin(x as f64), "y": fin(y as f64), "z": fin(z as f64)
                    }),
                    None => serde_json::Value::Null,
                };
                serde_json::to_string(&serde_json::json!({
                    "accelerometer": triple(s.accelerometer),
                    "gyroscope": triple(s.gyroscope),
                    "magnetometer": triple(s.magnetometer),
                }))
                .unwrap_or_else(|_| "{}".to_string())
            })?,
        )?;
    }

    // Fase 4 (real): readFile → contenido o null (el shim lanza el error controlado).
    {
        let b = Arc::clone(&bridge);
        ns.set(
            "__axo_read_file",
            Function::new(ctx.clone(), move |path: String| {
                let b = b.lock().unwrap();
                match b.storage.read(&path) {
                    Ok(data) => Some(String::from_utf8_lossy(&data).to_string()),
                    Err(_) => None,
                }
            })?,
        )?;
    }

    // Fase 4 (real): writeFile → boolean.
    {
        let b = Arc::clone(&bridge);
        ns.set(
            "__axo_write_file",
            Function::new(ctx.clone(), move |path: String, content: String| {
                let b = b.lock().unwrap();
                b.storage.write(&path, content.as_bytes()).is_ok()
            })?,
        )?;
    }

    // Fase 4 (real): deleteFile → boolean.
    {
        let b = Arc::clone(&bridge);
        ns.set(
            "__axo_delete_file",
            Function::new(ctx.clone(), move |path: String| {
                let b = b.lock().unwrap();
                b.storage.delete(&path).is_ok()
            })?,
        )?;
    }

    // Fase 4 (stub seguro): showNotification → log + true.
    {
        ns.set(
            "__axo_notify",
            Function::new(ctx.clone(), move |title: String, body: String| {
                println!("[Notification] {title}: {body}");
                true
            })?,
        )?;
    }

    ctx.globals().set("__axo_device_ns", ns)?;
    let _: rquickjs::Value = ctx.eval(DEVICE_SHIM)?;
    Ok(())
}
