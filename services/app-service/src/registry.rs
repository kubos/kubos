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
use kubos_app::RunLevel;
use std::cell::RefCell;
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

        if !app_path.is_dir() {
            return Err(format!("{} is not a directory", path));
        }

        let files: Vec<fs::DirEntry> = match fs::read_dir(app_path) {
            Ok(v) => v.filter_map(|file| file.ok()).collect(),
            Err(error) => return Err(format!("Failed to read directory: {}", error)),
        };

        if files.len() != 2 {
            return Err("Exactly two files should be present in the app directory".to_owned());
        }

        let mut manifest_file: Option<fs::DirEntry> = None;
        let mut app_file: Option<fs::DirEntry> = None;

        for file in files {
            match file.file_name().to_str() {
                Some("manifest.toml") => manifest_file = Some(file),
                Some(_) => app_file = Some(file),
                _ => {}
            }
        }

        let manifest = match manifest_file {
            Some(file) => file,
            None => return Err("Failed to find manifest file".to_owned()),
        };
        let app = match app_file {
            Some(file) => file,
            None => return Err("Failed to find app file".to_owned()),
        };

        let mut data = String::new();
        fs::File::open(manifest.path())
            .and_then(|mut fp| fp.read_to_string(&mut data))
            .or_else(|error| return Err(format!("Failed to read manifest: {}", error)))?;

        let metadata: AppMetadata = toml::from_str(&data)
            .or_else(|error| return Err(format!("Failed to parse manifest: {}", error)))?;

        let mut entries = self.entries.borrow_mut();
        let mut app_uuid = Uuid::new_v4().hyphenated().to_string();
        // TODO: Do the lookup based on the passed UUID
        // Also TODO: Allow a UUID to be passed...
        for entry in entries.iter_mut() {
            // Find the existing active version of the app and make it inactive.
            // Use the existing UUID for our new app
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
            fs::create_dir_all(app_dir).or_else(|err| {
                return Err(format!(
                    "Couldn't create app dir {}: {:?}",
                    app_dir.display(),
                    err
                ));
            })?;
        }

        match fs::copy(app.path(), app_dir.join(app.file_name())) {
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
                path: format!("{}/{}", app_dir_str, app.file_name().to_string_lossy()),
            },
            active_version: true,
        };

        // Add the new registry entry
        entries.push(reg_entry);
        // Create the app.toml file and save the metadata information
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
            // TODO: Unregister app if path doesn't exist
            return Err(format!("{} does not exist", &app.path));
        }

        match Command::new(app_path)
            .env("KUBOS_APP_UUID", app.uuid.clone())
            .arg("-r")
            .arg(format!("{}", run_level))
            .spawn()
        {
            Ok(child) => Ok(child.id()),
            Err(err) => Err(format!("Failed to spawn app: {:?}", err)),
        }
    }

    pub fn run_onboot(&self) -> Result<(), String> {
        // TODO: Decide whether or not we actually want to track started/failed apps
        let mut apps_started = 0;
        let mut apps_not_started = 0;

        let active_symlink = PathBuf::from(format!("{}/active", self.apps_dir));
        if !active_symlink.exists() {
            // TODO: Is this the way we want to handle this?
            return Err(format!("Failed to get list of active UUIDs"));
        }

        for entry in fs::read_dir(active_symlink)
            .or_else(|error| return Err(format!("Failed to process existing apps: {}", error)))?
        {
            match entry {
                Ok(file) => {
                    let uuid = file.file_name();
                    match self.start_app(&uuid.to_string_lossy(), RunLevel::OnBoot) {
                        Ok(_) => apps_started += 1,
                        Err(_) => apps_not_started += 1,
                    }
                }
                Err(_) => apps_not_started += 1,
            }
        }

        // TODO: Remove me. Debug line.
        println!(
            "Apps started: {}, Apps failed: {}",
            apps_started, apps_not_started
        );

        if apps_not_started != 0 {
            return Err(format!("Failed to start {} app/s", apps_not_started));
        }

        Ok(())
    }
}
