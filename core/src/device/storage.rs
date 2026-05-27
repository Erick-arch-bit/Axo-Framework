use std::path::PathBuf;

pub enum StorageError {
    IoError(String),
    PermissionDenied,
}

pub struct Storage {
    base_path: PathBuf,
}

impl Storage {
    pub fn new(app_name: &str) -> Self {
        let base = dirs_next::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(app_name);
        let _ = std::fs::create_dir_all(&base);
        Self { base_path: base }
    }

    pub fn read(&self, path: &str) -> Result<Vec<u8>, StorageError> {
        let full = self.base_path.join(path);
        std::fs::read(&full).map_err(|e| StorageError::IoError(e.to_string()))
    }

    pub fn write(&self, path: &str, data: &[u8]) -> Result<(), StorageError> {
        let full = self.base_path.join(path);
        if let Some(parent) = full.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        std::fs::write(&full, data).map_err(|e| StorageError::IoError(e.to_string()))
    }

    pub fn delete(&self, path: &str) -> Result<(), StorageError> {
        let full = self.base_path.join(path);
        std::fs::remove_file(&full).map_err(|e| StorageError::IoError(e.to_string()))
    }

    pub fn exists(&self, path: &str) -> bool {
        self.base_path.join(path).exists()
    }

    pub fn cache_dir(&self) -> PathBuf {
        self.base_path.join("cache")
    }

    pub fn documents_dir(&self) -> PathBuf {
        self.base_path.join("documents")
    }

    pub fn temp_dir(&self) -> PathBuf {
        std::env::temp_dir()
    }

    pub fn base_path(&self) -> &PathBuf {
        &self.base_path
    }
}
