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
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::os::unix::prelude::*;
use std::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command, Stdio};
use std::time::Duration;
use timeout_readwrite::{TimeoutReader, TimeoutWriter};

// Helper function for reading a line from a BufReader
fn do_read<R: BufRead>(mut reader: R) -> Result<Option<String>, ProtocolError> {
    let mut data = String::new();
    match reader.read_line(&mut data) {
        Ok(0) => Ok(None),
        Ok(_) => Ok(Some(data)),
        Err(err) => match err.kind() {
            io::ErrorKind::TimedOut => return Err(ProtocolError::ReadTimeout),
            _ => {
                return Err(ProtocolError::ProcesssError {
                    action: "reading".to_owned(),
                    err,
                });
            }
        },
    }
}

pub struct ProcessHandler {
    process: Child,
    pub stdout_reader: Option<BufReader<TimeoutReader<ChildStdout>>>,
    pub stderr_reader: Option<BufReader<TimeoutReader<ChildStderr>>>,
    pub stdin_writer: Option<BufWriter<TimeoutWriter<ChildStdin>>>,
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
            Some(stdout) => Some(BufReader::new(TimeoutReader::new(
                stdout,
                Duration::from_millis(5),
            ))),
            None => None,
        };

        let stderr_reader = match process.stderr.take() {
            Some(stderr) => Some(BufReader::new(TimeoutReader::new(
                stderr,
                Duration::from_millis(5),
            ))),
            None => None,
        };

        let stdin_writer = match process.stdin.take() {
            Some(stdin) => Some(BufWriter::new(TimeoutWriter::new(
                stdin,
                Duration::from_millis(5),
            ))),
            None => None,
        };

        Ok(ProcessHandler {
            process,
            stdout_reader,
            stderr_reader,
            stdin_writer,
        })
    }

    /// Attempt to read from stdout
    ///
    /// A return value of `None` indicates the stream is
    /// no longer available and likewise the process
    /// is likely no longer alive.
    pub fn read_stdout(&mut self) -> Result<Option<String>, ProtocolError> {
        match self.stdout_reader {
            Some(ref mut stdout_reader) => Ok(do_read(stdout_reader)?),
            None => Ok(None),
        }
    }

    /// Attempt to read from stderr
    ///
    /// A return value of `None` indicates the stream is
    /// no longer available and likewise the process
    /// is likely no longer alive.
    pub fn read_stderr(&mut self) -> Result<Option<String>, ProtocolError> {
        match self.stderr_reader {
            Some(ref mut stderr_reader) => Ok(do_read(stderr_reader)?),
            None => Ok(None),
        }
    }

    /// Attempt to write to stdin
    pub fn write_stdin(&mut self, data: &[u8]) -> Result<(), ProtocolError> {
        match self.stdin_writer {
            Some(ref mut stdin_writer) => {
                stdin_writer
                    .write_all(data)
                    .map_err(|err| ProtocolError::ProcesssError {
                        action: "write to stdin".to_owned(),
                        err,
                    })?;
                stdin_writer.flush();
                Ok(())
            }
            None => Ok(()),
        }
    }

    /// Retrieve ID of process
    pub fn id(&self) -> Result<u32, ProtocolError> {
        Ok(self.process.id())
    }

    /// Check to see if a process has exited and if the exit
    /// status is available
    pub fn status(&mut self) -> Result<Option<(u32, u32)>, ProtocolError> {
        match self.process.try_wait() {
            Ok(Some(status)) => Ok(Some((
                status.code().unwrap_or(0) as u32,
                status.signal().unwrap_or(0) as u32,
            ))),
            Ok(None) => Ok(None),
            Err(err) => Err(ProtocolError::ProcesssError {
                action: "get exit status".to_owned(),
                err,
            }),
        }
    }
}
