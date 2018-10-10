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

use error::ProtocolError;
use std::io::{BufRead, BufReader};
use std::process::{Child, ChildStderr, ChildStdout, Command, Stdio};

pub struct ProcessHandler {
    _process: Child,
    pub stdout_reader: Option<BufReader<ChildStdout>>,
    pub stderr_reader: Option<BufReader<ChildStderr>>,
}

impl ProcessHandler {
    pub fn spawn(
        command: String,
        args: Option<Vec<String>>,
    ) -> Result<ProcessHandler, ProtocolError> {
        let mut process = match Command::new(command.to_owned())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(args.unwrap_or(vec![]))
            .spawn()
        {
            Ok(process) => process,
            Err(err) => return Err(ProtocolError::SpawnError { cmd: command, err }),
        };

        let stdout_reader = match process.stdout.take() {
            Some(stdout) => Some(BufReader::new(stdout)),
            None => None,
        };

        let stderr_reader = match process.stderr.take() {
            Some(stderr) => Some(BufReader::new(stderr)),
            None => None,
        };

        Ok(ProcessHandler {
            _process: process,
            stdout_reader,
            stderr_reader,
        })
    }

    pub fn read_stdout(&mut self) -> Option<String> {
        match self.stdout_reader {
            Some(ref mut stdout_reader) => {
                let mut data = String::new();
                match stdout_reader.read_line(&mut data) {
                    Ok(0) => None,
                    Err(e) => {
                        warn!("stdout err {:?}", e);
                        None
                    }
                    Ok(_) => Some(data),
                }
            }
            None => None,
        }
    }

    pub fn read_stderr(&mut self) -> Option<String> {
        match self.stderr_reader {
            Some(ref mut stderr_reader) => {
                let mut data = String::new();
                match stderr_reader.read_line(&mut data) {
                    Ok(0) => None,
                    Err(e) => {
                        warn!("stderr err {:?}", e);
                        None
                    }
                    Ok(_) => Some(data),
                }
            }
            None => None,
        }
    }
}
