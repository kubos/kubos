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
//
use crate::meminfo::MemInfo;
use crate::process::ProcStat;
use crate::userinfo::UserInfo;
use systemstat::{Filesystem, LoadAverage};

pub struct MemInfoResponse {
    pub info: MemInfo,
}

graphql_object!(MemInfoResponse: () |&self| {
    field total() -> Option<i32> {
        self.info.total().map(|v| v as i32)
    }

    field free() -> Option<i32> {
        self.info.free().map(|v| v as i32)
    }

    field available() -> Option<i32> {
        self.info.available().map(|v| v as i32)
    }

    field low_free() -> Option<i32> {
        self.info.low_free().map(|v| v as i32)
    }
});

pub struct LoadAverageResponse {
    pub load_average: LoadAverage,
}

graphql_object!(LoadAverageResponse: () |&self| {
    field one() -> f64 {
        self.load_average.one.into()
    }

    field five() -> f64 {
        self.load_average.five.into()
    }

    field fifteen() -> f64 {
        self.load_average.fifteen.into()
    }
});

pub struct MountResponse {
    pub mount: Filesystem,
}

graphql_object!(MountResponse: () |&self| {
    field files() -> f64 {
        self.mount.files as f64
    }

    field files_total() -> f64 {
        self.mount.files_total as f64
    }

    field files_avail() -> f64 {
        self.mount.files_avail as f64
    }

    field free() -> f64 {
        self.mount.free.as_u64() as f64
    }

    field avail() -> f64 {
        self.mount.avail.as_u64() as f64
    }

    field total() -> f64 {
        self.mount.total.as_u64() as f64
    }

    field fs_type() -> String {
        self.mount.fs_type.clone()
    }

    field fs_mounted_from() -> String {
        self.mount.fs_mounted_from.clone()
    }

    field fs_mounted_on() -> String {
        self.mount.fs_mounted_on.clone()
    }
});

pub struct PSResponse {
    pub pid: i32,
    pub user: Option<UserInfo>,
    pub stat: Option<ProcStat>,
}

impl PSResponse {
    pub fn new(pid: i32) -> PSResponse {
        PSResponse {
            pid,
            user: UserInfo::from_pid(pid).ok(),
            stat: ProcStat::from_pid(pid).ok(),
        }
    }
}

graphql_object!(PSResponse: () |&self| {
    field pid(&executor) -> i32 {
        self.pid
    }

    field uid(&executor) -> Option<i32> {
        self.user.as_ref().map(|u| u.uid() as i32)
    }

    field gid(&executor) -> Option<i32> {
        self.user.as_ref().map(|u| u.gid() as i32)
    }

    field usr(&executor) -> Option<String> {
        self.user.as_ref().and_then(|u| u.user())
    }

    field grp(&executor) -> Option<String> {
        self.user.as_ref().and_then(|u| u.group())
    }

    field state(&executor) -> Option<String> {
        self.stat.as_ref().map(|stat| stat.state().to_string())
    }

    field ppid(&executor) -> Option<i32> {
        self.stat.as_ref().map(|stat| stat.parent_pid())
    }

    field mem(&executor) -> Option<i32> {
        self.stat.as_ref().map(|stat| stat.mem_usage() as i32)
    }

    field rss(&executor) -> Option<i32> {
        self.stat.as_ref().map(|stat| stat.rss() as i32)
    }

    field threads(&executor) -> Option<i32> {
        self.stat.as_ref().map(|stat| stat.num_threads() as i32)
    }

    field cmd(&executor) -> Option<String> {
        self.stat.as_ref().and_then(|stat| {
            stat.cmd().ok().map(|argv| argv.join(" "))
        })
    }
});
