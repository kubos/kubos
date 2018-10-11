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
    /// Spawn a process and setup stdout/stderr streams
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

    /// Attempt to read from stdout
    ///
    /// This will block as long as the process is running.
    /// A return value of `None` indicates the stream is
    /// no longer available and likewise the process
    /// is likely no longer alive.
    pub fn read_stdout(&mut self) -> Result<Option<String>, ProtocolError> {
        match self.stdout_reader {
            Some(ref mut stdout_reader) => {
                let mut data = String::new();
                match stdout_reader.read_line(&mut data) {
                    Ok(0) => Ok(None),
                    Ok(_) => Ok(Some(data)),
                    Err(err) => {
                        return Err(ProtocolError::ProcesssError {
                            action: "read stdout".to_owned(),
                            err,
                        })
                    }
                }
            }
            None => Ok(None),
        }
    }

    /// Attempt to read from stderr
    ///
    /// This will block as long as the process is running.
    /// A return value of `None` indicates the stream is
    /// no longer available and likewise the process
    /// is likely no longer alive.
    pub fn read_stderr(&mut self) -> Result<Option<String>, ProtocolError> {
        match self.stderr_reader {
            Some(ref mut stderr_reader) => {
                let mut data = String::new();
                match stderr_reader.read_line(&mut data) {
                    Ok(0) => Ok(None),
                    Ok(_) => Ok(Some(data)),
                    Err(err) => {
                        return Err(ProtocolError::ProcesssError {
                            action: "read stderr".to_owned(),
                            err,
                        })
                    }
                }
            }
            None => Ok(None),
        }
    }
}
