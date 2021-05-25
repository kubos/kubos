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

use crate::error::ProtocolError;
use libc::pid_t;
use nix::sys::signal;
use nix::unistd::Pid;
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
            io::ErrorKind::TimedOut => Err(ProtocolError::ReadTimeout),
            _ => Err(ProtocolError::ProcesssError {
                action: "reading".to_owned(),
                err,
            }),
        },
    }
}

/// Structure to handle lifetime and communications with child process
pub struct ProcessHandler {
    /// Handle to actual child process
    process: Child,
    /// Buffered timeout reader pointed to stdout pipe
    pub stdout_reader: Option<BufReader<TimeoutReader<ChildStdout>>>,
    /// Buffered timeout reader pointed to stderr pipe
    pub stderr_reader: Option<BufReader<TimeoutReader<ChildStderr>>>,
    /// Buffered timeout writer pointed to stdin pipe
    stdin_writer: Option<BufWriter<TimeoutWriter<ChildStdin>>>,
}

impl ProcessHandler {
    /// Spawn a process and setup handler structure
    ///
    /// # Arguments
    ///
    /// * command - Path to binary to execute
    /// * args - Optional arguments for binary
    ///
    /// # Examples
    ///
    /// ```
    /// use shell_protocol::*;
    ///
    /// let proc = ProcessHandler::spawn(&"/bin/bash".to_owned(), None);
    /// ```
    ///
    /// ```
    /// use shell_protocol::*;
    ///
    /// let proc = ProcessHandler::spawn(&"ls".to_owned(), Some(vec!["-l".to_owned()]));
    /// ```
    pub fn spawn(
        command: &str,
        args: Option<Vec<String>>,
    ) -> Result<ProcessHandler, ProtocolError> {
        let mut process = match Command::new(command.to_owned())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(args.unwrap_or_else(Vec::new))
            .spawn()
        {
            Ok(process) => process,
            Err(err) => {
                return Err(ProtocolError::SpawnError {
                    cmd: command.to_owned(),
                    err,
                });
            }
        };

        let stdout_reader = process
            .stdout
            .take()
            .map(|stdout| BufReader::new(TimeoutReader::new(stdout, Duration::from_millis(5))));

        let stderr_reader = process
            .stderr
            .take()
            .map(|stderr| BufReader::new(TimeoutReader::new(stderr, Duration::from_millis(5))));

        let stdin_writer = process
            .stdin
            .take()
            .map(|stdin| BufWriter::new(TimeoutWriter::new(stdin, Duration::from_millis(5))));

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
    ///
    /// # Examples
    ///
    /// ```
    /// use shell_protocol::*;
    ///
    /// let mut proc = ProcessHandler::spawn(&"ls".to_owned(), None).unwrap();
    /// match proc.read_stdout() {
    ///     Ok(Some(output)) => println!("Stdout: {}", output),
    ///     Ok(None) => println!("Stdout time out"),
    ///     Err(e) => eprintln!("Stdout err {}", e),
    /// }
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use shell_protocol::*;
    ///
    /// let mut proc = ProcessHandler::spawn(&"ls".to_owned(), None).unwrap();
    /// match proc.read_stderr() {
    ///     Ok(Some(output)) => println!("Stderr: {}", output),
    ///     Ok(None) => println!("Stderr time out"),
    ///     Err(e) => eprintln!("Stderr err {}", e),
    /// }
    /// ```
    pub fn read_stderr(&mut self) -> Result<Option<String>, ProtocolError> {
        match self.stderr_reader {
            Some(ref mut stderr_reader) => Ok(do_read(stderr_reader)?),
            None => Ok(None),
        }
    }

    /// Attempt to write to stdin
    ///
    /// # Arguments
    ///
    /// * data - Slice of bytes to write
    ///
    /// # Examples
    ///
    /// ```
    /// use shell_protocol::*;
    ///
    /// let cmd = "ls\n".as_bytes();
    /// let mut proc = ProcessHandler::spawn(&"/bin/bash".to_owned(), None).unwrap();
    /// match proc.write_stdin(&cmd) {
    ///     Ok(()) => println!("Stdin write success"),
    ///     Err(e) => eprintln!("Stdin err {}", e),
    /// }
    /// ```
    pub fn write_stdin(&mut self, data: &[u8]) -> Result<(), ProtocolError> {
        match self.stdin_writer {
            Some(ref mut stdin_writer) => {
                stdin_writer
                    .write_all(data)
                    .map_err(|err| ProtocolError::ProcesssError {
                        action: "write to stdin".to_owned(),
                        err,
                    })?;
                stdin_writer
                    .flush()
                    .map_err(|err| ProtocolError::ProcesssError {
                        action: "flush stdin".to_owned(),
                        err,
                    })?;
                Ok(())
            }
            None => Ok(()),
        }
    }

    /// Close process' stdin pipe
    ///
    /// # Examples
    ///
    /// ```
    /// use shell_protocol::*;
    ///
    /// let mut proc = ProcessHandler::spawn(&"/bin/bash".to_owned(), None).unwrap();
    /// match proc.close_stdin() {
    ///     Ok(()) => println!("Stdin closed"),
    ///     Err(e) => eprintln!("Stdin close err {}", e),
    /// }
    /// ```
    pub fn close_stdin(&mut self) -> Result<(), ProtocolError> {
        self.stdin_writer = None;
        Ok(())
    }

    /// Retrieve ID of process
    ///
    /// # Examples
    ///
    /// ```
    /// use shell_protocol::*;
    ///
    /// let proc = ProcessHandler::spawn(&"/bin/bash".to_owned(), None).unwrap();
    /// let pid = proc.id();
    /// ```
    pub fn id(&self) -> u32 {
        self.process.id()
    }

    /// Check to see if a process has exited and if the exit
    /// status is available
    ///
    /// # Examples
    ///
    /// ```
    /// use shell_protocol::*;
    ///
    /// let mut proc = ProcessHandler::spawn(&"/bin/bash".to_owned(), None).unwrap();
    /// match proc.status() {
    ///     Ok(Some((code, signal))) => println!("Process has exited: {}, {}", code, signal),
    ///     Ok(None) => println!("Process has not exited"),
    ///     Err(e) => eprintln!("Error getting process status {}", e)
    /// }
    /// ```
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

    /// Send killing signal to process
    ///
    /// # Arguments
    ///
    /// * signal - Optional signal to send to process
    ///
    /// # Examples
    ///
    /// ```
    /// use shell_protocol::*;
    ///
    /// let mut proc = ProcessHandler::spawn(&"/bin/bash".to_owned(), None).unwrap();
    /// match proc.kill(None) {
    ///     Ok(()) => println!("Process killed"),
    ///     Err(e) => eprintln!("Error killing process: {}", e),
    /// }
    /// ```
    pub fn kill(&mut self, signal: Option<u32>) -> Result<(), ProtocolError> {
        let pid = Pid::from_raw(self.process.id() as pid_t);
        let sig = signal::Signal::from_c_int(signal.unwrap_or(9) as i32)
            .unwrap_or(signal::Signal::SIGKILL);
        signal::kill(pid, sig).map_err(|err| ProtocolError::KillError { err })
    }
}
