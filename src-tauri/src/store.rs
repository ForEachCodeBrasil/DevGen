use crate::generators::AppPreferences;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

pub struct Store {
    pub preferences: Mutex<AppPreferences>,
    file_path: Option<PathBuf>,
}

impl Store {
    pub fn new(app: &AppHandle) -> Self {
        let file_path = app
            .path()
            .app_data_dir()
            .map(|d| d.join("preferences.json"))
            .ok();

        let preferences = if let Some(path) = &file_path {
            if path.exists() {
                fs::read_to_string(path)
                    .ok()
                    .and_then(|content| serde_json::from_str(&content).ok())
                    .unwrap_or_default()
            } else {
                AppPreferences::default()
            }
        } else {
            AppPreferences::default()
        };

        Self {
            preferences: Mutex::new(preferences),
            file_path,
        }
    }

    pub fn save(&self) -> Result<(), String> {
        if let Some(path) = &self.file_path {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            let prefs = self.preferences.lock().map_err(|e| e.to_string())?;
            let content = serde_json::to_string_pretty(&*prefs).map_err(|e| e.to_string())?;
            fs::write(path, content).map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("Failed to resolve app data directory".to_string())
        }
    }

    pub fn get(&self) -> Result<AppPreferences, String> {
        let prefs = self.preferences.lock().map_err(|e| e.to_string())?;
        Ok(prefs.clone())
    }

    pub fn update<F>(&self, f: F) -> Result<(), String>
    where
        F: FnOnce(&mut AppPreferences),
    {
        {
            let mut prefs = self.preferences.lock().map_err(|e| e.to_string())?;
            f(&mut prefs);
        }
        self.save()
    }
}
