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
use std::fmt;
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

impl AppMetadata {
    /// Create a new AppMetadata object
    ///
    /// # Examples
    ///
    /// ```
    /// # use kubos_app::registry::AppMetadata;
    /// let metadata = AppMetadata::new("my-app", "1.0", "Jane Doe <jane@doe.com>");
    /// ```
    pub fn new(name: &str, version: &str, author: &str) -> AppMetadata {
        AppMetadata {
            name: name.to_string(),
            version: version.to_string(),
            author: author.to_string()
        }
    }
}

/// The different RunLevels supported by KubOS applications
#[derive(Clone,Debug,Deserialize,Serialize,PartialEq)]
pub enum RunLevel {
    /// An application will start at system boot time, and is managed automatically by the
    /// Application Service
    OnBoot,
    /// An application will start when commanded through the `start_app` GraphQL mutation
    OnCommand
}

impl fmt::Display for RunLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RunLevel::OnBoot => write!(f, "OnBoot"),
            RunLevel::OnCommand => write!(f, "OnCommand")
        }
    }
}

/// Kubos App struct
#[derive(Clone,Debug,Deserialize,Serialize)]
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
#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct AppRegistryEntry {
    /// Whether or not this application is the active installation
    pub active: bool,
    /// The run level of the application
    pub run_level: RunLevel,
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
                        Err(_) => None
                    },
                    Err(_) => None
                }
            },
            Err(_) => None
        }
    }

    fn save(&self) -> Result<bool, String> {
        let mut app_toml = PathBuf::from(self.app.path.clone());
        app_toml.set_file_name("app.toml");

        match fs::File::create(app_toml) {
            Ok(mut file) => match toml::to_string(&self) {
                Ok(toml_str) => match file.write_all(&toml_str.into_bytes()) {
                    Ok(_) => Ok(true),
                    Err(err) => Err(format!("{}", err))
                },
                Err(err) => Err(format!("{}", err))
            },
            Err(err) => Err(format!("{}", err))
        }
    }
}

/// AppRegistry
#[derive(Deserialize,Serialize)]
pub struct AppRegistry {
    #[doc(hidden)]
    pub entries: RefCell<Vec<AppRegistryEntry>>,
    /// The managed root directory of the AppRegistry
    pub apps_dir: String
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
            apps_dir: String::from(apps_dir)
        };

        if !Path::new(apps_dir).is_dir() {
            return registry;
        }

        registry.entries.borrow_mut().extend(registry.discover_apps());
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
                                None => continue
                            };

                            if let Some(entry) = AppRegistryEntry::from_dir(version_path) {
                                reg_entries.push(entry);
                            }
                        }
                    },
                    Err(_) => continue
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

        let result = Command::new(path).args(&["--metadata"]).output();
        if result.is_err() {
            return Err(format!("Failed to get app metadata: {}", result.err().unwrap()));
        }

        let app_metadata = result.unwrap();
        if !app_metadata.status.success() {
            return Err("Bad exit code getting app metadata".to_string());
        }

        let metadata: AppMetadata =
            toml::from_slice(app_metadata.stdout.as_slice()).unwrap();

        let mut entries = self.entries.borrow_mut();
        let mut app_uuid: Uuid = Uuid::new_v4();
        for entry in entries.iter_mut() {
            if entry.active && entry.app.metadata.name == metadata.name {
                entry.active = false;
                app_uuid = Uuid::parse_str(&entry.app.uuid).unwrap();
                entry.save()?;
                break;
            }
        }

        let app_uuid_str = app_uuid.hyphenated().to_string();
        let apps_dir = Path::new(&self.apps_dir);
        if !apps_dir.exists() {
            match fs::create_dir(apps_dir) {
                Err(err) => { return Err(format!("Couldn't create apps dir {}: {:?}", apps_dir.display(), err)); },
                Ok(_) => {}
            }
        }

        if !apps_dir.exists() {
            return Err(format!("Couldn't create app dir: {}", apps_dir.display()));
        }

        let app_dir_path = format!("{}/{}/{}", self.apps_dir, app_uuid_str,
                                   metadata.version.as_str());
        let app_dir = Path::new(&app_dir_path);

        if !app_dir.exists() {
            match fs::create_dir_all(app_dir) {
                Err(err) => { return Err(format!("Couldn't create app dir {}: {:?}", app_dir.display(), err)); },
                Ok(_) => {}
            }
        }

        match app_path.file_name() {
            Some(app_filename) => {
                match fs::copy(path, app_dir.join(Path::new(app_filename))) {
                    Err(err) => { return Err(format!("Couldn't copy app binary: {:?}", err)); },
                    Ok(_) => {}
                }

                let active_dir = PathBuf::from(format!("{}/active", self.apps_dir));
                if !active_dir.exists() {
                    match fs::create_dir_all(active_dir.clone()) {
                        Err(err) => { return Err(format!("Couldn't create 'active' dir {}: {:?}", active_dir.display(), err)); },
                        Ok(_) => {}
                    }
                }

                let active_symlink = active_dir.join(app_uuid_str.clone());

                if active_symlink.exists() {
                    match fs::remove_file(active_symlink.clone()) {
                        Err(err) => { return Err(format!("Couldn't remove symlink {}: {:?}", active_symlink.display(), err)); },
                        Ok(_) => {}
                    }
                }

                match unix::fs::symlink(app_dir.to_str().expect("invalid app dir"),
                                        active_symlink.clone())
                {
                    Err(err) => { return Err(format!("Couldn't symlink {} to {}: {:?}", active_symlink.display(), app_dir.display(), err)); },
                    Ok(_) => {}
                }

                let reg_entry = AppRegistryEntry {
                    app: App {
                        uuid: app_uuid_str.to_string(),
                        metadata: metadata,
                        pid: 0,
                        path: app_dir.join(Path::new(app_filename)).to_str().expect("invalid app dir").to_string(),
                    },
                    active: true,
                    run_level: RunLevel::OnCommand
                };

                entries.push(reg_entry);
                entries[entries.len() - 1].save()?;
                Ok(entries[entries.len() - 1].clone())
            },
            None => {
                return Err(String::from("Couldn't get app filename"));
            }
        }
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
    pub fn uninstall(&self, app_uuid: &str, version: &str) -> Result<bool, String>
    {
        let mut entries = self.entries.borrow_mut();
        let app_index: usize;

        match entries.binary_search_by(
            |ref e| e.app.uuid.cmp(&String::from(app_uuid)).then(
                    e.app.metadata.version.cmp(&String::from(version))))
        {
            Ok(index) => {
                app_index = index;
            },
            Err(_) => {
                return Err(format!("Active app with UUID {} does not exist", app_uuid));
            }
        }

        let app_path = PathBuf::from(&entries[app_index].app.path);
        if !app_path.exists() {
            return Err(format!("{} does not exist", &entries[app_index].app.path));
        }

        match app_path.parent() {
            Some(parent) => {
                match fs::remove_dir_all(parent.clone()) {
                    Ok(_) => {
                        if app_index < entries.len() {
                            entries.remove(app_index);
                        }
                        Ok(true)
                    },
                    Err(err) => Err(format!("Error removing app directory: {}", err))
                }
            },
            None => Err(String::from("Error finding parent path of app"))
        }
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
    pub fn start_app(&self, app_uuid: &str, run_level: RunLevel) -> Result<u32, String>
    {
        let entries = self.entries.borrow();
        let app: &App;

        match entries.iter().find(|ref e| e.active && e.app.uuid == app_uuid) {
            Some(entry) => { app = &entry.app; },
            None => {
                return Err(format!("Active app with UUID {} does not exist", app_uuid));
            }
        }

        let app_path = PathBuf::from(&app.path);
        if !app_path.exists() {
            return Err(format!("{} does not exist", &app.path));
        }

        let child = Command::new(app_path)
                            .env("KUBOS_APP_UUID", app.uuid.clone())
                            .env("KUBOS_APP_RUN_LEVEL", format!("{}", run_level))
                            .spawn()
                            .expect("Failed to spawn app");

        Ok(child.id())
    }
}
