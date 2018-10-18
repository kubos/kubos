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

use app_entry::*;
use failure::Error;
use kubos_app::RunLevel;
use std::cell::RefCell;
use std::fs;
use std::io::Read;
use std::os::unix;
use std::path::{Path, PathBuf};
use std::process::Command;
use toml;
use uuid::Uuid;

/// The default application registry directory in KubOS
pub const K_APPS_DIR: &'static str = "/home/system/kubos/apps";

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
    pub fn new_from_dir(apps_dir: &str) -> Result<AppRegistry, Error> {
        let registry = AppRegistry {
            entries: RefCell::new(Vec::new()),
            apps_dir: String::from(apps_dir),
        };

        let active_dir = PathBuf::from(format!("{}/active", apps_dir));
        if !active_dir.exists() {
            fs::create_dir_all(&active_dir)?;
        }

        registry
            .entries
            .borrow_mut()
            .extend(registry.discover_apps()?);

        Ok(registry)
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
    pub fn new() -> Result<AppRegistry, Error> {
        Self::new_from_dir(K_APPS_DIR)
    }

    fn discover_apps(&self) -> Result<Vec<AppRegistryEntry>, Error> {
        let mut reg_entries: Vec<AppRegistryEntry> = Vec::new();

        for entry in fs::read_dir(&self.apps_dir)? {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() && entry.file_name().to_str() != Some("active") {
                        reg_entries.extend(self.discover_versions(entry.path())?);
                    }
                }
            }
            //TODO: Remove bad entries
        }

        Ok(reg_entries)
    }

    fn discover_versions(&self, app_dir: PathBuf) -> Result<Vec<AppRegistryEntry>, Error> {
        let mut reg_entries: Vec<AppRegistryEntry> = Vec::new();

        for version in fs::read_dir(app_dir)? {
            if version.is_err() {
                // TODO: Remove bad directory
                continue;
            }

            let version = version.unwrap();
            match version
                .file_type()
                .and_then(|file_type| Ok(file_type.is_dir()))
            {
                Ok(true) => {
                    if let Ok(entry) = AppRegistryEntry::from_dir(&version.path()) {
                        reg_entries.push(entry);
                    } else {
                        // TODO: Remove bad directory
                    }
                }
                _ => {
                    // TODO: Remove bad directory
                    continue;
                }
            }
        }

        Ok(reg_entries)
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
    pub fn register(&self, path: &str) -> Result<AppRegistryEntry, Error> {
        let app_path = Path::new(path);
        if !app_path.exists() {
            bail!("{} does not exist", path);
        }

        if !app_path.is_dir() {
            bail!("{} is not a directory", path);
        }

        let files: Vec<fs::DirEntry> = fs::read_dir(app_path)?
            .filter_map(|file| file.ok())
            .collect();

        if files.len() != 2 {
            bail!("Exactly two files should be present in the app directory");
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
            None => bail!("Failed to find manifest file"),
        };
        let app = match app_file {
            Some(file) => file,
            None => bail!("Failed to find app file"),
        };

        let mut data = String::new();
        fs::File::open(manifest.path()).and_then(|mut fp| fp.read_to_string(&mut data))?;

        let metadata: AppMetadata = toml::from_str(&data)?;

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
            fs::create_dir_all(app_dir)?;
        }

        fs::copy(app.path(), app_dir.join(app.file_name()))?;

        let active_symlink = PathBuf::from(format!("{}/active/{}", self.apps_dir, app_uuid));
        if active_symlink.exists() {
            match fs::remove_file(active_symlink.clone()) {
                Err(err) => {
                    bail!(
                        "Couldn't remove symlink {}: {:?}",
                        active_symlink.display(),
                        err
                    );
                }
                Ok(_) => {}
            }
        }

        match unix::fs::symlink(&app_dir_str, active_symlink.clone()) {
            Err(err) => {
                bail!(
                    "Couldn't symlink {} to {}: {:?}",
                    active_symlink.display(),
                    app_dir_str,
                    err
                );
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
    pub fn uninstall(&self, app_uuid: &str, version: &str) -> Result<bool, Error> {
        let mut entries = self.entries.borrow_mut();
        let app_index = match entries.binary_search_by(|ref e| {
            e.app
                .uuid
                .cmp(&String::from(app_uuid))
                .then(e.app.metadata.version.cmp(&String::from(version)))
        }) {
            Ok(index) => index,
            Err(_) => bail!("Active app with UUID {} does not exist", app_uuid),
        };

        let app_path = PathBuf::from(&entries[app_index].app.path);
        if app_path.exists() {
            let parent = match app_path.parent() {
                Some(parent) => parent,
                // This should never happen
                None => bail!("Error finding parent path of app"),
            };

            if let Err(err) = fs::remove_dir_all(parent.clone()) {
                bail!("Error removing app directory: {}", err);
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
    pub fn start_app(
        &self,
        app_uuid: &str,
        run_level: RunLevel,
        args: Option<Vec<String>>,
    ) -> Result<u32, Error> {
        let entries = self.entries.borrow();

        let app = match entries
            .iter()
            .find(|ref e| e.active_version && e.app.uuid == app_uuid)
        {
            Some(entry) => &entry.app,
            None => bail!("Active app with UUID {} does not exist", app_uuid),
        };

        let app_path = PathBuf::from(&app.path);
        if !app_path.exists() {
            // TODO: Unregister app if path doesn't exist
            bail!("{} does not exist", &app.path);
        }

        let mut cmd = Command::new(app_path);

        cmd.env("KUBOS_APP_UUID", app.uuid.clone())
            .arg("-r")
            .arg(format!("{}", run_level));

        if let Some(add_args) = args {
            cmd.args(&add_args);
        }

        match cmd.spawn() {
            Ok(child) => Ok(child.id()),
            Err(err) => bail!("Failed to spawn app: {:?}", err),
        }
    }

    /// Call the active version of all registered applications with the "OnBoot" run level
    ///
    /// # Examples
    ///
    /// ```
    /// # use kubos_app::registry::{AppRegistry, RunLevel};
    /// let registry = AppRegistry::new();
    /// registry.run_onboot();
    /// ```
    pub fn run_onboot(&self) -> Result<(), Error> {
        // TODO: Decide whether or not we actually want to track started/failed apps
        let mut apps_started = 0;
        let mut apps_not_started = 0;

        let active_symlink = PathBuf::from(format!("{}/active", self.apps_dir));
        if !active_symlink.exists() {
            bail!("Failed to get list of active UUIDs");
        }

        for entry in fs::read_dir(active_symlink)
            .or_else(|error| bail!("Failed to process existing apps: {}", error))?
        {
            match entry {
                Ok(file) => {
                    let uuid = file.file_name();
                    match self.start_app(&uuid.to_string_lossy(), RunLevel::OnBoot, None) {
                        Ok(_) => apps_started += 1,
                        Err(_) => apps_not_started += 1,
                    }
                }
                Err(_) => apps_not_started += 1,
            }
        }

        // QUESTION: Keep this or not? It's kind of a nice informational message
        println!(
            "Apps started: {}, Apps failed: {}",
            apps_started, apps_not_started
        );

        if apps_not_started != 0 {
            bail!("Failed to start {} app/s", apps_not_started);
        }

        Ok(())
    }
}
