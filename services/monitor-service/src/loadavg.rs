//
// Copyright (C) 2022 Xplore Inc.
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
use std::f64;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct LoadAvg {
    load_1m: Option<f64>,
    load_5m: Option<f64>,
    load_15m: Option<f64>,
    processes_active: Option<u64>,
    processes_total:  Option<u64>,
    last_pid: Option<u64>,
}

impl LoadAvg {
    pub fn new() -> Self {
        Self {
            load_1m: None,
            load_5m: None,
            load_15m: None,
            processes_active: None,
            processes_total: None,
            last_pid: None,
        }
    }

    fn parse_f64(avg: &str) -> Option<f64> {
        match avg.parse::<f64>() {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    fn parse_u64(s: &str) -> Option<u64>{
        match s.parse::<u64>() {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    pub fn parse<R>(mut raw: R) -> Result<LoadAvg, failure::Error>
    where
        R: BufRead,
    {
        let mut load_avg: LoadAvg = LoadAvg::new();
        let mut buf = String::new();
        let _len = raw.read_line(&mut buf)?;
        let ws = buf.split_whitespace();
        let split_vec = ws.collect::<Vec<_>>().to_owned();
        let split_procs = split_vec[3].split('/').collect::<Vec<_>>();

        load_avg.load_1m = Self::parse_f64(split_vec[0]);
        load_avg.load_5m = Self::parse_f64(split_vec[1]);
        load_avg.load_15m = Self::parse_f64(split_vec[2]);
        load_avg.processes_active =Self::parse_u64(split_procs[0]);
        load_avg.processes_total = Self::parse_u64(split_procs[1]);
        load_avg.last_pid = Self::parse_u64(split_vec[4]);

        Ok(load_avg)
    }

    pub fn from_proc() -> Result<LoadAvg, failure::Error> {
        let file = File::open("/proc/loadavg")?;
        let reader = BufReader::new(file);

        Self::parse(reader)
    }

    // System load averaged over the past one minute
    pub fn load_1m(&self) -> Option<f64> {
        self.load_1m
    }

    // System load averaged over the past five minutes
    pub fn load_5m(&self) -> Option<f64> {
        self.load_5m
    }

    // System load averaged over the past fifteen minutes
    pub fn load_15m(&self) -> Option<f64> {
        self.load_15m
    }

    // System active processes
    pub fn processes_active(&self) -> Option<u64> {
        self.processes_active
    }


    // System total number of processes
    pub fn processes_total(&self) -> Option<u64> {
        self.processes_total
    }

    // Total threads launched on the system
    pub fn last_pid(&self) -> Option<u64> {
        self.last_pid
    }
} 

#[cfg(test)]
#[allow(clippy::unreadable_literal)]
mod tests {
    use super::*;

    const RAW0: &[u8] = b"0.03 0.05 0.06 1/541 6275\n";
    //const RAW1: &[u8] = b"0.19 0.13 0.09 1/543 11348\n";
    //const RAW_PARTIAL: &[u8] = b"0.19 0.13 \n";

    #[test]
    fn loadavg_parse() {
        let info: Result<LoadAvg, failure::Error> = LoadAvg::parse(RAW0);
        assert_eq!(
            info.ok(),
            Some(LoadAvg {
                load_1m: Some(0.03),
                load_5m: Some(0.05),
                load_15m: Some(0.06),
                processes_active: Some(1),
                processes_total: Some(541),
                last_pid: Some(6275),
            })
        );
    }
}
