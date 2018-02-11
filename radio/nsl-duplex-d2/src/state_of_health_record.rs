//• (4 byte integer) Current epoch reset count, starts at 0, incremented for each power system reset, persistent over the life of the mission
//• (4 byte integer) Current time (seconds) from start of most recent reset
//• (1 byte integer) Current RSSI (Received Signal Strength Indicator), 0 to 4
//• (1 byte integer) Connection status, 0 (connected) or 1 (disconnected)
//• (1 byte integer) Globalstar gateway connected to, proprietary ID, 0 to 255
//• (4 byte integer) Last contact time, seconds since latest reset
//• (4 byte integer) Last attempt time, seconds since latest reset
//• (4 byte integer) Count of call attempts since latest reset
//• (4 byte integer) Count of successful connects since latest reset
//• (4 byte integer) Average connection duration (seconds)
//• (4 byte integer) Connection duration standard deviation (seconds)

#[derive(Default)]
pub struct StateOfHealthRecord {
   reset_count: [u8; 4],
}

impl StateOfHealthRecord {
    pub fn from_response(soh_response: Vec<u8>) -> StateOfHealthRecord {
        let mut state_of_health_record: StateOfHealthRecord = Default::default();
        state_of_health_record.reset_count.copy_from_slice(&soh_response[2..6]);
        state_of_health_record
    }
}

#[cfg(test)]
mod tests {
    use ::*;
    use state_of_health_record::*;

    #[test]
    fn test_reset_count() {
        let mut soh_message = Vec::<u8>::new();
        let reset_count = [0,0,1,2];
        soh_message.extend(RESP_HEADER.as_bytes());
        soh_message.extend(reset_count.iter());

        let state_of_health_record = StateOfHealthRecord::from_response(soh_message);
        assert_eq!(state_of_health_record.reset_count, [0,0,1,2]);
    }
}
