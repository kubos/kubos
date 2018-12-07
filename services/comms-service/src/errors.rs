use failure::Error;

/// This enum defines all errors that can occur within the `comms-service`.
#[derive(Fail, Debug, PartialEq)]
pub enum CommsServiceError {
    /// The mutex guarding the telemetry cache has been poisoned.
    #[fail(display = "The mutex guarding the telemetry cache has been poisoned.")]
    MutexPoisoned,
    /// A UDP header was unable to be correctly parsed.
    #[fail(display = "A UDP header was unable to be correctly parsed.")]
    HeaderParsing,
    /// The length of a UDP packet does not match the length found in the header.
    #[fail(display = "The length of a UDP packet does not match the length found in the header.")]
    InvalidPacketLength,
    /// The checksum of a UDP packet does not match the one found in the header.
    #[fail(display = "The checksum of a UDP packet does not match the one found in the header.")]
    InvalidChecksum,
    /// The number of `write` methods and the number of downlink ports are not the same.
    #[fail(
        display = "The number of write methods and the number of downlink ports are not the same."
    )]
    ParameterLengthMismatch,
    /// The read thread could not be started because a no `write()` method was specified.
    #[fail(
        display = "The read thread could not be started because no write method was specified."
    )]
    MissingWriteMethod,
}

/// Result returned by the `comms-service`.
pub type CommsResult<T> = Result<T, Error>;
