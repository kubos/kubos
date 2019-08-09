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

use crate::app_entry::*;
use crate::error::*;
use crate::monitor::*;
use chrono::Utc;
use failure::format_err;
use fs_extra;
use kubos_app::RunLevel;
use log::*;
use std::fs;
use std::io::Read;
use std::os::unix;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use toml;

/// The default application registry directory in KubOS
pub static K_APPS_DIR: &str = "/home/system/kubos/apps";
pub static DEFAULT_CONFIG: &str = "/home/system/etc/config.toml";

/// AppRegistry
#[derive(Clone, Debug)]
pub struct AppRegistry {
    #[doc(hidden)]
    pub entries: Arc<Mutex<Vec<AppRegistryEntry>>>,
    pub monitoring: Arc<Mutex<Vec<MonitorEntry>>>,
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
    pub fn new_from_dir(apps_dir: &str) -> Result<AppRegistry, AppError> {
        let registry = AppRegistry {
            entries: Arc::new(Mutex::new(Vec::new())),
            monitoring: Arc::new(Mutex::new(Vec::new())),
            apps_dir: String::from(apps_dir),
        };

        let active_dir = PathBuf::from(format!("{}/active", apps_dir));
        if !active_dir.exists() {
            fs::create_dir_all(&active_dir)?;
        }

        registry
            .entries
            .lock()
            .map_err(|err| AppError::RegistryError {
                err: format!("Couldn't get entries mutex: {:?}", err),
            })?
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
    pub fn new() -> Result<AppRegistry, AppError> {
        Self::new_from_dir(K_APPS_DIR)
    }

    fn discover_apps(&self) -> Result<Vec<AppRegistryEntry>, AppError> {
        let mut reg_entries: Vec<AppRegistryEntry> = Vec::new();

        for entry in fs::read_dir(&self.apps_dir)? {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() && entry.file_name().to_str() != Some("active") {
                        reg_entries.extend(self.discover_versions(entry.path())?);
                    }
                }
            }
        }

        Ok(reg_entries)
    }

    fn discover_versions(&self, app_dir: PathBuf) -> Result<Vec<AppRegistryEntry>, AppError> {
        let mut reg_entries: Vec<AppRegistryEntry> = Vec::new();

        for version in fs::read_dir(app_dir)? {
            if version.is_err() {
                continue;
            }

            let version = version.unwrap();

            match version
                .file_type()
                .and_then(|file_type| Ok(file_type.is_dir()))
            {
                Ok(true) => {
                    if let Ok(entry) = AppRegistryEntry::from_dir(&version.path()) {
                        if entry.active_version {
                            self.set_active(&entry.app.name, &version.path().to_string_lossy())?;
                        }
                        reg_entries.push(entry);
                    } else {
                        // Don't really care if this fails, since this is just trying to help
                        // prevent orphan files
                        let _ = fs::remove_dir_all(version.path());
                    }
                }
                _ => {
                    // Don't really care if this fails, since this is just trying to help
                    // prevent orphan files
                    let _ = fs::remove_dir_all(version.path());
                }
            }
        }

        Ok(reg_entries)
    }

    // Create or update the active version symlink for an application
    fn set_active(&self, name: &str, app_dir: &str) -> Result<(), AppError> {
        let active_symlink = PathBuf::from(format!("{}/active/{}", self.apps_dir, name));
        if active_symlink.exists() {
            if let Err(err) = fs::remove_file(active_symlink.clone()) {
                return Err(AppError::RegisterError {
                    err: format!(
                        "Couldn't remove symlink {}: {:?}",
                        active_symlink.display(),
                        err
                    ),
                });
            }
        }

        if let Err(err) = unix::fs::symlink(app_dir, active_symlink.clone()) {
            // Make sure the 'active' directory exists
            // If it doesn't, we'll go ahead and recreate it and try again
            let active_dir = PathBuf::from(format!("{}/active", self.apps_dir));
            if !active_dir.exists() {
                fs::create_dir_all(&active_dir)?;

                if unix::fs::symlink(app_dir, active_symlink.clone()).is_ok() {
                    return Ok(());
                }
            }
            return Err(AppError::RegisterError {
                err: format!(
                    "Couldn't symlink {} to {}: {:?}",
                    active_symlink.display(),
                    app_dir,
                    err
                ),
            });
        }

        Ok(())
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
    pub fn register(&self, path: &str) -> Result<AppRegistryEntry, AppError> {
        let app_path = Path::new(path);
        if !app_path.exists() {
            return Err(AppError::RegisterError {
                err: format!("{} does not exist", path),
            });
        }

        if !app_path.is_dir() {
            return Err(AppError::RegisterError {
                err: format!("{} is not a directory", path),
            });
        }

        // Load the metadata
        let mut data = String::new();
        fs::File::open(app_path.join("manifest.toml"))
            .and_then(|mut fp| fp.read_to_string(&mut data))?;

        let metadata: AppMetadata = match toml::from_str(&data) {
            Ok(val) => val,
            Err(error) => {
                return Err(AppError::ParseError {
                    entity: "manifest.toml".to_owned(),
                    err: error.to_string(),
                });
            }
        };

        let app_name = metadata.name.clone();

        // If the file which should be called for execution wasn't explicitly defined, use the
        // application name instead.
        let app_exec = if let Some(path) = metadata.executable.clone() {
            path
        } else {
            metadata.name.clone()
        };

        // Make sure the file which should be called for execution is present in the directory
        if !app_path.join(app_exec.clone()).exists() {
            return Err(AppError::RegisterError {
                err: format!("Application file {} not found in given path", app_exec),
            });
        }

        // Check for a custom configuration file
        let config = if let Some(path) = metadata.config {
            path
        } else {
            DEFAULT_CONFIG.to_owned()
        };

        let mut entries = self.entries.lock().map_err(|err| AppError::RegisterError {
            err: format!("Couldn't get entries mutex: {:?}", err),
        })?;

        // If the app has been registered before, get the index of the current active version.
        // We'll use this information later
        let old_active = if Path::new(&format!("{}/{}", self.apps_dir, app_name)).exists() {
            entries
                .iter()
                .position(|ref e| e.active_version && e.app.name == app_name)
        } else {
            None
        };

        // Set up the directory for this new version of the app
        let app_dir_str = format!("{}/{}/{}", self.apps_dir, app_name, metadata.version);
        let app_dir = Path::new(&app_dir_str);

        if app_dir.exists() {
            return Err(AppError::RegisterError {
                err: format!(
                    "App {} version {} already exists",
                    app_name, metadata.version
                ),
            });
        } else {
            fs::create_dir_all(app_dir)?;
        }

        // Copy everything into the official registry directory
        let files: Vec<PathBuf> = fs::read_dir(app_path)?
            .filter_map(|file| {
                if let Ok(entry) = file {
                    Some(entry.path())
                } else {
                    None
                }
            })
            .collect();

        fs_extra::copy_items(&files, app_dir, &fs_extra::dir::CopyOptions::new()).map_err(
            |error| {
                // Remove this new app version directory
                let _ = fs::remove_dir_all(app_dir);
                // Try to remove the parent directory. This will only work if no other versions of the
                // app exist.
                let _ = fs::remove_dir(format!("{}/{}", self.apps_dir, app_name));

                AppError::RegisterError {
                    err: format!("Error copying files into registry dir: {}", error),
                }
            },
        )?;

        let reg_entry = AppRegistryEntry {
            app: App {
                name: app_name.clone(),
                executable: format!("{}/{}", app_dir_str, app_exec),
                version: metadata.version,
                author: metadata.author,
                config,
            },
            active_version: true,
        };

        // Add the new registry entry
        entries.push(reg_entry);
        // Create the app.toml file and save the metadata information
        entries[entries.len() - 1].save().or_else(|err| {
            // Remove this new app version directory
            let _ = fs::remove_dir_all(app_dir);
            // Try to remove the parent directory. This will only work if no other versions of the
            // app exist.
            let _ = fs::remove_dir(format!("{}/{}", self.apps_dir, app_name));
            Err(err)
        })?;

        // Mark the old version as inactive
        if let Some(index) = old_active {
            entries[index].active_version = false;
            entries[index].save()?;
        }

        // Update the active app symlink
        self.set_active(&app_name, &app_dir_str)?;

        Ok(entries[entries.len() - 1].clone())
    }

    /// Uninstall a version of an application from the AppRegistry
    ///
    /// # Arguments
    ///
    /// * `app_name` - The name of the application
    /// * `version` - The version of the app to delete
    ///
    /// # Examples
    ///
    /// ```
    /// # use kubos_app::registry::AppRegistry;
    /// let registry = AppRegistry::new();
    /// registry.uninstall("my-app", "1.0");
    /// ```
    ///
    pub fn uninstall(&self, app_name: &str, version: &str) -> Result<bool, AppError> {
        let mut errors = None;

        // Delete the application files
        let app_dir = format!("{}/{}/{}", self.apps_dir, app_name, version);

        if let Err(error) = fs::remove_dir_all(app_dir) {
            errors = Some(format!("Failed to remove app directory: {}", error));
        }

        // If that was the last version, also remove the parent directory.
        // (If this call fails, it's probably because the directory wasn't empty because some
        // version of the app still exists, so ignore the error)
        if fs::remove_dir(format!("{}/{}", self.apps_dir, app_name)).is_ok() {
            // That worked, so we also want to remove the active version symlink
            if let Err(error) = fs::remove_file(format!("{}/active/{}", self.apps_dir, app_name)) {
                errors = Some(format!(
                    "{}. Failed to remove active symlink for {}: {}",
                    error, app_name, error
                ));
            }
        }

        // Remove the app entry from the registry list
        let mut entries = self
            .entries
            .lock()
            .map_err(|err| AppError::UninstallError {
                err: format!("Couldn't get entries mutex: {:?}", err),
            })?;

        match entries
            .iter()
            .position(|ref e| e.app.name == app_name && e.app.version == version)
        {
            Some(index) => {
                entries.remove(index);
            }
            None => {
                if let Some(error) = errors {
                    errors = Some(format!(
                        "{}. {} version {} not found in registry",
                        error, app_name, version
                    ));
                } else {
                    errors = Some(format!(
                        "{} version {} not found in registry",
                        app_name, version
                    ));
                }
            }
        }

        match errors {
            Some(err) => Err(AppError::UninstallError { err }),
            None => Ok(true),
        }
    }

    /// Uninstall all versions of an application from the AppRegistry
    ///
    /// # Arguments
    ///
    /// * `app_name` - The name of the application
    ///
    /// # Examples
    ///
    /// ```
    /// # use kubos_app::registry::AppRegistry;
    /// let registry = AppRegistry::new();
    /// registry.uninstall("my-app", "1.0");
    /// ```
    ///
    pub fn uninstall_all(&self, app_name: &str) -> Result<bool, AppError> {
        let mut errors = vec![];

        // Delete the application files
        let app_dir = format!("{}/{}", self.apps_dir, app_name);

        if let Err(error) = fs::remove_dir_all(app_dir) {
            errors.push(format!("Failed to remove app directory: {}", error));
        }

        // Remove the active version symlink
        if let Err(error) = fs::remove_file(format!("{}/active/{}", self.apps_dir, app_name)) {
            errors.push(format!(
                "Failed to remove active symlink for {}: {}",
                app_name, error
            ));
        }

        match self.entries.lock() {
            Ok(mut entries) => {
                // Remove the app entries from the registry list.
                // `drain_filter` is currently a nightly-only function, so instead we'll just keep
                // the apps that don't have the name we're trying to remove...
                entries.retain(|entry| entry.app.name != app_name);
            }
            Err(err) => errors.push(format!("Couldn't get entries mutex: {:?}", err)),
        }

        if errors.is_empty() {
            Ok(true)
        } else {
            let err = errors.join(". ");
            Err(AppError::UninstallError { err })
        }
    }

    /// Set the current active version of an application
    ///
    /// # Arguments
    ///
    /// * `app_name` - The name of the application
    /// * `version` - The version of the app to use
    ///
    /// # Examples
    ///
    /// ```
    /// # use kubos_app::registry::AppRegistry;
    /// let registry = AppRegistry::new();
    /// registry.set_version("my-app", "1.0");
    /// ```
    ///
    pub fn set_version(&self, app_name: &str, version: &str) -> Result<(), AppError> {
        let mut entries = self.entries.lock().map_err(|err| AppError::RegistryError {
            err: format!("Couldn't get entries mutex: {:?}", err),
        })?;

        // Get the current active version of the application
        let curr_active = entries
            .iter()
            .position(|ref e| e.active_version && e.app.name == app_name);

        if let Some(index) = curr_active {
            if entries[index].app.version == version {
                return Ok(());
            }
        }

        // Get the desired active version of the application
        let new_active = entries
            .iter()
            .position(|ref e| e.app.name == app_name && e.app.version == version)
            .ok_or(AppError::RegistryError {
                err: format!("App {} version {} not found in registry", app_name, version),
            })?;

        // Mark the new version as active
        entries[new_active].active_version = true;
        entries[new_active]
            .save()
            .map_err(|error| AppError::RegistryError {
                err: format!("Failed to update new active version entry: {:?}", error),
            })?;

        // Mark the old version as inactive
        if let Some(index) = curr_active {
            entries[index].active_version = false;
            entries[index]
                .save()
                .map_err(|error| AppError::RegistryError {
                    err: format!("Failed to update old active version entry: {:?}", error),
                })?;
        }

        // Update the active app symlink
        self.set_active(
            &app_name,
            &format!("{}/{}/{}", self.apps_dir, app_name, version),
        )?;

        Ok(())
    }

    /// Start an application. If successful, returns the PID of the application process.
    ///
    /// # Arguments
    ///
    /// * `app_name` - The name of the app to start
    /// * `run_level` - Which Run Level to run the app with
    ///
    /// # Examples
    ///
    /// ```
    /// # use kubos_app::registry::{AppRegistry, RunLevel};
    /// let registry = AppRegistry::new();
    /// registry.start_app("my-app", RunLevel::OnCommand);
    /// ```
    pub fn start_app(
        &self,
        app_name: &str,
        run_level: &RunLevel,
        config: Option<String>,
        args: Option<Vec<String>>,
    ) -> Result<Option<i32>, AppError> {
        // Look up the active version of the requested application
        let app = {
            let entries = self.entries.lock().map_err(|err| AppError::StartError {
                err: format!("Couldn't get entries mutex: {:?}", err),
            })?;
            match entries
                .iter()
                .find(|ref e| e.active_version && e.app.name == app_name)
            {
                Some(entry) => entry.app.clone(),
                None => {
                    return Err(AppError::StartError {
                        err: format!("No active version found for app {}", app_name),
                    });
                }
            }
        };

        let app_path = PathBuf::from(&app.executable);
        if !app_path.exists() {
            let msg = match self.uninstall(&app.name, &app.version) {
                Ok(_) => format!(
                    "{} does not exist. {} version {} automatically uninstalled",
                    app.executable, app.name, app.version
                ),
                Err(error) => format!("{} does not exist. {}", app.executable, error),
            };

            return Err(AppError::StartError { err: msg });
        }

        // Change our current directory to the app's directory so that it can access any
        // auxiliary files with relative file paths
        if let Err(err) = app_path
            .parent()
            .ok_or_else(|| format_err!("Failed to get parent dir"))
            .and_then(|parent_dir| {
                ::std::env::set_current_dir(parent_dir).map_err(|err| err.into())
            })
        {
            // If we can't change the current directory, we'll log an error and then just
            // continue trying to execute the application
            warn!("Failed to set cwd before executing {}: {:?}", app_name, err);
        }

        let mut cmd = Command::new(app_path);

        cmd.arg("-r").arg(format!("{}", run_level));

        let config_path = match config {
            // Use the requested config file
            Some(path) => path,
            // Use the config file which was set when the app was registered
            None => app.config.clone(),
        };

        cmd.arg("-c").arg(config_path.clone());

        if let Some(add_args) = args.clone() {
            cmd.args(&add_args);
        }

        let mut child = cmd.spawn().map_err(|err| AppError::StartError {
            err: format!("Failed to spawn app: {:?}", err),
        })?;

        let start_time = Utc::now();
        info!(
            "Starting {}. Run Level: {}, Config: {:?}, Args: {:?}",
            app_name, run_level, config_path, args
        );

        // Give the app a moment to run
        thread::sleep(Duration::from_millis(300));

        // See if the app already exited
        //
        // try_wait returns 1 of 3 things:
        //   - Ok(Some(status)) - App exited. Status is the exit code.
        //   - Ok(None) - App is still running
        //   - Err(err) - Something went wrong while trying to check if the app is still running.
        match child.try_wait() {
            Ok(Some(status)) => {
                // App exited already. Check for errors and then return
                if !status.success() {
                    Err(AppError::StartError {
                        err: format!("App returned {}", status),
                    })
                } else {
                    // App finished successfully, so there are no errors, but also no PID
                    Ok(None)
                }
            }
            Ok(None) => {
                let name = app_name.to_owned();
                let pid = child.id() as i32;
                let run_level_str = format!("{}", run_level);
                let registry = self.monitoring.clone();

                // Check if app is already running
                let running_status = find_entry(&registry, app_name, &run_level_str);
                if running_status == Ok(true) {
                    return Err(AppError::StartError {
                        err: format!("Instance of {} already running {}", app_name, run_level_str),
                    });
                } else if let Err(err) = running_status {
                    // The only way this happens is if the monitoring registry mutex gets poisoned.
                    // In that case, we want to crash this service so that it can be restarted in a
                    // good state.
                    error!("Crashing service: {:?}", err);
                    panic!("{:?}", err);
                }

                // Spawn monitor thread
                thread::spawn(move || {
                    monitor_app(
                        registry,
                        child,
                        MonitorEntry {
                            start_time,
                            name,
                            version: app.version,
                            pid,
                            run_level: run_level_str,
                            args,
                            config: config_path,
                        },
                    )
                });

                Ok(Some(pid))
            }
            Err(err) => Err(AppError::StartError {
                err: format!(
                    "Started app, but failed to fetch status information: {:?}",
                    err
                ),
            }),
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
    pub fn run_onboot(&self) -> Result<(), AppError> {
        let mut apps_started = 0;
        let mut apps_not_started = 0;

        let active_symlink = PathBuf::from(format!("{}/active", self.apps_dir));
        if !active_symlink.exists() {
            return Err(AppError::FileError {
                err: "Failed to get list of active applications".to_owned(),
            });
        }

        for entry in fs::read_dir(active_symlink)? {
            match entry {
                Ok(file) => {
                    let name = file.file_name();
                    match self.start_app(&name.to_string_lossy(), &RunLevel::OnBoot, None, None) {
                        Ok(_) => apps_started += 1,
                        Err(error) => {
                            error!("Failed to start {}: {}", name.to_string_lossy(), error);
                            apps_not_started += 1
                        }
                    }
                }
                Err(_) => apps_not_started += 1,
            }
        }

        info!(
            "Apps started: {}, Apps failed: {}",
            apps_started, apps_not_started
        );

        if apps_not_started != 0 {
            return Err(AppError::SystemError {
                err: format!("Failed to start {} app/s", apps_not_started),
            });
        }

        Ok(())
    }
}
