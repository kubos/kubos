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

use failure;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct MemInfo {
    total: Option<u32>,
    free: Option<u32>,
    available: Option<u32>,
    low_free: Option<u32>,
}

impl MemInfo {
    /// Create an empty MemInfo object
    pub fn new() -> Self {
        Self {
            total: None,
            free: None,
            available: None,
            low_free: None,
        }
    }

    fn parse_amount(amount: &str) -> Option<u32> {
        let mut iter = amount.split_whitespace();
        match iter.next() {
            Some(amount) => match u32::from_str(amount) {
                Ok(amount) => Some(amount),
                Err(_) => None,
            },
            None => None,
        }
    }

    pub fn parse<R>(info: R) -> Result<MemInfo, failure::Error>
    where
        R: BufRead,
    {
        let mut mem_info = MemInfo::new();

        for line in info.lines() {
            let line = line?;
            let mut iter = line.split_whitespace();
            let key = iter.next();
            let val = iter.next();

            if let (Some(key), Some(val)) = (key, val) {
                match key.get(0..key.len() - 1).unwrap_or("") {
                    "MemTotal" => mem_info.total = Self::parse_amount(val),
                    "MemFree" => mem_info.free = Self::parse_amount(val),
                    "MemAvailable" => mem_info.available = Self::parse_amount(val),
                    "LowFree" => mem_info.low_free = Self::parse_amount(val),
                    _ => {}
                }
            }
        }
        Ok(mem_info)
    }

    pub fn from_proc() -> Result<MemInfo, failure::Error> {
        let file = File::open("/proc/meminfo")?;
        let reader = BufReader::new(file);
        Self::parse(reader)
    }

    /// Total system memory available in kB
    pub fn total(&self) -> Option<u32> {
        self.total
    }
    /// Total system memory free in kB
    pub fn free(&self) -> Option<u32> {
        self.free
    }
    /// Total system memory available in kB
    pub fn available(&self) -> Option<u32> {
        self.available
    }
    /// The low mark for system memory free in kB
    pub fn low_free(&self) -> Option<u32> {
        self.low_free
    }
}

#[cfg(test)]
#[allow(clippy::unreadable_literal)]
mod tests {
    use super::*;

    const RAW: &[u8] = b"MemTotal:         515352 kB\n\
                         MemFree:          317980 kB\n\
                         MemAvailable:     498232 kB\n\
                         Buffers:            4736 kB\n\
                         Cached:           177268 kB\n\
                         SwapCached:            0 kB\n\
                         Active:           104448 kB\n\
                         Inactive:          79084 kB\n\
                         Active(anon):       1524 kB\n\
                         Inactive(anon):        0 kB\n\
                         Active(file):     102924 kB\n\
                         Inactive(file):    79084 kB\n\
                         Unevictable:           0 kB\n\
                         Mlocked:               0 kB\n\
                         HighTotal:             0 kB\n\
                         HighFree:              0 kB\n\
                         LowTotal:         515352 kB\n\
                         LowFree:          317980 kB";

    const RAW_PARTIAL: &[u8] = b"MemTotal:         515352 kB\n\
                                 MemFree:          317980 kB\n";

    #[test]
    fn meminfo_parse() {
        let info = MemInfo::parse(RAW);
        assert_eq!(
            info.ok(),
            Some(MemInfo {
                total: Some(515352),
                free: Some(317980),
                available: Some(498232),
                low_free: Some(317980),
            })
        );
    }

    #[test]
    fn meminfo_getters() {
        let info = MemInfo::parse(RAW);
        assert!(info.is_ok());

        let info = info.unwrap();
        assert_eq!(info.total(), Some(515352));
        assert_eq!(info.free(), Some(317980));
        assert_eq!(info.available(), Some(498232));
        assert_eq!(info.low_free(), Some(317980));
    }

    #[test]
    fn meminfo_partial() {
        let info = MemInfo::parse(RAW_PARTIAL);
        assert!(info.is_ok());

        let info = info.unwrap();
        assert_eq!(
            info,
            MemInfo {
                total: Some(515352),
                free: Some(317980),
                available: None,
                low_free: None,
            }
        );

        assert_eq!(info.total(), Some(515352));
        assert_eq!(info.free(), Some(317980));
        assert_eq!(info.available(), None);
        assert_eq!(info.low_free(), None);
    }
}
