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

use crate::app::App;
use crate::error::SchedulerError;
use chrono::offset::TimeZone;
use chrono::Utc;
use juniper::GraphQLObject;
use log::error;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::time::Instant;
use tokio::prelude::*;
use tokio::timer::Delay;
use tokio::timer::Interval;

// Configuration used to schedule app execution
#[derive(Clone, Debug, GraphQLObject, Serialize, Deserialize)]
pub struct Task {
    // Description of task
    pub description: String,
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
    pub app: App,
}

impl Task {
    // Parse timer delay duration from either delay or time fields
    pub fn get_duration(&self) -> Result<Duration, SchedulerError> {
        if self.delay.is_some() && self.time.is_some() {
            return Err(SchedulerError::TaskParseError {
                err: "Both delay and time defined".to_owned(),
                description: self.description.to_owned(),
            });
        }
        if let Some(delay) = &self.delay {
            Ok(parse_hms_field(delay.to_owned())?)
        } else if let Some(time) = &self.time {
            let run_time = Utc
                .datetime_from_str(time, "%Y-%m-%d %H:%M:%S")
                .map_err(|e| SchedulerError::TaskParseError {
                    err: format!("Failed to parse time field '{}': {}", time, e),
                    description: self.description.to_owned(),
                })?;
            let now = chrono::Utc::now();

            if run_time < now {
                Err(SchedulerError::TaskTimeError {
                    err: format!("Task scheduled for past time: {}", time),
                    description: self.description.to_owned(),
                })
            } else if (run_time - now) > chrono::Duration::days(90) {
                Err(SchedulerError::TaskTimeError {
                    err: format!("Task scheduled beyond 90 days in the future: {}", time),
                    description: self.description.to_owned(),
                })
            } else {
                Ok((run_time - now)
                    .to_std()
                    .map_err(|e| SchedulerError::TaskParseError {
                        err: format!("Failed to calculate run time: {}", e),
                        description: self.description.to_owned(),
                    })?)
            }
        } else {
            Err(SchedulerError::TaskParseError {
                err: "No delay or time defined".to_owned(),
                description: self.description.to_owned(),
            })
        }
    }

    pub fn get_period(&self) -> Result<Option<Duration>, SchedulerError> {
        if let Some(period) = &self.period {
            Ok(Some(parse_hms_field(period.to_owned())?))
        } else {
            Ok(None)
        }
    }

    pub fn schedule(&self, service_url: String) -> Box<dyn Future<Item = (), Error = ()> + Send> {
        let name = self.app.name.to_owned();
        let duration = match self.get_duration() {
            Ok(d) => d,
            Err(e) => {
                error!(
                    "Failed to parse time specification for task '{}': {}",
                    name, e
                );
                return Box::new(future::err::<(), ()>(()));
            }
        };

        let when = Instant::now() + duration;
        let period = self.get_period();
        let app = self.app.clone();

        match period {
            Ok(Some(period)) => Box::new(
                Interval::new(when, period)
                    .for_each(move |_| {
                        app.execute(&service_url.clone());
                        Ok(())
                    })
                    .map_err(move |e| {
                        error!("Recurring interval errored for task '{}': {}", name, e);
                        panic!("Recurring interval errored for task '{}': {}", name, e)
                    }),
            ),
            _ => Box::new(
                Delay::new(when)
                    .and_then(move |_| {
                        app.execute(&service_url);
                        Ok(())
                    })
                    .map_err(move |e| {
                        error!("Delay errored for task '{}': {}", name, e);
                        panic!("Delay errored for task '{}': {}", name, e)
                    }),
            ),
        }
    }
}

fn parse_hms_field(field: String) -> Result<Duration, SchedulerError> {
    let field_parts: Vec<String> = field.split(' ').map(|s| s.to_owned()).collect();
    let mut duration: u64 = 0;
    if field_parts.is_empty() {
        return Err(SchedulerError::HmsParseError {
            err: "No parts found".to_owned(),
            field,
        });
    }
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
                    return Err(SchedulerError::HmsParseError {
                        err: "Found invalid unit".to_owned(),
                        field,
                    });
                }
            }
        } else {
            return Err(SchedulerError::HmsParseError {
                err: "Failed to parse number".to_owned(),
                field,
            });
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
