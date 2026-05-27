use mlua::prelude::*;
use std::sync::{Arc, Mutex};

use lumina_core::device::{DeviceInfo, GeoLocation, SensorData, Storage};
use lumina_core::permissions::{DesktopPermissionHandler, Permission, PermissionHandler, PermissionState};

pub struct DeviceBridge {
    pub permissions: Arc<Mutex<dyn PermissionHandler + Send>>,
    pub storage: Storage,
    pub geo: GeoLocation,
    pub sensors: SensorData,
    pub info: DeviceInfo,
}

impl Default for DeviceBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceBridge {
    pub fn new() -> Self {
        Self {
            permissions: Arc::new(Mutex::new(DesktopPermissionHandler::new())),
            storage: Storage::new("lumina"),
            geo: GeoLocation::mock(),
            sensors: SensorData::current(),
            info: DeviceInfo::current(),
        }
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

    // ── Permissions ──
    {
        let bridge = Arc::clone(&bridge);
        let check_fn = lua.create_function(move |_, perm: String| {
            let b = bridge.lock().unwrap();
            let p: Permission = perm.parse()
                .map_err(mlua::Error::RuntimeError)?;
            let state = b.permissions.lock().unwrap().check_permission(p);
            Ok(match state {
                PermissionState::Granted => "granted",
                PermissionState::Denied => "denied",
                PermissionState::NotRequested => "not_requested",
                PermissionState::Restricted => "restricted",
                PermissionState::Limited => "limited",
            }
            .to_string())
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
            Ok(match state {
                PermissionState::Granted => "granted",
                PermissionState::Denied => "denied",
                PermissionState::NotRequested => "not_requested",
                PermissionState::Restricted => "restricted",
                PermissionState::Limited => "limited",
            }
            .to_string())
        })?;
        device_table.set("requestPermission", request_fn)?;
    }
    {
        let perms_list_fn = lua.create_function(|lua, _: ()| {
            let perms = ["camera", "location", "storage", "notifications", "microphone", "contacts"];
            let t = lua.create_table()?;
            for (i, p) in perms.iter().enumerate() {
                t.set(i + 1, *p)?;
            }
            Ok(t)
        })?;
        device_table.set("availablePermissions", perms_list_fn)?;
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

    // ── Camera (stub) ──
    {
        let camera_fn = lua.create_function(|lua, _: ()| {
            lua.create_table()
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

    lua.globals().set("Device", device_table)?;

    Ok(())
}
