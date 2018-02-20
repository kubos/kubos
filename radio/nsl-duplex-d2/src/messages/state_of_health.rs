use nom::{IResult, be_u32, be_u8};

#[derive(Debug)]
pub struct StateOfHealth {
    reset_count: u32, // (4 byte integer) Current epoch reset count, starts at 0, incremented for each power system reset, persistent over the life of the mission
    current_time: u32, // (4 byte integer) Current time (seconds) from start of most recent reset
    current_rssi: u8, // (1 byte integer) Current RSSI (Received Signal Strength Indicator), 0 to 4
    connection_status: u8, // (1 byte integer) Connection status, 0 (connected) or 1 (disconnected)
    globalstar_gateway: u8, // (1 byte integer) Globalstar gateway connected to, proprietary ID, 0 to 255
    last_contact_time: u32, // (4 byte integer) Last contact time, seconds since latest reset
    last_attempt_time: u32, // (4 byte integer) Last attempt time, seconds since latest reset
    call_attempts_since_reset: u32, // (4 byte integer) Count of call attempts since latest reset
    successful_connects_since_reset: u32, // (4 byte integer) Count of successful connects since latest reset
    average_connection_duration: u32,     // (4 byte integer) Average connection duration (seconds)
    connection_duration_std_dev: u32, // (4 byte integer) Connection duration standard deviation (seconds)
}

impl StateOfHealth {
    pub fn parse(input: &[u8]) -> IResult<&[u8], StateOfHealth> {
        let (input, reset_count) = be_u32(input)?;
        let (input, current_time) = be_u32(input)?;
        let (input, current_rssi) = be_u8(input)?;
        let (input, connection_status) = be_u8(input)?;
        let (input, globalstar_gateway) = be_u8(input)?;
        let (input, last_contact_time) = be_u32(input)?;
        let (input, last_attempt_time) = be_u32(input)?;
        let (input, call_attempts_since_reset) = be_u32(input)?;
        let (input, successful_connects_since_reset) = be_u32(input)?;
        let (input, average_connection_duration) = be_u32(input)?;
        let (input, connection_duration_std_dev) = be_u32(input)?;
        Ok((
            input,
            StateOfHealth {
                reset_count,
                current_time,
                current_rssi,
                connection_status,
                globalstar_gateway,
                last_contact_time,
                last_attempt_time,
                call_attempts_since_reset,
                successful_connects_since_reset,
                average_connection_duration,
                connection_duration_std_dev,
            },
        ))
    }
}
