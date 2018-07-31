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
use std::os::unix::fs::MetadataExt;
use std::io::{Read, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// Stats provided by the Linux /proc/<pid>/stat file format
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProcStat {
    pid: i32,
    comm: String,
    state: char,
    ppid: i32,
    pgrp: i32,
    session: i32,
    tty_nr: i32,
    tpgid: i32,
    flags: u32,
    minflt: u32,
    cminflt: u32,
    majflt: u32,
    cmajflt: u32,
    utime: u32,
    stime: u32,
    cutime: i32,
    cstime: i32,
    priority: i32,
    nice: i32,
    num_threads: i32,
    itrealvalue: i32,
    starttime: i64,
    vsize: u32,
    rss: i32,
    rsslim: u32,
    startcode: u32,
    endcode: u32,
    startstack: u32,
    kstkesp: u32,
    kstkeip: u32,
    signal: u32,
    blocked: u32,
    sigignore: u32,
    sigcatch: u32,
    wchan: u32,
    nswap: u32,
    cnswap: u32,
    exit_signal: i32,
    processor: i32,
    rt_priority: u32,
    policy: u32,
    delayacct_blkio_ticks: u64,
    guest_time: u32,
    cguest_time: i32,
    start_data: u32,
    end_data: u32,
    start_brk: u32,
    arg_start: u32,
    arg_end: u32,
    env_start: u32,
    env_end: u32,
    exit_code: i32,
}

#[cfg(not(test))]
#[inline]
fn root_dir() -> PathBuf {
    PathBuf::from("/")
}

macro_rules! root_path {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_path = root_dir();
            $(
                temp_path.push($x.to_string());
            )*
            temp_path
        }
    };
}

impl ProcStat {
    /// Convenience function that parses the stat file for a specific process ID
    /// See ProcStat::parse for more information
    pub fn from_pid(pid: i32) -> Result<Self, failure::Error> {
        let file = File::open(root_path!("proc", pid, "stat"))?;
        Ok(Self::parse(BufReader::new(file)))
    }

    /// Parse a String with the format of a /proc/[pid]/stat file
    /// See http://man7.org/linux/man-pages/man5/proc.5.html for more information
    pub fn parse<R>(stat: R) -> Self
        where R: Read
    {
        let mut ps = Self::default();

        // Order and format of these fields taken from
        // http://man7.org/linux/man-pages/man5/proc.5.html
        scan!(stat.bytes().map(|c| c.unwrap()) =>
            "{} ({}) {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} \
             {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
            ps.pid, ps.comm, ps.state, ps.ppid, ps.pgrp, ps.session, ps.tty_nr, ps.tpgid,
            ps.flags, ps.minflt, ps.cminflt, ps.majflt, ps.cmajflt, ps.utime, ps.stime, ps.cutime,
            ps.cstime, ps.priority, ps.nice, ps.num_threads, ps.itrealvalue, ps.starttime,
            ps.vsize, ps.rss, ps.rsslim, ps.startcode, ps.endcode, ps.startstack, ps.kstkesp,
            ps.kstkeip, ps.signal, ps.blocked, ps.sigignore, ps.sigcatch, ps.wchan, ps.nswap,
            ps.cnswap, ps.exit_signal, ps.processor, ps.rt_priority, ps.policy,
            ps.delayacct_blkio_ticks, ps.guest_time, ps.cguest_time, ps.start_data, ps.end_data,
            ps.start_brk, ps.arg_start, ps.arg_end, ps.env_start, ps.env_end, ps.exit_code
        );

        ps
    }

    /// One of the following characters, indicating process state:
    ///
    /// * `R` - Running
    /// * `S` - Sleeping in an interruptible wait
    /// * `D` - Waiting in uninterruptible disk sleep
    /// * `Z` - Zombie
    /// * `T` - Stopped (on a signal) or (before Linux 2.6.33) trace stopped
    /// * `t` - Tracing stop (Linux 2.6.33 onward)
    /// * `W` - Paging (only before Linux 2.6.0)
    /// * `X` - Dead (from Linux 2.6.0 onward)
    /// * `x` - Dead (Linux 2.6.33 to 3.13 only)
    /// * `K` - Wakekill (Linux 2.6.33 to 3.13 only)
    /// * `W` - Waking (Linux 2.6.33 to 3.13 only)
    /// * `P` - Parked (Linux 3.9 to 3.13 only)
    pub fn state(&self) -> char {
        self.state
    }

    /// The PID of the parent of this process
    pub fn parent_pid(&self) -> i32 {
        self.ppid
    }

    /// Virtual memory size in bytes
    pub fn mem_usage(&self) -> u32 {
        self.vsize
    }

    /// Resident Set Size: number of pages the process has in real memory.  This is just the pages
    /// which count toward text, data, or stack space.  This does not include pages which have not
    /// been demand-loaded in, or which are swapped out.
    pub fn rss(&self) -> i32 {
        self.rss
    }

    /// Number of threads in this process
    pub fn num_threads(&self) -> i32 {
        self.num_threads
    }

    /// Attempts to read the command line arguments used to execute this process, and falls
    /// back to the raw process name if /proc/[pid]/cmdline does not exist or is empty
    pub fn cmd(&self) -> Result<Vec<String>, failure::Error> {
        let file = File::open(root_path!("proc", self.pid, "cmdline"))?;
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;
        if contents.len() == 0 {
            Ok(vec![self.comm.clone()])
        } else {
            Ok(contents.split('\0').into_iter().map(|a| String::from(a)).collect())
        }
    }
}

/// Finds the running process IDs by finding the valid numerical directory names in /proc
pub fn running_pids() -> Vec<i32> {
    let mut info: Vec<i32> = Vec::new();
    let entries = match fs::read_dir(root_path!("proc")) {
        Ok(e) => e,
        Err(_) => return info,
    };

    for entry in entries.filter_map(|e| e.ok()).filter(|e| e.path().is_dir()) {
        if let Ok(pid) = i32::from_str(&entry.file_name().to_string_lossy()) {
            info.push(pid);
        }
    }
    info
}

#[derive(Clone)]
pub struct UserInfo {
    uid: u32,
    gid: u32,
}

impl UserInfo {

    /// Build user/group info for a running process by PID
    pub fn from_pid(pid: i32) -> Result<UserInfo, failure::Error> {
        let meta = fs::metadata(root_path!("proc", pid))?;
        Ok(UserInfo::new(meta.uid(), meta.gid()))
    }

    /// Parse a user or group name given it's ID in the format used by /etc/passwd and /etc/group
    fn name_from_id<R>(read: R, id: u32) -> Option<String>
        where R: Read
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
    pub fn uid(&self) -> u32 { self.uid }

    /// The group ID used by the system
    pub fn gid(&self) -> u32 { self.gid }

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

// Unit tests

#[cfg(test)]
lazy_static! {
    static ref ROOTFS: PathBuf = {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("testroot");
        path 
    };
}

#[cfg(test)]
fn root_dir() -> PathBuf {
    ROOTFS.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    const STAT: &[u8] = b"720 (sh) S 1 720 720 0 -1 4194560 240 1400 0 0 1 2 9 3 20 0 1 0 \
                          248701832 2981888 458 4294967295 65536 425632 3200257616 3200256600 \
                          3068202372 0 0 3670016 95335 1 0 0 17 0 0 0 0 0 0 491520 492921 \
                          2895872 3200257854 3200257858 3200257858 3200258036 0";

    #[test]
    fn procstat_parse() {
        assert_eq!(ProcStat::parse(STAT), ProcStat {
            pid: 720, comm: "sh".into(),
            state: 'S', ppid: 1,
            pgrp: 720, session: 720,
            tty_nr: 0, tpgid: -1,
            flags: 4194560, minflt: 240,
            cminflt: 1400, majflt: 0,
            cmajflt: 0, utime: 1,
            stime: 2, cutime: 9,
            cstime: 3, priority: 20,
            nice: 0, num_threads: 1,
            itrealvalue: 0, starttime: 248701832,
            vsize: 2981888, rss: 458,
            rsslim: 4294967295, startcode: 65536,
            endcode: 425632, startstack: 3200257616,
            kstkesp: 3200256600, kstkeip: 3068202372,
            signal: 0, blocked: 0,
            sigignore: 3670016, sigcatch: 95335,
            wchan: 1, nswap: 0,
            cnswap: 0, exit_signal: 17,
            processor: 0, rt_priority: 0,
            policy: 0, delayacct_blkio_ticks: 0,
            guest_time: 0, cguest_time: 0,
            start_data: 491520, end_data: 492921,
            start_brk: 2895872, arg_start: 3200257854,
            arg_end: 3200257858, env_start: 3200257858,
            env_end: 3200258036, exit_code: 0,
        });
    }

    #[test]
    fn procstat_getters() {
        let stat = ProcStat::parse(STAT);
        assert_eq!(stat.state(), 'S');
        assert_eq!(stat.parent_pid(), 1);
        assert_eq!(stat.mem_usage(), 2981888);
        assert_eq!(stat.rss(), 458);
        assert_eq!(stat.num_threads(), 1);
    }


    #[test]
    fn procstat_from_pid() {
        let stat = ProcStat::from_pid(232).unwrap();
        assert_eq!(stat.state(), 'S');
        assert_eq!(stat.parent_pid(), 2);
        assert_eq!(stat.mem_usage(), 0);
        assert_eq!(stat.rss(), 0);
        assert_eq!(stat.num_threads(), 1);

        let cmd = stat.cmd().unwrap();
        assert_eq!(cmd, ["edac-poller"]);
    }

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

    #[test]
    fn running_pids() {
        let mut pids = super::running_pids();
        pids.sort_unstable();

        assert_eq!(pids, vec![232, 380, 720, 761]);
    }
}
