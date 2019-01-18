//
// Copyright (C) 2018 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use failure;

use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read};
use std::os::unix::fs::MetadataExt;
use std::str::FromStr;

use crate::process::root_dir;

#[derive(Clone)]
pub struct UserInfo {
    uid: u32,
    gid: u32,
}

impl UserInfo {
    /// Build user/group info for a running process by PID
    pub fn from_pid(pid: i32) -> Result<Self, failure::Error> {
        let meta = fs::metadata(root_path!("proc", pid))?;
        Ok(UserInfo::new(meta.uid(), meta.gid()))
    }

    /// Parse a user or group name given it's ID in the format used by /etc/passwd and /etc/group
    fn name_from_id<R>(read: R, id: u32) -> Option<String>
    where
        R: Read,
    {
        for line in BufReader::new(read).lines().filter_map(|l| l.ok()) {
            let tokens: Vec<&str> = line.split(':').collect();
            if tokens.len() < 3 {
                continue;
            }

            if Ok(id) == u32::from_str(tokens[2]) {
                return Some(String::from(tokens[0]));
            }
        }
        None
    }

    pub fn new(uid: u32, gid: u32) -> Self {
        Self { uid, gid }
    }

    /// The user ID used by the system
    pub fn uid(&self) -> u32 {
        self.uid
    }

    /// The group ID used by the system
    pub fn gid(&self) -> u32 {
        self.gid
    }

    /// Returns the username associated with `uid` in `/etc/passwd`, or `None` if a username for
    /// the uid was not found
    pub fn user(&self) -> Option<String> {
        match File::open(root_path!("etc", "passwd")) {
            Ok(file) => Self::name_from_id(file, self.uid),
            Err(_) => None,
        }
    }

    /// Returns the group name associated with `gid` in `/etc/group`, or `None` if a group name for
    /// the gid was not found
    pub fn group(&self) -> Option<String> {
        match File::open(root_path!("etc", "group")) {
            Ok(file) => Self::name_from_id(file, self.gid),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn userinfo_user() {
        let passwd: &[u8] = b"root:x:0:0:root:/root:/bin/sh\n\
                              daemon:x:1:1:daemon:/usr/sbin:/bin/false\n\
                              bin:x:2:2:bin:/bin:/bin/false\n\
                              kubos:x:1000:100::/home/kubos:/bin/sh";
        let username = |uid| UserInfo::name_from_id(passwd, uid);

        assert_eq!(username(0), Some("root".into()));
        assert_eq!(username(1), Some("daemon".into()));
        assert_eq!(username(2), Some("bin".into()));
        assert_eq!(username(1000), Some("kubos".into()));
        assert_eq!(username(100), None);
    }

    #[test]
    fn userinfo_group() {
        let groups: &[u8] = b"root:x:0:\n\
                              daemon:x:1:\n\
                              bin:x:2:\n\
                              users:x:100:";
        let groupname = |gid| UserInfo::name_from_id(groups, gid);

        assert_eq!(groupname(0), Some("root".into()));
        assert_eq!(groupname(1), Some("daemon".into()));
        assert_eq!(groupname(2), Some("bin".into()));
        assert_eq!(groupname(100), Some("users".into()));
        assert_eq!(groupname(1000), None);
    }

    #[test]
    fn userinfo_from_uid_gid() {
        // Loaded from info in tests/testroot/etc/group and tests/testroot/etc/passwd
        let operator = UserInfo::new(37, 37);
        assert_eq!(operator.user(), Some("operator".into()));
        assert_eq!(operator.group(), Some("operator".into()));

        let kubos = UserInfo::new(1000, 100);
        assert_eq!(kubos.user(), Some("kubos".into()));
        assert_eq!(kubos.group(), Some("users".into()));

        let system = UserInfo::new(1001, 100);
        assert_eq!(system.user(), Some("system".into()));
        assert_eq!(system.group(), Some("users".into()));

        let nouser = UserInfo::new(5000, 5000);
        assert_eq!(nouser.user(), None);
        assert_eq!(nouser.group(), None);
    }
}
