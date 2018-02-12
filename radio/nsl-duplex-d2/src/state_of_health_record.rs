use d2_message::{D2Message};

#[derive(Default)]
pub struct StateOfHealthRecord {
    message: Vec<u8>,
    pub reset_count:                     [u8; 4], // (4 byte integer) Current epoch reset count, starts at 0, incremented for each power system reset, persistent over the life of the mission
    pub current_time:                    [u8; 4], // (4 byte integer) Current time (seconds) from start of most recent reset
    pub current_rssi:                    u8,      // (1 byte integer) Current RSSI (Received Signal Strength Indicator), 0 to 4
    pub connection_status:               u8,      // (1 byte integer) Connection status, 0 (connected) or 1 (disconnected)
    pub globalstar_gateway:              u8,      // (1 byte integer) Globalstar gateway connected to, proprietary ID, 0 to 255
    pub last_contact_time:               [u8; 4], // (4 byte integer) Last contact time, seconds since latest reset
    pub last_attempt_time:               [u8; 4], // (4 byte integer) Last attempt time, seconds since latest reset
    pub call_attempts_since_reset:       [u8; 4], // (4 byte integer) Count of call attempts since latest reset
    pub successful_connects_since_reset: [u8; 4], // (4 byte integer) Count of successful connects since latest reset
    pub average_connection_duration:     [u8; 4], // (4 byte integer) Average connection duration (seconds)
    pub connection_duration_std_dev:     [u8; 4]  // (4 byte integer) Connection duration standard deviation (seconds)
}

impl D2Message for StateOfHealthRecord {
  fn new(message: Vec<u8>) -> StateOfHealthRecord {
        let mut record: StateOfHealthRecord = Default::default();
        record.reset_count.copy_from_slice(&message[2..6]);
        record.current_time.copy_from_slice(&message[6..10]);
        record.current_rssi = message[10];
        record.connection_status = message[11];
        record.globalstar_gateway = message[12];
        record.last_contact_time.copy_from_slice(&message[13..17]);
        record.last_attempt_time.copy_from_slice(&message[17..21]);
        record.call_attempts_since_reset.copy_from_slice(&message[21..25]);
        record.successful_connects_since_reset.copy_from_slice(&message[25..29]);
        record.average_connection_duration.copy_from_slice(&message[29..33]);
        record.connection_duration_std_dev.copy_from_slice(&message[33..37]);
        record.message = message;
        record
  }

    fn message(&self) -> & Vec<u8> {
        & self.message
    }
}

#[cfg(test)]
pub mod tests {
    use ::*;
    use state_of_health_record::*;

    #[test]
    fn test_new_from_message() {
        let test_record = StateOfHealthRecord::new(soh_message());
        assert_eq!(test_record.reset_count, [0,0,1,2]);
        assert_eq!(test_record.current_time, [5,6,7,8]);
        assert_eq!(test_record.current_rssi, 9);
        assert_eq!(test_record.connection_status, 3);
        assert_eq!(test_record.globalstar_gateway, 8);
        assert_eq!(test_record.last_contact_time, [0,7,0,7]);
        assert_eq!(test_record.last_attempt_time, [1,0,1,0]);
        assert_eq!(test_record.call_attempts_since_reset, [3,3,3,3]);
        assert_eq!(test_record.successful_connects_since_reset, [1,1,9,9]);
        assert_eq!(test_record.average_connection_duration, [7,6,5,4]);
        assert_eq!(test_record.connection_duration_std_dev, [2,2,2,2]);
    }

    pub fn soh_message() -> Vec<u8> {
        let mut soh_message = Vec::<u8>::new();
        let reset_count = [0,0,1,2];
        let current_time = [5,6,7,8];
        let current_rssi = 9;
        let connection_status = 3;
        let globalstar_gateway = 8;
        let last_contact_time = [0,7,0,7];
        let last_attempt_time = [1,0,1,0];
        let call_attempts_since_reset = [3,3,3,3];
        let successful_connects_since_reset = [1,1,9,9];
        let average_connection_duration = [7,6,5,4];
        let connection_duration_std_dev = [2,2,2,2];

        soh_message.extend(RESP_HEADER.as_bytes());
        soh_message.extend(reset_count.iter());
        soh_message.extend(current_time.iter());
        soh_message.push(current_rssi);
        soh_message.push(connection_status);
        soh_message.push(globalstar_gateway);
        soh_message.extend(last_contact_time.iter());
        soh_message.extend(last_attempt_time.iter());
        soh_message.extend(call_attempts_since_reset.iter());
        soh_message.extend(successful_connects_since_reset.iter());
        soh_message.extend(average_connection_duration.iter());
        soh_message.extend(connection_duration_std_dev.iter());
        soh_message
    }
}
