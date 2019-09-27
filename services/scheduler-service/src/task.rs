/*
 * Copyright (C) 2019 Kubos Corporation
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

//!
//! Definitions and functions for dealing with tasks & scheduling
//!

use crate::app::ScheduleApp;
use chrono::offset::TimeZone;
use chrono::Utc;
use juniper::GraphQLObject;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::time::Instant;
use tokio::prelude::*;
use tokio::timer::Delay;
use tokio::timer::Interval;

// Configuration used to schedule app execution
#[derive(Clone, Debug, GraphQLObject, Serialize, Deserialize)]
pub struct ScheduleTask {
    // Descriptive name of task
    pub name: String,
    // Start delay specified in Xh Ym Zs format
    // Used by init and recurring tasks
    pub delay: Option<String>,
    // Start time specified in yyyy-mm-dd hh:mm:ss format
    // Used by onetime tasks
    pub time: Option<String>,
    // Period of recurrence specified in Xh Ym Zs format
    // Used by recurring tasks
    pub period: Option<String>,
    // Details of the app to be executed
    pub app: ScheduleApp,
}

impl ScheduleTask {
    // Parse timer delay duration from either delay or time fields
    pub fn get_duration(&self) -> Result<Duration, String> {
        if let Some(delay) = &self.delay {
            Ok(parse_hms_field(delay.to_owned())?)
        } else if let Some(time) = &self.time {
            let run_time = Utc
                .datetime_from_str(&time, "%Y-%m-%d %H:%M:%S")
                .map_err(|e| format!("Failed to parse one_time time: {:?}", e))?;
            let now = chrono::Utc::now();

            if run_time < now {
                Err("Task scheduled for past time".to_owned())
            } else {
                Ok((run_time - now)
                    .to_std()
                    .map_err(|e| format!("Failed to create time diff: {}", e))?)
            }
        } else {
            Err("No delay defined for task".to_owned())
        }
    }

    pub fn get_period(&self) -> Result<Option<Duration>, String> {
        if let Some(period) = &self.period {
            Ok(Some(parse_hms_field(period.to_owned())?))
        } else {
            Ok(None)
        }
    }

    pub fn schedule(&self, service_url: String) -> Box<dyn Future<Item = (), Error = ()> + Send> {
        let duration = match self.get_duration() {
            Ok(d) => d,
            Err(e) => {
                error!("Failed to parse task delay duration: {}", e);
                return Box::new(future::err::<(), ()>(()));
            }
        };

        let when = Instant::now() + duration;
        let period = self.get_period();
        let app = self.app.clone();

        match period {
            Ok(Some(period)) => Box::new(
                Interval::new(when, period)
                    .for_each(move |instant| {
                        info!("fire; instant={:?}", instant);
                        app.execute(&service_url.clone());
                        Ok(())
                    })
                    .map_err(|e| {
                        error!("Recurring task interval errored: {}", e);
                        panic!("Recurring task interval errored: {}", e)
                    }),
            ),
            _ => Box::new(
                Delay::new(when)
                    .and_then(move |_| {
                        app.execute(&service_url);
                        Ok(())
                    })
                    .map_err(|e| {
                        error!("Task delay errored; err={:?}", e);
                        panic!("Task delay errored; err={:?}", e)
                    }),
            ),
        }
    }
}

fn parse_hms_field(field: String) -> Result<Duration, String> {
    let field_parts: Vec<String> = field.split(' ').map(|s| s.to_owned()).collect();
    let mut duration: u64 = 0;
    for mut part in field_parts {
        let unit: Option<char> = part.pop();
        let num: Result<u64, _> = part.parse();
        if let Ok(num) = num {
            match unit {
                Some('s') => {
                    duration += num;
                }
                Some('m') => {
                    duration += num * 60;
                }
                Some('h') => {
                    duration += num * 60 * 60;
                }
                _ => {
                    error!("Failed to parse field for duration");
                    return Err("Failed to parse field for duration".to_owned());
                }
            }
        }
    }
    Ok(Duration::from_secs(duration))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_seconds() {
        assert_eq!(
            parse_hms_field("21s".to_owned()),
            Ok(Duration::from_secs(21))
        );
    }

    #[test]
    fn test_parse_minutes() {
        assert_eq!(
            parse_hms_field("3m".to_owned()),
            Ok(Duration::from_secs(180))
        );
    }

    #[test]
    fn test_parse_hours() {
        assert_eq!(
            parse_hms_field("2h".to_owned()),
            Ok(Duration::from_secs(7200))
        );
    }

    #[test]
    fn test_parse_minutes_seconds() {
        assert_eq!(
            parse_hms_field("1m 1s".to_owned()),
            Ok(Duration::from_secs(61))
        );
    }

    #[test]
    fn test_parse_hours_minutes() {
        assert_eq!(
            parse_hms_field("3h 10m".to_owned()),
            Ok(Duration::from_secs(11400))
        );
    }

    #[test]
    fn test_parse_hours_seconds() {
        assert_eq!(
            parse_hms_field("5h 44s".to_owned()),
            Ok(Duration::from_secs(18044))
        );
    }

    #[test]
    fn test_parse_hours_minutes_seconds() {
        assert_eq!(
            parse_hms_field("2h 2m 2s".to_owned()),
            Ok(Duration::from_secs(7322))
        );
    }
}
