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

pub const K_APPS_DIR: &'static str = "/home/system/kubos/apps";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppMetadata {
    pub name: String,
    pub version: String,
    pub author: String,
}

impl AppMetadata {
    pub fn new(name: &str, version: &str, author: &str) -> AppMetadata {
        AppMetadata {
            name: name.to_string(),
            version: version.to_string(),
            author: author.to_string()
        }
    }
}

/// Kubos RunLevel struct
#[derive(Clone,Debug,Deserialize,Serialize,PartialEq)]
pub enum RunLevel {
    OnBoot,
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
    pub uuid: String,
    pub pid: u32,
    pub path: String,
    pub metadata: AppMetadata,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct AppRegistryEntry {
    pub active: bool,
    pub run_level: RunLevel,
    pub app: App,
}


impl Clone for AppRegistryEntry {
    fn clone(&self) -> AppRegistryEntry {
        AppRegistryEntry {
            app: self.app.clone(),
            active: self.active,
            run_level: self.run_level.clone()
        }
    }
}

impl AppRegistryEntry {
    pub fn from_dir(dir: &str) -> Option<AppRegistryEntry> {
        let app_toml: PathBuf = [dir, "app.toml"].iter().collect();
        if !app_toml.exists() {
            return None;
        }

        let mut f = fs::File::open(app_toml).unwrap();
        let mut buffer = String::new();
        if let Ok(_) = f.read_to_string(&mut buffer) {
            if let Ok(entry) = toml::from_str::<AppRegistryEntry>(&buffer) {
                return Some(entry);
            }
        }
        return None;
    }

    pub fn save(&self) -> Result<bool, String> {
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

        //let mut file = fs::File::create(app_toml).unwrap();
        //let _result = file.write_all(&toml::to_string(&self).unwrap().into_bytes());
    }
}

#[derive(Deserialize,Serialize)]
pub struct AppRegistry {
    pub entries: RefCell<Vec<AppRegistryEntry>>,
    pub apps_dir: String
}

impl AppRegistry {
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
                if let Ok(version) = version {
                    if let Ok(v_file_type) = version.file_type() {
                        if v_file_type.is_dir() {
                            let v_path = version.path();
                            let version_path = v_path.to_str().expect("invalid version path");

                            if let Some(entry) = AppRegistryEntry::from_dir(version_path) {
                                reg_entries.push(entry);
                            }
                        }
                    }
                }
            }
        }
        reg_entries
    }

    pub fn register(&self, path: &str) -> Result<AppRegistryEntry, String> {
        let app_path = Path::new(path);
        if !app_path.exists() {
            return Err(format!("{} does not exist", path));
        }

        let app_metadata = Command::new(path)
                                   .args(&["--metadata"])
                                   .output()
                                   .expect("Failed to get app metadata");

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

        assert!(apps_dir.exists());
        let app_dir_path = format!("{}/{}/{}", self.apps_dir, app_uuid_str,
                                   metadata.version.as_str());
        let app_dir = Path::new(&app_dir_path);

        if !app_dir.exists() {
            match fs::create_dir_all(app_dir) {
                Err(err) => { return Err(format!("Couldn't create app dir {}: {:?}", app_dir.display(), err)); },
                Ok(_) => {}
            }
        }

        let app_filename = app_path.file_name().expect("couldn't get app filename");

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
    }

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
