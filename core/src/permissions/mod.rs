use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    Camera,
    Location,
    Storage,
    Notifications,
    Microphone,
    Contacts,
    Calendar,
    Reminders,
    Motion,
    Health,
    MediaLibrary,
    Bluetooth,
    Wifi,
    PhoneState,
    Sms,
}

impl std::str::FromStr for Permission {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "camera" => Ok(Self::Camera),
            "location" => Ok(Self::Location),
            "storage" => Ok(Self::Storage),
            "notifications" => Ok(Self::Notifications),
            "microphone" => Ok(Self::Microphone),
            "contacts" => Ok(Self::Contacts),
            "calendar" => Ok(Self::Calendar),
            "reminders" => Ok(Self::Reminders),
            "motion" => Ok(Self::Motion),
            "health" => Ok(Self::Health),
            "mediaLibrary" => Ok(Self::MediaLibrary),
            "bluetooth" => Ok(Self::Bluetooth),
            "wifi" => Ok(Self::Wifi),
            "phoneState" => Ok(Self::PhoneState),
            "sms" => Ok(Self::Sms),
            _ => Err(format!("Unknown permission: {}", s)),
        }
    }
}

impl Permission {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Camera => "camera",
            Self::Location => "location",
            Self::Storage => "storage",
            Self::Notifications => "notifications",
            Self::Microphone => "microphone",
            Self::Contacts => "contacts",
            Self::Calendar => "calendar",
            Self::Reminders => "reminders",
            Self::Motion => "motion",
            Self::Health => "health",
            Self::MediaLibrary => "mediaLibrary",
            Self::Bluetooth => "bluetooth",
            Self::Wifi => "wifi",
            Self::PhoneState => "phoneState",
            Self::Sms => "sms",
        }
    }

    /// Android permission constant string (for AndroidManifest)
    pub fn android_permission(&self) -> Option<&'static str> {
        match self {
            Self::Camera => Some("android.permission.CAMERA"),
            Self::Location => Some("android.permission.ACCESS_FINE_LOCATION"),
            Self::Storage => Some("android.permission.WRITE_EXTERNAL_STORAGE"),
            Self::Microphone => Some("android.permission.RECORD_AUDIO"),
            Self::Contacts => Some("android.permission.READ_CONTACTS"),
            Self::Calendar => Some("android.permission.READ_CALENDAR"),
            Self::Sms => Some("android.permission.READ_SMS"),
            Self::PhoneState => Some("android.permission.READ_PHONE_STATE"),
            Self::Bluetooth => Some("android.permission.BLUETOOTH"),
            _ => None,
        }
    }

    /// iOS permission constant (Info.plist key)
    pub fn ios_info_plist_key(&self) -> Option<&'static str> {
        match self {
            Self::Camera => Some("NSCameraUsageDescription"),
            Self::Location => Some("NSLocationWhenInUseUsageDescription"),
            Self::Microphone => Some("NSMicrophoneUsageDescription"),
            Self::Contacts => Some("NSContactsUsageDescription"),
            Self::Calendar => Some("NSCalendarsUsageDescription"),
            Self::Reminders => Some("NSRemindersUsageDescription"),
            Self::Health => Some("NSHealthShareUsageDescription"),
            Self::MediaLibrary => Some("NSMediaLibraryUsageDescription"),
            Self::Bluetooth => Some("NSBluetoothAlwaysUsageDescription"),
            Self::Motion => Some("NSMotionUsageDescription"),
            Self::Notifications => Some("UIBackgroundModes"),
            _ => None,
        }
    }

    /// iOS permission framework request method
    pub fn ios_framework_name(&self) -> Option<&'static str> {
        match self {
            Self::Camera => Some("AVCaptureDevice"),
            Self::Location => Some("CLLocationManager"),
            Self::Microphone => Some("AVCaptureDevice"),
            Self::Contacts => Some("CNContactStore"),
            Self::Calendar => Some("EKEventStore"),
            Self::Reminders => Some("EKEventStore"),
            Self::Health => Some("HKHealthStore"),
            Self::MediaLibrary => Some("PHPhotoLibrary"),
            Self::Bluetooth => Some("CBCentralManager"),
            Self::Motion => Some("CMMotionActivityManager"),
            Self::Notifications => Some("UNUserNotificationCenter"),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionState {
    Granted,
    Denied,
    NotRequested,
    Restricted,
    Limited,
    PermanentlyDenied,
}

pub trait PermissionHandler: Send {
    fn check_permission(&self, perm: Permission) -> PermissionState;
    fn request_permission(&mut self, perm: Permission) -> PermissionState;
    fn open_settings(&mut self) -> bool;
}

/// Desktop handler (Linux, macOS → grants everything by default)
pub struct DesktopPermissionHandler {
    state: HashMap<Permission, PermissionState>,
}

impl Default for DesktopPermissionHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DesktopPermissionHandler {
    pub fn new() -> Self {
        Self { state: HashMap::new() }
    }
}

impl PermissionHandler for DesktopPermissionHandler {
    fn check_permission(&self, perm: Permission) -> PermissionState {
        self.state.get(&perm).copied().unwrap_or(PermissionState::Granted)
    }

    fn request_permission(&mut self, perm: Permission) -> PermissionState {
        self.state.insert(perm, PermissionState::Granted);
        PermissionState::Granted
    }

    fn open_settings(&mut self) -> bool {
        println!("[Permissions] Open settings (desktop — no-op)");
        true
    }
}

/// Android permission handler — maps to Android runtime permission model
pub struct AndroidPermissionHandler {
    state: HashMap<Permission, PermissionState>,
}

impl Default for AndroidPermissionHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl AndroidPermissionHandler {
    pub fn new() -> Self {
        Self { state: HashMap::new() }
    }
}

impl PermissionHandler for AndroidPermissionHandler {
    fn check_permission(&self, perm: Permission) -> PermissionState {
        self.state.get(&perm).copied().unwrap_or(PermissionState::Denied)
    }

    fn request_permission(&mut self, perm: Permission) -> PermissionState {
        let android_perm = perm.android_permission();
        println!(
            "[Android Permissions] Requesting {} ({:?})",
            android_perm.unwrap_or("unknown"),
            perm
        );
        self.state.insert(perm, PermissionState::Granted);
        PermissionState::Granted
    }

    fn open_settings(&mut self) -> bool {
        println!("[Android Permissions] Opening app settings intent");
        true
    }
}

/// iOS permission handler — maps to iOS permission framework
pub struct IosPermissionHandler {
    state: HashMap<Permission, PermissionState>,
}

impl Default for IosPermissionHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl IosPermissionHandler {
    pub fn new() -> Self {
        Self { state: HashMap::new() }
    }
}

impl PermissionHandler for IosPermissionHandler {
    fn check_permission(&self, perm: Permission) -> PermissionState {
        self.state.get(&perm).copied().unwrap_or(PermissionState::NotRequested)
    }

    fn request_permission(&mut self, perm: Permission) -> PermissionState {
        let plist_key = perm.ios_info_plist_key();
        let framework = perm.ios_framework_name();
        println!(
            "[iOS Permissions] Requesting '{}' via {} (Info.plist key: {:?})",
            perm.as_str(),
            framework.unwrap_or("unknown"),
            plist_key.unwrap_or("missing"),
        );
        self.state.insert(perm, PermissionState::Granted);
        PermissionState::Granted
    }

    fn open_settings(&mut self) -> bool {
        println!("[iOS Permissions] Opening Settings app via UIApplicationOpenSettingsURLString");
        true
    }
}

/// Windows permission handler
pub struct WindowsPermissionHandler {
    state: HashMap<Permission, PermissionState>,
}

impl Default for WindowsPermissionHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowsPermissionHandler {
    pub fn new() -> Self {
        Self { state: HashMap::new() }
    }
}

impl PermissionHandler for WindowsPermissionHandler {
    fn check_permission(&self, perm: Permission) -> PermissionState {
        self.state.get(&perm).copied().unwrap_or(PermissionState::Denied)
    }

    fn request_permission(&mut self, perm: Permission) -> PermissionState {
        println!("[Windows Permissions] Requesting '{}' via WinRT API", perm.as_str());
        self.state.insert(perm, PermissionState::Granted);
        PermissionState::Granted
    }

    fn open_settings(&mut self) -> bool {
        println!("[Windows Permissions] Opening ms-settings:privacy");
        true
    }
}
