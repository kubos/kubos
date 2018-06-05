/*
 * Copyright (C) 2018 Kubos Corporation
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

use byteorder::{LittleEndian, WriteBytesExt};
use crc32::*;
use messages::*;
use nom;
use rust_uart::UartError;
use rust_uart::*;
use serial;
use std::sync::mpsc::{Receiver, RecvTimeoutError, SyncSender, TrySendError};
use std::sync::{Arc, Mutex};
use std::time::Duration;

const CHAR_SIZE: serial::CharSize = serial::Bits8;
const PARITY: serial::Parity = serial::ParityNone;
const STOP_BITS: serial::StopBits = serial::Stop1;
const FLOW_CONTROL: serial::FlowControl = serial::FlowNone;
const TIMEOUT: Duration = Duration::from_millis(60);

/// Continually read messages from the OEM6 device
///
/// Messages will either be a response to a previously sent command,
/// or a log message. The function will detect the type and then forward
/// the message to the appropriate channel receiver.
///
/// # Arguments
///
/// * rx_conn - UART connection stream to use for communication
/// * log_send - SyncSender for forwarding received log messages
/// * response_send - SyncSender for forwarding recieved response messages
///
/// # Examples
///
/// ```
/// use novatel_oem6_api::*;
/// use std::thread;
/// use std::sync::mpsc::sync_channel;
///
/// # fn func() -> OEMResult<()> {
/// let bus = "/dev/ttyS5";
///
/// let (log_send, log_recv) = sync_channel(5);
/// let (response_send, response_recv) = sync_channel(5);
///
/// let oem = OEM6::new(bus, BaudRate::Baud9600, log_recv, response_recv).unwrap();
///
/// let rx_conn = oem.conn.clone();
///
/// thread::spawn(move || read_thread(rx_conn, log_send, response_send));
/// # Ok(())
/// # }
/// ```
///

pub fn read_thread(
    rx_conn: Arc<Mutex<Connection>>,
    log_send: SyncSender<(Header, Vec<u8>)>,
    response_send: SyncSender<(Header, Vec<u8>)>,
) {
    let mut log_err = false;
    let mut response_err = false;

    loop {
        {
            // Give any writes the chance to grab the lock first
            ::std::thread::sleep(Duration::from_millis(1));

            // Take the stream connection mutex
            // If the lock() call fails, it means that a different thread poisoned
            // the mutex. We want to maintain our ability to read messages from the
            // device for as long as possible, so we'll go ahead and just ignore the
            // poisoned status. Ideally, the master thread will have detected whatever
            // error caused the problem and will take error handling measures.
            let conn = rx_conn.lock().unwrap_or_else(|err| err.into_inner());

            // Read SYNC bytes
            let mut message = match conn.read(3, Duration::from_millis(250)) {
                Ok(v) => v,
                Err(err) => match err {
                    #[cfg(test)]
                    UartError::GenericError => continue,
                    UartError::IoError {
                        cause: ::std::io::ErrorKind::TimedOut,
                        description: _,
                    } => continue,
                    _ => panic!(err),
                },
            };

            if message != SYNC {
                continue;
            }

            // Read the rest of the header
            let mut hdr = match conn.read(25, TIMEOUT) {
                Ok(v) => v,
                Err(err) => match err {
                    UartError::IoError {
                        cause: ::std::io::ErrorKind::TimedOut,
                        description: _,
                    } => continue,
                    _ => panic!(err),
                },
            };
            message.append(&mut hdr);

            let hdr = match Header::parse(&message) {
                Some(v) => v,
                None => {
                    continue;
                }
            };

            // Read body + CRC bytes
            let mut body = match conn.read((hdr.msg_len + 4) as usize, TIMEOUT) {
                Ok(v) => v,
                Err(err) => match err {
                    UartError::IoError {
                        cause: ::std::io::ErrorKind::TimedOut,
                        description: _,
                    } => continue,
                    _ => panic!(err),
                },
            };
            message.append(&mut body);

            let len = message.len();

            // Read CRC
            let crc = nom::le_u32(message.split_off(len - 4).as_slice())
                .unwrap()
                .1;

            // Verify CRC
            let calc = calc_crc(&message);
            if calc != crc {
                continue;
            }

            let body = message.split_off(HDR_LEN.into());

            match hdr.msg_type & 0x80 == 0x80 {
                true => response_send
                    .try_send((hdr, body))
                    .or_else::<TrySendError<(Header, Vec<u8>)>, _>(|err| match err {
                        // Our buffer is full, but the receiver should still be alive, so let's keep going
                        TrySendError::Full(_) => Ok(()),
                        // The receiver is gone. If both receivers are gone, then there's no point in this
                        // loop still trying
                        TrySendError::Disconnected(_) => {
                            if log_err {
                                panic!("Both message receivers have disconnected")
                            }
                            response_err = true;
                            Ok(())
                        }
                    })
                    .unwrap(),
                false => log_send
                    .try_send((hdr, body))
                    .or_else::<TrySendError<(Header, Vec<u8>)>, _>(|err| match err {
                        TrySendError::Full(_) => Ok(()),
                        TrySendError::Disconnected(_) => {
                            if response_err {
                                panic!("Both message receivers have disconnected")
                            }
                            log_err = true;
                            Ok(())
                        }
                    })
                    .unwrap(),
            };
        }
    }
}

/// Structure for OEM6 device instance
#[derive(Clone)]
pub struct OEM6 {
    /// Device connection structure
    pub conn: Arc<Mutex<Connection>>,
    /// Channel for receiving log messages
    pub log_recv: Arc<Mutex<Receiver<(Header, Vec<u8>)>>>,
    /// Channel for receiveing response messages
    pub response_recv: Arc<Mutex<Receiver<(Header, Vec<u8>)>>>,
}

impl OEM6 {
    /// Constructor for OEM6 structure
    ///
    /// # Arguments
    ///
    /// * bus - Serial bus to use for communication
    /// * baud_rate - Communication data rate
    /// * log_recv - Receiver for log messages sent by read thread
    /// * response_recv - Receiver for response messages sent by read thread
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`OEMError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use novatel_oem6_api::*;
    /// use std::sync::mpsc::sync_channel;
    ///
    /// # fn func() -> OEMResult<()> {
    /// let bus = "/dev/ttyS5";
    ///
    /// let (log_send, log_recv) = sync_channel(5);
    /// let (response_send, response_recv) = sync_channel(5);
    ///
    /// let oem = OEM6::new(bus, BaudRate::Baud9600, log_recv, response_recv).unwrap();
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`OEMError`]: enum.OEMError.html
    pub fn new(
        bus: &str,
        baud_rate: serial::BaudRate,
        log_recv: Receiver<(Header, Vec<u8>)>,
        response_recv: Receiver<(Header, Vec<u8>)>,
    ) -> OEMResult<OEM6> {
        let settings = serial::PortSettings {
            baud_rate,
            char_size: CHAR_SIZE,
            parity: PARITY,
            stop_bits: STOP_BITS,
            flow_control: FLOW_CONTROL,
        };

        let conn = Arc::new(Mutex::new(Connection::from_path(bus, settings, TIMEOUT)?));

        Ok(OEM6 {
            conn,
            log_recv: Arc::new(Mutex::new(log_recv)),
            response_recv: Arc::new(Mutex::new(response_recv)),
        })
    }

    /// Request the system version information
    ///
    /// Note: A subsequent [`get_log()`] call is required to fetch the information
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`OEMError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use novatel_oem6_api::*;
    /// # use std::thread;
    /// # use std::sync::mpsc::sync_channel;
    ///
    /// # fn func() -> OEMResult<()> {
    /// # let bus = "/dev/ttyS5";
    /// # let (log_send, log_recv) = sync_channel(5);
    /// # let (response_send, response_recv) = sync_channel(5);
    /// let oem = OEM6::new(bus, BaudRate::Baud9600, log_recv, response_recv).unwrap();
    /// let rx_conn = oem.conn.clone();
    /// thread::spawn(move || read_thread(rx_conn, log_send, response_send));
    ///
    /// oem.request_version()?;
    ///
    /// // Read the next log message, which should have the reply
    /// let entry = oem.get_log()?;
    ///
    /// match entry {
    ///     Log::Version(log) => {
    ///         println!("Version Info ({}):\n", log.num_components);
    ///         for component in log.components.iter() {
    ///             println!(
    ///                 "Type: {} Model: {} SN: {}",
    ///                 component.comp_type, component.model, component.serial_num
    ///             );
    ///             println!("    HW Version: {}", component.hw_version);
    ///             println!("    SW Version: {}", component.sw_version);
    ///             println!("    Boot Version: {}", component.boot_version);
    ///             println!(
    ///                 "    Compiled: {} {}",
    ///                 component.compile_date, component.compile_time
    ///             );
    ///             println!("");
    ///         }
    ///     }
    ///     _ => {},
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`get_log()`]: method.get_log.html
    /// [`OEMError`]: enum.OEMError.html
    pub fn request_version(&self) -> OEMResult<()> {
        let request = LogCmd::new(
            Port::COM1 as u32,
            MessageID::Version as u16,
            LogTrigger::Once,
            0.0,
            0.0,
            false,
        );

        self.send_message(request)
            .and_then(|_| self.get_response(MessageID::Log))
    }

    /// Request BestXYZ position log/s from the device
    ///
    /// Note: Subsequent [`get_log()`] calls are required to fetch the information
    ///
    /// # Arguments
    ///
    /// * interval - Frequency, in seconds, at which the OEM6 should emit position log messages
    /// * offset - Offset, in seconds, of the message emit frequency
    /// * hold - Whether the [`unlog_all`] command should be able to apply to this log. A value
    ///          of `true` will prevent [`unlog_all`] from applying to this log.
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`OEMError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use novatel_oem6_api::*;
    /// # use std::thread;
    /// # use std::sync::mpsc::sync_channel;
    ///
    /// # fn func() -> OEMResult<()> {
    /// # let bus = "/dev/ttyS5";
    /// # let (log_send, log_recv) = sync_channel(5);
    /// # let (response_send, response_recv) = sync_channel(5);
    /// let oem = OEM6::new(bus, BaudRate::Baud9600, log_recv, response_recv).unwrap();
    /// let rx_conn = oem.conn.clone();
    /// thread::spawn(move || read_thread(rx_conn, log_send, response_send));
    ///
    /// oem.request_position(1.0, 0.0, false)?;
    ///
    /// // Continually read the position log messages
    /// loop {
    ///     // Read the next log message, which should have the reply
    ///     let entry = oem.get_log()?;
    ///
    ///     match entry {
    ///         Log::BestXYZ(log) => {
    ///             println!("Best XYZ Data:");
    ///             println!("    Position: {:?}", log.position);
    ///             println!("    Velocity: {:?}", log.velocity);
    ///         }
    ///         _ => {},
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`get_log()`]: method.get_log.html
    /// [`unlog_all`]: method.unlog_all.html
    /// [`OEMError`]: enum.OEMError.html
    pub fn request_position(&self, interval: f64, offset: f64, hold: bool) -> OEMResult<()> {
        let trigger = if interval == 0.0 {
            LogTrigger::Once
        } else {
            LogTrigger::OnTime
        };

        let request = LogCmd::new(
            Port::COM1 as u32,
            MessageID::BestXYZ as u16,
            trigger,
            interval,
            offset,
            hold,
        );

        self.send_message(request)
            .and_then(|_| self.get_response(MessageID::Log))
    }

    /// Request that the device send error messages as they occur
    ///
    /// # Arguments
    ///
    /// * hold - Whether the [`unlog_all`] command should be able to apply to this log. A value
    ///          of `true` will prevent [`unlog_all`] from applying to this log.
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`OEMError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use novatel_oem6_api::*;
    /// # use std::thread;
    /// # use std::sync::mpsc::sync_channel;
    ///
    /// # fn func() -> OEMResult<()> {
    /// # let bus = "/dev/ttyS5";
    /// # let (log_send, log_recv) = sync_channel(5);
    /// # let (response_send, response_recv) = sync_channel(5);
    /// let oem = OEM6::new(bus, BaudRate::Baud9600, log_recv, response_recv).unwrap();
    ///
    /// oem.request_errors(false)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`unlog_all`]: method.unlog_all.html
    /// [`OEMError`]: enum.OEMError.html
    pub fn request_errors(&self, hold: bool) -> OEMResult<()> {
        let request = LogCmd::new(
            Port::COM1 as u32,
            MessageID::RxStatusEvent as u16,
            LogTrigger::OnChanged,
            0.0,
            0.0,
            hold,
        );

        self.send_message(request)
            .and_then(|_| self.get_response(MessageID::Log))
    }

    /// Request that automatic logging for a particular log type be stopped
    ///
    /// # Arguments
    ///
    /// * id - Message ID which should no longer be logged
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`OEMError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use novatel_oem6_api::*;
    /// # use std::thread;
    /// # use std::sync::mpsc::sync_channel;
    ///
    /// # fn func() -> OEMResult<()> {
    /// # let bus = "/dev/ttyS5";
    /// # let (log_send, log_recv) = sync_channel(5);
    /// # let (response_send, response_recv) = sync_channel(5);
    /// let oem = OEM6::new(bus, BaudRate::Baud9600, log_recv, response_recv).unwrap();
    ///
    /// oem.request_unlog(MessageID::BestXYZ)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`OEMError`]: enum.OEMError.html
    pub fn request_unlog(&self, id: MessageID) -> OEMResult<()> {
        let request = UnlogCmd::new(Port::COM1 as u32, id as u16);

        self.send_message(request)
            .and_then(|_| self.get_response(MessageID::Unlog))
    }

    /// Request that all automatic logging be stopped
    ///
    /// # Arguments
    ///
    /// * clear_holds - Specifies whether log messages which were set with the `hold` option
    ///                 should also be unlogged
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`OEMError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use novatel_oem6_api::*;
    /// # use std::thread;
    /// # use std::sync::mpsc::sync_channel;
    ///
    /// # fn func() -> OEMResult<()> {
    /// # let bus = "/dev/ttyS5";
    /// # let (log_send, log_recv) = sync_channel(5);
    /// # let (response_send, response_recv) = sync_channel(5);
    /// let oem = OEM6::new(bus, BaudRate::Baud9600, log_recv, response_recv).unwrap();
    ///
    /// // Request position information every second and prevent unlogging with unlog_all()
    /// oem.request_position(1.0, 0.0, true)?;
    ///
    /// // Unlog everything. Previous request for position information will remain intact
    /// oem.request_unlog_all(false)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ```
    /// # use novatel_oem6_api::*;
    /// # use std::thread;
    /// # use std::sync::mpsc::sync_channel;
    ///
    /// # fn func() -> OEMResult<()> {
    /// # let bus = "/dev/ttyS5";
    /// # let (log_send, log_recv) = sync_channel(5);
    /// # let (response_send, response_recv) = sync_channel(5);
    /// let oem = OEM6::new(bus, BaudRate::Baud9600, log_recv, response_recv).unwrap();
    ///
    /// // Request position information every second and prevent unlogging with unlog_all()
    /// oem.request_position(1.0, 0.0, true)?;
    ///
    /// // Unlog everything. This will override the position request's `hold=true` value and
    /// // also unlog that request
    /// oem.request_unlog_all(true)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`OEMError`]: enum.OEMError.html
    pub fn request_unlog_all(&self, clear_holds: bool) -> OEMResult<()> {
        let request = UnlogAllCmd::new(Port::COM1 as u32, clear_holds);

        self.send_message(request)
            .and_then(|_| self.get_response(MessageID::UnlogAll))
    }

    /// Directly send a message without formatting or checksum calculation
    ///
    /// Note: The message will not be verified by checking for a command response
    ///
    /// # Arguments
    ///
    /// * msg - Message to send
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`OEMError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use novatel_oem6_api::*;
    /// # use std::thread;
    /// # use std::sync::mpsc::sync_channel;
    ///
    /// # fn func() -> OEMResult<()> {
    /// # let bus = "/dev/ttyS5";
    /// # let (log_send, log_recv) = sync_channel(5);
    /// # let (response_send, response_recv) = sync_channel(5);
    /// let oem = OEM6::new(bus, BaudRate::Baud9600, log_recv, response_recv).unwrap();
    ///
    /// let packet: [u8; 6] = [0, 1, 2, 3, 4, 5];
    /// oem.passthrough(&packet)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    ///
    /// [`OEMError`]: enum.OEMError.html
    pub fn passthrough(&self, msg: &[u8]) -> OEMResult<()> {
        // If the mutex has been poisoned by the read thread,
        // go ahead and attempt the write anyways, to try to
        // preserve functionality, but inform the caller afterwards
        match self.conn.lock() {
            Ok(conn) => conn.write(msg).map_err(|err| err.into()),
            Err(conn) => conn.into_inner()
                .write(msg)
                .map_err(|err| err.into())
                .and(Err(OEMError::ThreadCommError)),
        }
    }

    fn send_message<T: Message>(&self, msg: T) -> OEMResult<()> {
        let mut raw = msg.serialize();

        // Get the calculated CRC
        let crc = calc_crc(&raw);
        raw.write_u32::<LittleEndian>(crc).unwrap();

        // If the mutex has been poisoned by the read thread,
        // go ahead and attempt the write anyways, to try to
        // preserve functionality, but inform the caller afterwards
        match self.conn.lock() {
            Ok(conn) => conn.write(raw.as_slice()).map_err(|err| err.into()),
            Err(conn) => conn.into_inner()
                .write(raw.as_slice())
                .map_err(|err| err.into())
                .and(Err(OEMError::ThreadCommError)),
        }
    }

    fn get_response(&self, id: MessageID) -> OEMResult<()> {
        let (hdr, body) = self.response_recv
            .lock()
            .map_err(|_| OEMError::MutexError)?
            .recv_timeout(Duration::from_millis(500))
            .map_err(|_| OEMError::NoResponse)?;

        // Make sure we got specifically a response message
        if hdr.msg_type & 0x80 != 0x80 {
            throw!(OEMError::NoResponse);
        }

        let resp = match Response::new(body) {
            Some(v) => v,
            None => {
                throw!(OEMError::NoResponse);
            }
        };

        if hdr.msg_id != id {
            throw!(OEMError::ResponseMismatch);
        }

        if resp.resp_id != ResponseID::Ok {
            throw!(OEMError::CommandError {
                id: resp.resp_id,
                description: resp.resp_string.clone(),
            });
        }

        Ok(())
    }

    /// Fetch a log message from the OEM6 read thread
    ///
    /// # Errors
    ///
    /// If this function encounters any errors, an [`OEMError`] variant will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use novatel_oem6_api::*;
    /// # use std::thread;
    /// # use std::sync::mpsc::sync_channel;
    ///
    /// # fn func() -> OEMResult<()> {
    /// # let bus = "/dev/ttyS5";
    /// # let (log_send, log_recv) = sync_channel(5);
    /// # let (response_send, response_recv) = sync_channel(5);
    /// let oem = OEM6::new(bus, BaudRate::Baud9600, log_recv, response_recv).unwrap();
    /// let rx_conn = oem.conn.clone();
    /// thread::spawn(move || read_thread(rx_conn, log_send, response_send));
    ///
    /// let entry = oem.get_log()?;
    ///
    /// match entry {
    ///     Log::Version(log) => println!("Received version information: {:?}", log),
    ///     Log::BestXYZ(log) =>  println!("Received position information: {:?}", log),
    ///     Log::RxStatusEvent(log) =>  println!("Received system event: {:?}", log),
    ///     _ => println!("Received unknown log type"),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`OEMError`]: enum.OEMError.html
    pub fn get_log(&self) -> OEMResult<Log> {
        loop {
            let (hdr, body) = match self.log_recv
                .lock()
                .map_err(|_| OEMError::MutexError)?
                .recv_timeout(Duration::from_secs(5))
            {
                Ok(v) => v,
                Err(RecvTimeoutError::Timeout) => continue,
                Err(RecvTimeoutError::Disconnected) => throw!(OEMError::ThreadCommError),
            };

            // Make sure it's not a response message
            if hdr.msg_type & 0x80 == 0x80 {
                continue;
            }

            match Log::new(
                hdr.msg_id,
                hdr.recv_status,
                hdr.time_status,
                hdr.week,
                hdr.ms,
                body,
            ) {
                Some(v) => return Ok(v),
                None => {
                    continue;
                }
            };
        }
    }
}

/// Common Error for OEM Actions
#[derive(Fail, Display, Debug, Clone, PartialEq)]
pub enum OEMError {
    /// Catch-all error
    #[display(fmt = "Generic Error")]
    GenericError,
    /// An issue occurred while attempted to obtain a mutex lock
    #[display(fmt = "Mutex Error")]
    MutexError,
    /// A response message was received, but the ID doesn't match the command that was sent
    #[display(fmt = "Response ID Mismatch")]
    ResponseMismatch,
    /// A command was sent, but we were unable to get the response
    #[display(fmt = "Failed to get command response")]
    NoResponse,
    /// The thread reading messages from the device is no longer working
    #[display(fmt = "Failed to communicate with read thread")]
    ThreadCommError,
    /// A response was recieved and indicates an error with the previously sent command
    #[display(fmt = "Command Error({:?}): {}", id, description)]
    CommandError {
        /// The underlying error
        id: ResponseID,
        /// Description of error encountered
        description: String,
    },
    /// Received a valid message, but the message ID doesn't match any known message type
    #[display(fmt = "Unknown Message Received: {:X}", id)]
    UnknownMessage {
        /// ID of message received
        id: u16,
    },
    /// An error was thrown by the serial communication driver
    #[display(fmt = "UART Error")]
    UartError {
        /// The underlying error
        #[fail(cause)]
        cause: UartError,
    },
}

impl From<UartError> for OEMError {
    fn from(error: UartError) -> Self {
        OEMError::UartError { cause: error }
    }
}

/// Custom error type for OEM6 operations.
pub type OEMResult<T> = Result<T, OEMError>;
