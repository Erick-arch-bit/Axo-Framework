use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    Camera,
    Location,
    Storage,
    Notifications,
    Microphone,
    Contacts,
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
        }
    }
}

impl Default for DesktopPermissionHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionState {
    Granted,
    Denied,
    NotRequested,
    Restricted,
    Limited,
}

pub trait PermissionHandler: Send {
    fn check_permission(&self, perm: Permission) -> PermissionState;
    fn request_permission(&mut self, perm: Permission) -> PermissionState;
}

pub struct DesktopPermissionHandler {
    state: HashMap<Permission, PermissionState>,
}

impl DesktopPermissionHandler {
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
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
}
