/*
 * Copyright (C) 2018 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::cell::RefCell;
use kubos_app::RunLevel;
use std::fs;
use std::io::{Read, Write};
use std::os::unix;
use std::path::{Path, PathBuf};
use std::process::Command;

use toml;
use uuid::Uuid;

/// The default application registry directory in KubOS
pub const K_APPS_DIR: &'static str = "/home/system/kubos/apps";

/// The high level metadata of an application
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppMetadata {
    /// A unique name for the application (usually the same as the name of the binary)
    pub name: String,
    /// The version of this application
    pub version: String,
    /// The author of the application
    pub author: String,
}

/// Kubos App struct
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct App {
    /// The generated UUID for the application
    pub uuid: String,
    /// The process ID of the application, if it's currently running (0 otherwise)
    pub pid: u32,
    /// The absolute path to the application binary
    pub path: String,
    /// The associated metadata of the application
    pub metadata: AppMetadata,
}

/// AppRegistryEntry
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppRegistryEntry {
    /// Whether or not this application is the active installation
    pub active_version: bool,
    /// The app itself
    pub app: App,
}

impl AppRegistryEntry {
    fn from_dir(dir: &str) -> Option<AppRegistryEntry> {
        let app_toml: PathBuf = [dir, "app.toml"].iter().collect();
        if !app_toml.exists() {
            return None;
        }

        match fs::File::open(app_toml) {
            Ok(mut f) => {
                let mut buffer = String::new();
                match f.read_to_string(&mut buffer) {
                    Ok(_) => match toml::from_str::<AppRegistryEntry>(&buffer) {
                        Ok(entry) => Some(entry),
                        Err(_) => None,
                    },
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }

    fn save(&self) -> Result<bool, String> {
        let mut app_toml = PathBuf::from(self.app.path.clone());
        app_toml.set_file_name("app.toml");

        match fs::File::create(app_toml) {
            Ok(mut file) => match toml::to_string(&self) {
                Ok(toml_str) => match file.write_all(&toml_str.into_bytes()) {
                    Ok(_) => Ok(true),
                    Err(err) => Err(format!("{}", err)),
                },
                Err(err) => Err(format!("{}", err)),
            },
            Err(err) => Err(format!("{}", err)),
        }
    }
}

/// AppRegistry
#[derive(Deserialize, Serialize)]
pub struct AppRegistry {
    #[doc(hidden)]
    pub entries: RefCell<Vec<AppRegistryEntry>>,
    /// The managed root directory of the AppRegistry
    pub apps_dir: String,
}

impl AppRegistry {
    /// Create a new AppRegistry located at a specific directory in the filesystem
    ///
    /// # Arguments
    ///
    /// * `apps_dir` - The root directory that applications are loaded from
    ///
    /// # Examples
    ///
    /// ```
    /// # use kubos_app::registry::AppRegistry;
    /// let registry = AppRegistry::new_from_dir("/my/apps");
    /// ```
    pub fn new_from_dir(apps_dir: &str) -> AppRegistry {
        let registry = AppRegistry {
            entries: RefCell::new(Vec::new()),
            apps_dir: String::from(apps_dir),
        };

        let apps_dir = Path::new(apps_dir);
        if !apps_dir.exists() {
            if let Err(err) = fs::create_dir(apps_dir) {
                eprintln!("Couldn't create apps dir {}: {:?}", apps_dir.display(), err);
                return registry;
            }
        }

        let active_dir = apps_dir.clone().join("active");
        if !active_dir.exists() {
            if let Err(err) = fs::create_dir_all(&active_dir) {
                eprintln!(
                    "Couldn't create 'active' dir {}: {:?}",
                    active_dir.display(),
                    err
                );
                return registry;
            }
        }

        registry
            .entries
            .borrow_mut()
            .extend(registry.discover_apps());
        return registry;
    }

    /// Create a new AppRegistry located at the default directory (see [`K_APPS_DIR`])
    ///
    /// [`K_APPS_DIR`]: constant.K_APPS_DIR.html
    /// # Examples
    ///
    /// ```
    /// # use kubos_app::registry::AppRegistry;
    /// let registry = AppRegistry::new();
    /// ```
    pub fn new() -> AppRegistry {
        Self::new_from_dir(K_APPS_DIR)
    }

    fn discover_apps(&self) -> Vec<AppRegistryEntry> {
        let mut reg_entries: Vec<AppRegistryEntry> = Vec::new();

        if let Ok(entries) = fs::read_dir(&self.apps_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_dir() && entry.file_name().to_str() != Some("active") {
                            reg_entries.extend(self.discover_versions(entry.path()));
                        }
                    }
                }
            }
        }

        reg_entries
    }

    fn discover_versions(&self, app_dir: PathBuf) -> Vec<AppRegistryEntry> {
        let mut reg_entries: Vec<AppRegistryEntry> = Vec::new();
        if let Ok(versions) = fs::read_dir(app_dir) {
            for version in versions {
                if version.is_err() {
                    continue;
                }

                let version = version.unwrap();
                match version.file_type() {
                    Ok(v_file_type) => {
                        if v_file_type.is_dir() {
                            let v_path = version.path();
                            let version_path = match v_path.to_str() {
                                Some(v) => v,
                                None => continue,
                            };

                            if let Some(entry) = AppRegistryEntry::from_dir(version_path) {
                                reg_entries.push(entry);
                            }
                        }
                    }
                    Err(_) => continue,
                }
            }
        }
        reg_entries
    }

    /// Register an application binary with the AppRegistry, extracting metadata and installing it
    /// into the proper folder structure under the AppRegistry directory.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to an application binary
    ///
    /// # Examples
    ///
    /// ```
    /// # use kubos_app::registry::AppRegistry;
    /// let registry = AppRegistry::new();
    /// registry.register("/home/kubos/my-app-bin");
    /// ```
    pub fn register(&self, path: &str) -> Result<AppRegistryEntry, String> {
        let app_path = Path::new(path);
        if !app_path.exists() {
            return Err(format!("{} does not exist", path));
        }

        let app_filename = match app_path.file_name() {
            Some(filename) => filename,
            None => return Err(String::from("Couldn't get app filename")),
        };

        let result = Command::new(path).args(&["--metadata"]).output();
        if result.is_err() {
            return Err(format!(
                "Failed to get app metadata: {}",
                result.err().unwrap()
            ));
        }

        let app_metadata = result.unwrap();
        if !app_metadata.status.success() {
            return Err("Bad exit code getting app metadata".to_string());
        }

        let metadata: AppMetadata = toml::from_slice(app_metadata.stdout.as_slice()).unwrap();

        let mut entries = self.entries.borrow_mut();
        let mut app_uuid = Uuid::new_v4().hyphenated().to_string();
        for entry in entries.iter_mut() {
            if entry.active_version && entry.app.metadata.name == metadata.name {
                entry.active_version = false;
                app_uuid = entry.app.uuid.clone();
                entry.save()?;
                break;
            }
        }

        let app_dir_str = format!(
            "{}/{}/{}",
            self.apps_dir,
            app_uuid,
            metadata.version.as_str()
        );
        let app_dir = Path::new(&app_dir_str);

        if !app_dir.exists() {
            match fs::create_dir_all(app_dir) {
                Err(err) => {
                    return Err(format!(
                        "Couldn't create app dir {}: {:?}",
                        app_dir.display(),
                        err
                    ));
                }
                Ok(_) => {}
            }
        }

        match fs::copy(path, app_dir.join(Path::new(app_filename))) {
            Err(err) => {
                return Err(format!("Couldn't copy app binary: {:?}", err));
            }
            Ok(_) => {}
        }

        let active_symlink = PathBuf::from(format!("{}/active/{}", self.apps_dir, app_uuid));
        if active_symlink.exists() {
            match fs::remove_file(active_symlink.clone()) {
                Err(err) => {
                    return Err(format!(
                        "Couldn't remove symlink {}: {:?}",
                        active_symlink.display(),
                        err
                    ));
                }
                Ok(_) => {}
            }
        }

        match unix::fs::symlink(&app_dir_str, active_symlink.clone()) {
            Err(err) => {
                return Err(format!(
                    "Couldn't symlink {} to {}: {:?}",
                    active_symlink.display(),
                    app_dir_str,
                    err
                ));
            }
            Ok(_) => {}
        }

        let reg_entry = AppRegistryEntry {
            app: App {
                uuid: app_uuid,
                metadata: metadata,
                pid: 0,
                path: format!("{}/{}", app_dir_str, app_filename.to_string_lossy()).to_owned(),
            },
            active_version: true,
        };

        entries.push(reg_entry);
        entries[entries.len() - 1].save()?;
        Ok(entries[entries.len() - 1].clone())
    }

    /// Uninstall an application from the AppRegistry
    ///
    /// # Arguments
    ///
    /// * `app_uuid` - The UUID generated for the app
    /// * `version` - The version of the app to delete
    ///
    /// # Examples
    ///
    /// ```
    /// # use kubos_app::registry::AppRegistry;
    /// let registry = AppRegistry::new();
    /// registry.uninstall("01234567-89ab-cdef0-1234-56789abcdef0", "1.0");
    /// ```
    ///
    pub fn uninstall(&self, app_uuid: &str, version: &str) -> Result<bool, String> {
        let mut entries = self.entries.borrow_mut();
        let app_index = match entries.binary_search_by(|ref e| {
            e.app
                .uuid
                .cmp(&String::from(app_uuid))
                .then(e.app.metadata.version.cmp(&String::from(version)))
        }) {
            Ok(index) => index,
            Err(_) => return Err(format!("Active app with UUID {} does not exist", app_uuid)),
        };

        let app_path = PathBuf::from(&entries[app_index].app.path);
        if app_path.exists() {
            let parent = match app_path.parent() {
                Some(parent) => parent,
                // This should never happen
                None => return Err(String::from("Error finding parent path of app")),
            };

            if let Err(err) = fs::remove_dir_all(parent.clone()) {
                return Err(format!("Error removing app directory: {}", err));
            }
        }

        if app_index < entries.len() {
            entries.remove(app_index);
        }

        Ok(true)
    }

    /// Start an application. If successful, returns the pid of the application process.
    ///
    /// # Arguments
    ///
    /// * `app_uuid` - The UUID generated for the app when it was registered
    /// * `run_level` - Which Run Level to run the app with
    ///
    /// # Examples
    ///
    /// ```
    /// # use kubos_app::registry::{AppRegistry, RunLevel};
    /// let registry = AppRegistry::new();
    /// registry.start_app("01234567-89ab-cdef0-1234-56789abcdef0", RunLevel::OnCommand);
    /// ```
    pub fn start_app(&self, app_uuid: &str, run_level: RunLevel) -> Result<u32, String> {
        let entries = self.entries.borrow();

        let app = match entries
            .iter()
            .find(|ref e| e.active_version && e.app.uuid == app_uuid)
        {
            Some(entry) => &entry.app,
            None => return Err(format!("Active app with UUID {} does not exist", app_uuid)),
        };

        let app_path = PathBuf::from(&app.path);
        if !app_path.exists() {
            return Err(format!("{} does not exist", &app.path));
        }

        match Command::new(app_path)
            .env("KUBOS_APP_UUID", app.uuid.clone())
            .env("KUBOS_APP_RUN_LEVEL", format!("{}", run_level))
            .spawn()
        {
            Ok(child) => Ok(child.id()),
            Err(err) => Err(format!("Failed to spawn app: {:?}", err)),
        }
    }
}
