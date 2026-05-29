use mlua::prelude::*;
use std::sync::{Arc, Mutex};

use axo_core::device::{
    BatteryInfo, DeviceInfo, DisplayInfo, GeoLocation, NetworkInfo, SensorData, Storage, SystemInfo,
    Clipboard, Haptics, HapticFeedback,
};
use axo_core::permissions::{
    AndroidPermissionHandler, DesktopPermissionHandler, IosPermissionHandler,
    Permission, PermissionHandler, PermissionState, WindowsPermissionHandler,
};

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

pub fn register_device_api(lua: &Lua, bridge: Arc<Mutex<DeviceBridge>>) -> LuaResult<()> {
    let device_table = lua.create_table()?;

    // ── Device Info ──
    {
        let bridge = Arc::clone(&bridge);
        let info_fn = lua.create_function(move |lua, _: ()| {
            let b = bridge.lock().unwrap();
            let info = &b.info;
            let t = lua.create_table()?;
            t.set("os_name", info.os_name.as_str())?;
            t.set("os_version", info.os_version.as_str())?;
            t.set("device_model", info.device_model.as_str())?;
            t.set("screen_width", info.screen_width)?;
            t.set("screen_height", info.screen_height)?;
            t.set("is_mobile", info.is_mobile)?;
            t.set("language", info.language.as_str())?;
            t.set("timezone", info.timezone.as_str())?;
            t.set("app_version", info.app_version.as_str())?;
            Ok(t)
        })?;
        device_table.set("info", info_fn)?;
    }

    // ── System Info ──
    {
        let bridge = Arc::clone(&bridge);
        let sys_fn = lua.create_function(move |lua, _: ()| {
            let b = bridge.lock().unwrap();
            let s = &b.system;
            let t = lua.create_table()?;
            t.set("total_memory_mb", s.total_memory_mb)?;
            t.set("free_memory_mb", s.free_memory_mb)?;
            t.set("cpu_usage", s.cpu_usage)?;
            t.set("cpu_cores", s.cpu_cores)?;
            t.set("cpu_arch", s.cpu_arch.as_str())?;
            t.set("kernel_version", s.kernel_version.as_str())?;
            t.set("uptime_seconds", s.uptime_seconds)?;
            t.set("hostname", s.hostname.as_str())?;
            t.set("username", s.username.as_str())?;
            t.set("locale", s.locale.as_str())?;
            Ok(t)
        })?;
        device_table.set("getSystemInfo", sys_fn)?;
    }

    // ── Display Info ──
    {
        let bridge = Arc::clone(&bridge);
        let disp_fn = lua.create_function(move |lua, _: ()| {
            let b = bridge.lock().unwrap();
            let d = &b.display;
            let t = lua.create_table()?;
            t.set("width", d.width)?;
            t.set("height", d.height)?;
            t.set("dpi", d.dpi)?;
            t.set("refresh_rate", d.refresh_rate)?;
            t.set("brightness", d.brightness)?;
            t.set("orientation", d.orientation.as_str())?;
            t.set("color_gamut", d.color_gamut.as_str())?;
            t.set("hdr_supported", d.hdr_supported)?;
            Ok(t)
        })?;
        device_table.set("getDisplayInfo", disp_fn)?;
    }

    // ── Battery Info ──
    {
        let bridge = Arc::clone(&bridge);
        let bat_fn = lua.create_function(move |lua, _: ()| {
            let b = bridge.lock().unwrap();
            let bat = &b.battery;
            let t = lua.create_table()?;
            t.set("level", bat.level)?;
            t.set("is_charging", bat.is_charging)?;
            t.set("health", bat.health.as_str())?;
            t.set("temperature", bat.temperature)?;
            t.set("voltage", bat.voltage)?;
            t.set("capacity", bat.capacity)?;
            Ok(t)
        })?;
        device_table.set("getBatteryInfo", bat_fn)?;
    }

    // ── Network Info ──
    {
        let bridge = Arc::clone(&bridge);
        let net_fn = lua.create_function(move |lua, _: ()| {
            let b = bridge.lock().unwrap();
            let n = &b.network;
            let t = lua.create_table()?;
            t.set("connected", n.connected)?;
            t.set("connection_type", n.connection_type.as_str())?;
            t.set("ip_address", n.ip_address.as_str())?;
            t.set("signal_strength", n.signal_strength)?;
            t.set("wifi_ssid", n.wifi_ssid.clone())?;
            t.set("is_metered", n.is_metered)?;
            t.set("downlink_mbps", n.downlink_mbps)?;
            Ok(t)
        })?;
        device_table.set("getNetworkInfo", net_fn)?;
    }

    // ── Platform / OS detection ──
    {
        let plat_fn = lua.create_function(|lua, _: ()| {
            let t = lua.create_table()?;
            t.set("os", std::env::consts::OS)?;
            t.set("arch", std::env::consts::ARCH)?;
            t.set("family", std::env::consts::FAMILY)?;
            t.set("is_desktop", cfg!(not(any(target_os = "android", target_os = "ios"))))?;
            t.set("is_mobile", cfg!(any(target_os = "android", target_os = "ios")))?;
            let exe = std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|p| p.to_string_lossy().to_string()))
                .unwrap_or_default();
            t.set("exe_dir", exe)?;
            Ok(t)
        })?;
        device_table.set("getPlatform", plat_fn)?;
    }

    // ── Permissions ──
    {
        let bridge = Arc::clone(&bridge);
        let check_fn = lua.create_function(move |_, perm: String| {
            let b = bridge.lock().unwrap();
            let p: Permission = perm.parse()
                .map_err(mlua::Error::RuntimeError)?;
            let state = b.permissions.lock().unwrap().check_permission(p);
            Ok(perm_state_str(state).to_string())
        })?;
        device_table.set("checkPermission", check_fn)?;
    }
    {
        let bridge = Arc::clone(&bridge);
        let request_fn = lua.create_function(move |_, perm: String| {
            let b = bridge.lock().unwrap();
            let p: Permission = perm.parse()
                .map_err(mlua::Error::RuntimeError)?;
            let state = b.permissions.lock().unwrap().request_permission(p);
            Ok(perm_state_str(state).to_string())
        })?;
        device_table.set("requestPermission", request_fn)?;
    }
    {
        let perms_list_fn = lua.create_function(|lua, _: ()| {
            let perms = [
                "camera", "location", "storage", "notifications", "microphone",
                "contacts", "calendar", "reminders", "motion", "health",
                "mediaLibrary", "bluetooth", "wifi", "phoneState", "sms",
            ];
            let t = lua.create_table()?;
            for (i, p) in perms.iter().enumerate() {
                t.set(i + 1, *p)?;
            }
            Ok(t)
        })?;
        device_table.set("availablePermissions", perms_list_fn)?;
    }
    {
        let bridge = Arc::clone(&bridge);
        let settings_fn = lua.create_function(move |_, _: ()| {
            let b = bridge.lock().unwrap();
            let result = b.permissions.lock().unwrap().open_settings();
            Ok(result)
        })?;
        device_table.set("openSettings", settings_fn)?;
    }
    {
        let perm_info_fn = lua.create_function(move |lua, perm: String| {
            let p: Permission = perm.parse()
                .map_err(mlua::Error::RuntimeError)?;
            let t = lua.create_table()?;
            t.set("name", p.as_str())?;
            t.set("android_permission", p.android_permission())?;
            t.set("ios_plist_key", p.ios_info_plist_key())?;
            t.set("ios_framework", p.ios_framework_name())?;
            Ok(t)
        })?;
        device_table.set("getPermissionInfo", perm_info_fn)?;
    }

    // ── Geolocation ──
    {
        let bridge = Arc::clone(&bridge);
        let geo_fn = lua.create_function(move |lua, _: ()| {
            let b = bridge.lock().unwrap();
            let geo = &b.geo;
            let t = lua.create_table()?;
            t.set("latitude", geo.latitude)?;
            t.set("longitude", geo.longitude)?;
            t.set("accuracy", geo.accuracy)?;
            t.set("altitude", geo.altitude)?;
            Ok(t)
        })?;
        device_table.set("getLocation", geo_fn)?;
    }

    // ── Sensors ──
    {
        let bridge = Arc::clone(&bridge);
        let sens_fn = lua.create_function(move |lua, _: ()| {
            let b = bridge.lock().unwrap();
            let s = &b.sensors;
            let t = lua.create_table()?;
            let accel = s.accelerometer.map(|(x, y, z)| {
                let at = lua.create_table().unwrap();
                at.set("x", x).unwrap();
                at.set("y", y).unwrap();
                at.set("z", z).unwrap();
                at
            });
            t.set("accelerometer", accel)?;
            let gyro = s.gyroscope.map(|(x, y, z)| {
                let at = lua.create_table().unwrap();
                at.set("x", x).unwrap();
                at.set("y", y).unwrap();
                at.set("z", z).unwrap();
                at
            });
            t.set("gyroscope", gyro)?;
            let mag = s.magnetometer.map(|(x, y, z)| {
                let at = lua.create_table().unwrap();
                at.set("x", x).unwrap();
                at.set("y", y).unwrap();
                at.set("z", z).unwrap();
                at
            });
            t.set("magnetometer", mag)?;
            Ok(t)
        })?;
        device_table.set("getSensors", sens_fn)?;
    }

    // ── Storage ──
    {
        let bridge = Arc::clone(&bridge);
        let read_fn = lua.create_function(move |_, path: String| {
            let b = bridge.lock().unwrap();
            match b.storage.read(&path) {
                Ok(data) => Ok(Some(String::from_utf8_lossy(&data).to_string())),
                Err(_) => Ok(None::<String>),
            }
        })?;
        device_table.set("readFile", read_fn)?;
    }
    {
        let bridge = Arc::clone(&bridge);
        let write_fn = lua.create_function(move |_, (path, content): (String, String)| {
            let b = bridge.lock().unwrap();
            match b.storage.write(&path, content.as_bytes()) {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        })?;
        device_table.set("writeFile", write_fn)?;
    }
    {
        let bridge = Arc::clone(&bridge);
        let delete_fn = lua.create_function(move |_, path: String| {
            let b = bridge.lock().unwrap();
            match b.storage.delete(&path) {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        })?;
        device_table.set("deleteFile", delete_fn)?;
    }
    {
        let bridge = Arc::clone(&bridge);
        let exists_fn = lua.create_function(move |_, path: String| {
            let b = bridge.lock().unwrap();
            Ok(b.storage.exists(&path))
        })?;
        device_table.set("fileExists", exists_fn)?;
    }
    {
        let bridge = Arc::clone(&bridge);
        let base_fn = lua.create_function(move |_, _: ()| {
            let b = bridge.lock().unwrap();
            Ok(b.storage.base_path().to_string_lossy().to_string())
        })?;
        device_table.set("storagePath", base_fn)?;
    }
    {
        let bridge = Arc::clone(&bridge);
        let cache_fn = lua.create_function(move |_, _: ()| {
            let b = bridge.lock().unwrap();
            Ok(b.storage.cache_dir().to_string_lossy().to_string())
        })?;
        device_table.set("cacheDir", cache_fn)?;
    }
    {
        let bridge = Arc::clone(&bridge);
        let temp_fn = lua.create_function(move |_, _: ()| {
            let b = bridge.lock().unwrap();
            Ok(b.storage.temp_dir().to_string_lossy().to_string())
        })?;
        device_table.set("tempDir", temp_fn)?;
    }
    {
        let bridge = Arc::clone(&bridge);
        let docs_fn = lua.create_function(move |_, _: ()| {
            let b = bridge.lock().unwrap();
            Ok(b.storage.documents_dir().to_string_lossy().to_string())
        })?;
        device_table.set("documentsDir", docs_fn)?;
    }
    {
        let bridge = Arc::clone(&bridge);
        let list_fn = lua.create_function(move |_, path: String| {
            let b = bridge.lock().unwrap();
            let dir = b.storage.base_path().join(&path);
            match std::fs::read_dir(&dir) {
                Ok(entries) => {
                    let names: Vec<String> = entries
                        .filter_map(|e| e.ok())
                        .map(|e| e.file_name().to_string_lossy().to_string())
                        .collect();
                    Ok(Some(names))
                }
                Err(_) => Ok(None::<Vec<String>>),
            }
        })?;
        device_table.set("listFiles", list_fn)?;
    }

    // ── Clipboard ──
    {
        let set_fn = lua.create_function(|_, text: String| {
            Clipboard::set(&text);
            Ok(true)
        })?;
        device_table.set("setClipboard", set_fn)?;
    }
    {
        let get_fn = lua.create_function(|_, _: ()| {
            Ok(Clipboard::get())
        })?;
        device_table.set("getClipboard", get_fn)?;
    }
    {
        let has_fn = lua.create_function(|_, _: ()| {
            Ok(Clipboard::has_text())
        })?;
        device_table.set("hasClipboard", has_fn)?;
    }

    // ── Haptics ──
    {
        let vibrate_fn = lua.create_function(|_, duration_ms: u64| {
            Haptics::vibrate(duration_ms);
            Ok(true)
        })?;
        device_table.set("vibrate", vibrate_fn)?;
    }
    {
        let impact_fn = lua.create_function(|_, style: String| {
            let hf = match style.as_str() {
                "light" => HapticFeedback::Light,
                "medium" => HapticFeedback::Medium,
                "heavy" => HapticFeedback::Heavy,
                "selection" => HapticFeedback::Selection,
                "success" => HapticFeedback::Success,
                "warning" => HapticFeedback::Warning,
                "error" => HapticFeedback::Error,
                "click" => HapticFeedback::Click,
                _ => HapticFeedback::Medium,
            };
            Haptics::impact(hf);
            Ok(true)
        })?;
        device_table.set("hapticImpact", impact_fn)?;
    }
    {
        let supported_fn = lua.create_function(|_, _: ()| {
            Ok(Haptics::is_supported())
        })?;
        device_table.set("hapticSupported", supported_fn)?;
    }

    // ── Camera (stub) ──
    {
        let camera_fn = lua.create_function(|lua, _: ()| {
            let t = lua.create_table()?;
            t.set("uri", "camera://placeholder")?;
            t.set("width", 1920)?;
            t.set("height", 1080)?;
            Ok(t)
        })?;
        device_table.set("takePhoto", camera_fn)?;
    }

    // ── Notifications ──
    {
        let notif_fn = lua.create_function(|_, (title, body): (String, String)| {
            println!("[Notification] {}: {}", title, body);
            Ok(true)
        })?;
        device_table.set("showNotification", notif_fn)?;
    }
    {
        let schedule_fn = lua.create_function(|_, (title, body, delay_ms): (String, String, u64)| {
            println!("[Notification] Scheduled ({}ms): {}: {}", delay_ms, title, body);
            Ok(true)
        })?;
        device_table.set("scheduleNotification", schedule_fn)?;
    }

    // ── Dialogs (native) ──
    {
        let alert_fn = lua.create_function(|_, (title, message, buttons): (String, String, Vec<String>)| {
            println!("[Dialog] Alert '{}': {} — buttons: {:?}", title, message, buttons);
            Ok(0) // first button index
        })?;
        device_table.set("showAlert", alert_fn)?;
    }
    {
        let confirm_fn = lua.create_function(|_, (title, message): (String, String)| {
            println!("[Dialog] Confirm '{}': {}", title, message);
            Ok(true)
        })?;
        device_table.set("showConfirm", confirm_fn)?;
    }
    {
        let prompt_fn = lua.create_function(|_, (title, message, default_text): (String, String, String)| {
            println!("[Dialog] Prompt '{}': {} (default: {})", title, message, default_text);
            Ok(default_text)
        })?;
        device_table.set("showPrompt", prompt_fn)?;
    }

    // ── Screen ──
    {
        let keep_on_fn = lua.create_function(|_, keep: bool| {
            println!("[Screen] Keep awake: {}", keep);
            Ok(true)
        })?;
        device_table.set("keepScreenOn", keep_on_fn)?;
    }
    {
        let brightness_fn = lua.create_function(|_, level: f32| {
            println!("[Screen] Set brightness: {}", level);
            Ok(true)
        })?;
        device_table.set("setBrightness", brightness_fn)?;
    }

    lua.globals().set("Device", device_table)?;

    Ok(())
}
