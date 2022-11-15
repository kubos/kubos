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
use log::*;
use nix::sys::signal;
use nix::unistd::Pid;
use std::ffi::OsStr;
use std::fs;
use std::io::Read;
use std::os::unix;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

/// The default application registry directory in KubOS
pub static K_APPS_DIR: &str = "/home/system/kubos/apps";
pub static DEFAULT_CONFIG: &str = "/etc/kubos-config.toml";

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
        let active_dir = PathBuf::from(format!("{}/active", apps_dir));
        if !active_dir.exists() {
            fs::create_dir_all(&active_dir)?;
        }

        // Create absolute path from apps_dir so symlinks work
        let apps_path = Path::new(&apps_dir);
        let apps_dir = apps_path
            .canonicalize()
            .map_err(|e| AppError::RegistryError {
                err: format!("Failed to get absolute apps_dir: {}", e),
            })?;
        let apps_dir = apps_dir.to_str().ok_or_else(|| AppError::RegistryError {
            err: format!("Failed to create absolute apps_dir path: {:?}", apps_path),
        })?;

        let registry = AppRegistry {
            entries: Arc::new(Mutex::new(Vec::new())),
            monitoring: Arc::new(Mutex::new(Vec::new())),
            apps_dir: String::from(apps_dir),
        };

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

        for entry in fs::read_dir(&self.apps_dir)?.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() && entry.file_name().to_str() != Some("active") {
                    reg_entries.extend(self.discover_versions(entry.path())?);
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
                .file_type().map(|file_type| file_type.is_dir())
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
    /// * `path` - The path to either an application directory containing a manifest and binary,
    ///            or the path to a .tgz file containing a manifest and binary at its root.
    ///            (Create a flat tar file in an application directory with a command like
    ///             'tar -czf archive.tgz *')
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
        } else if !app_path.is_dir() {
            // Handle tgz archives.
            return match app_path.extension().and_then(OsStr::to_str) {
                Some("tgz") => extract_archive(self, path),
                Some(extension) => Err(AppError::RegisterError {
                                            err: format!("Provided file with extension {} is neither a directory nor a tgz archive file", extension)
                                         }),
                None => Err(AppError::RegisterError {
                            err: String::from("Provided file is neither a directory nor a tgz archive file"),
                        })
            };
        }

        // Load the metadata
        let mut data = String::new();
        fs::File::open(app_path.join("manifest.toml"))
            .and_then(|mut fp| fp.read_to_string(&mut data))
            .map_err(|error| AppError::RegisterError {
                err: format!("Unable to load manifest.toml: {}", error),
            })?;

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
                .position(|e| e.active_version && e.app.name == app_name)
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
        entries[entries.len() - 1].save().map_err(|err| {
            // Remove this new app version directory
            let _ = fs::remove_dir_all(app_dir);
            // Try to remove the parent directory. This will only work if no other versions of the
            // app exist.
            let _ = fs::remove_dir(format!("{}/{}", self.apps_dir, app_name));
            err
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
        let mut errors: Vec<String> = vec![];

        // Delete the application files
        let app_dir = format!("{}/{}/{}", self.apps_dir, app_name, version);

        if let Err(error) = fs::remove_dir_all(app_dir) {
            errors.push(format!("Failed to remove app directory: {}", error));
        }

        // If that was the last version, also remove the parent directory.
        // (If this call fails, it's probably because the directory wasn't empty because some
        // version of the app still exists, so ignore the error)
        if fs::remove_dir(format!("{}/{}", self.apps_dir, app_name)).is_ok() {
            // That worked, so we also want to remove the active version symlink
            if let Err(error) = fs::remove_file(format!("{}/active/{}", self.apps_dir, app_name)) {
                errors.push(format!(
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
            .position(|e| e.app.name == app_name && e.app.version == version)
        {
            Some(index) => {
                entries.remove(index);
            }
            None => {
                errors.push(format!(
                    "{} version {} not found in registry",
                    app_name, version
                ));
            }
        }

        // Kill any instances of this version of the app which are still running
        if let Ok(Some(entry)) = find_running(&self.monitoring, app_name) {
            if entry.version == version {
                if let Some(pid) = entry.pid {
                    if let Err(err) = uninstall_kill(pid) {
                        errors.push(format!("Failed to kill {}: {:?}", app_name, err));
                    }
                }
            }
        }

        // Remove the app entry from the monitoring list
        // Note: If the app was never run, then no entry will be present
        if let Err(new_error) = remove_entry(&self.monitoring, app_name, version) {
            errors.push(new_error.to_string());
        }

        if errors.is_empty() {
            Ok(true)
        } else {
            let err: String = errors.join(". ");
            Err(AppError::UninstallError { err })
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

        // Kill any instances of the app which are still running
        if let Ok(Some(entry)) = find_running(&self.monitoring, app_name) {
            if let Some(pid) = entry.pid {
                if let Err(err) = uninstall_kill(pid) {
                    errors.push(format!("Failed to kill {}: {:?}", app_name, err));
                }
            }
        }

        // Remove all matching app entries from the monitoring list
        if let Err(new_error) = remove_entries(&self.monitoring, app_name) {
            errors.push(new_error.to_string());
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
            .position(|e| e.active_version && e.app.name == app_name);

        if let Some(index) = curr_active {
            if entries[index].app.version == version {
                return Ok(());
            }
        }

        // Get the desired active version of the application
        let new_active = entries
            .iter()
            .position(|e| e.app.name == app_name && e.app.version == version)
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
            app_name,
            &format!("{}/{}/{}", self.apps_dir, app_name, version),
        )?;

        Ok(())
    }

    /// Start an application. If successful, returns the PID of the application process.
    ///
    /// # Arguments
    ///
    /// * `app_name` - The name of the app to start
    /// * `config` - (Optional) The custom config file path to use
    /// * `args` - (Optional) Arguments which should be passed to the application
    ///
    /// # Examples
    ///
    /// ```
    /// # use kubos_app::registry::AppRegistry;
    /// let registry = AppRegistry::new();
    /// registry.start_app("my-app", None, None);
    /// ```
    pub fn start_app(
        &self,
        app_name: &str,
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
                .find(|e| e.active_version && e.app.name == app_name)
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

            error!("{}", msg);
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

        // Check if app is already running
        let running_status = find_running(&self.monitoring, app_name);
        match running_status {
            Ok(None) => {}
            Ok(Some(_)) => {
                return Err(AppError::StartError {
                    err: format!("Instance of {} already running", app_name),
                });
            }
            Err(err) => {
                // The only way this happens is if the monitoring registry mutex gets poisoned.
                // In that case, we want to crash this service so that it can be restarted in a
                // good state.
                error!("Crashing service: {:?}", err);
                panic!("{:?}", err);
            }
        }

        let mut cmd = Command::new(app_path);

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

        let mut child = cmd.spawn().map_err(|err| {
            error!("Failed to spawn app {}: {:?}", app_name, err);
            AppError::StartError {
                err: format!("Failed to spawn app: {:?}", err),
            }
        })?;

        let start_time = Utc::now();
        info!(
            "Starting {}. Config: {:?}, Args: {:?}",
            app_name, config_path, args
        );

        // Add/update the monitoring registry with the new run info
        let entry = MonitorEntry {
            start_time,
            end_time: None,
            name: app.name.clone(),
            version: app.version.clone(),
            running: true,
            pid: Some(child.id() as i32),
            last_rc: None,
            last_signal: None,
            args,
            config: config_path,
        };
        if let Err(error) = start_entry(&self.monitoring, &entry) {
            // The only way this happens is if the monitoring registry mutex gets poisoned.
            // In that case, we want to crash this service so that it can be restarted in a
            // good state.
            error!("Crashing service: {:?}", error);
            panic!("{:?}", error);
        }

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
                finish_entry(&self.monitoring, app_name, &app.version, status)?;

                if !status.success() {
                    Err(AppError::StartError {
                        err: format!("App returned {}", status),
                    })
                } else {
                    Ok(None)
                }
            }
            Ok(None) => {
                let name = app_name.to_owned();
                let pid = child.id() as i32;
                let registry = self.monitoring.clone();

                // Spawn monitor thread
                thread::spawn(move || {
                    let result = monitor_app(registry, child, &name, &app.version);

                    if let Err(error) = result {
                        error!("{:?}", error);
                    }
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

    pub fn kill_app(&self, name: &str, signal: Option<i32>) -> Result<(), AppError> {
        // Lookup the app in the monitoring registry to get the PID to kill
        let app = find_running(&self.monitoring, name)?.ok_or(AppError::KillError {
            err: "No matching monitoring entry found".to_owned(),
        })?;

        let pid = app.pid.ok_or(AppError::KillError {
            err: "No active PID found in registry".to_owned(),
        })?;

        let pid = Pid::from_raw(pid);
        let sig = signal::Signal::from_c_int(signal.unwrap_or(15) as i32)
            .unwrap_or(signal::Signal::SIGTERM);

        signal::kill(pid, sig).map_err(|err| AppError::KillError {
            err: err.to_string(),
        })
    }
}

fn uninstall_kill(pid: i32) -> Result<(), nix::Error> {
    let pid = Pid::from_raw(pid);
    signal::kill(pid, Some(signal::Signal::SIGTERM))?;
    thread::spawn(move || {
        // Give the app 2 seconds to shut down nicely
        thread::sleep(Duration::from_secs(2));
        // Kill it less nicely
        // (If the app already stopped, this call will fail and do nothing)
        let _ = signal::kill(pid, Some(signal::Signal::SIGKILL));
    });

    Ok(())
}

fn extract_archive(registry: &AppRegistry, path: &str) -> Result<AppRegistryEntry, AppError> {
    let tmp_dir = TempDir::new().map_err(|error| AppError::RegisterError {
        err: format!(
            "Error creating temporary directory to expand archive: {}",
            error
        ),
    })?;

    let mut command = if PathBuf::from("/usr/bin/tar").exists() {
        Command::new("/usr/bin/tar")
    } else if PathBuf::from("/bin/tar").exists() {
        Command::new("/bin/tar")
    } else {
        return Err(AppError::RegisterError {
            err: String::from("Error expanding archive: tar command not found"),
        });
    };

    let output = command
        .arg("-zxf")
        .arg(path)
        .arg("--directory")
        .arg(tmp_dir.path())
        .output()
        .map_err(|error| AppError::RegisterError {
            err: format!("Error expanding archive: {}", error),
        })?;

    if output.status.success() {
        // Ensure they packaged the tarball correctly.
        if Path::new(&tmp_dir.path().join("manifest.toml")).exists() {
            let tmp_dir_path =
                OsStr::to_str(tmp_dir.path().as_os_str()).ok_or(AppError::RegisterError {
                    err: String::from("Error converting temp dir path to UTF8"),
                })?;
            registry.register(tmp_dir_path)
        } else {
            Err(AppError::RegisterError {
                err: String::from("Manifest file manifest.toml not found in root of archive. When you create the archive, do so in the application directory, with a command like: tar -czf archive.tgz *")
            })
        }
    } else {
        Err(AppError::RegisterError {
            err: format!(
                "Non-successful status when expanding archive: {:?}",
                output.status.code()
            ),
        })
    }
}
