use command::Command;
use file::File;

pub const RESP_HEADER: u32 = 0x4755;

struct GetUploadedFileCount {}

impl Command<u32> for GetUploadedFileCount {
    fn command_bytes(&self) -> Vec<u8> {
       vec![0x47, 0x55, 0x47, 0x45, 0x54, 0x55, 0x46, 0x43]
    }

    fn process_response(&self, response: &[u8]) -> Result<u32, String> {
        File::process_file_count(response)
    }
}
    //pub fn get_uploaded_file(&self) -> Result<File, String> {
    //    File::from_response(&self.send_command(GET_UPLOADED_FILE)?)
    //}

    //pub fn get_uploaded_file_count(&self) -> Result<u32, String> {
    //    File::process_file_count(&self.send_command(GET_UPLOADED_FILE_COUNT)?)
    //}

    //pub fn get_state_of_health_record(&self) -> Result<StateOfHealthRecord, String> {
    //    Ok(StateOfHealthRecord::new(
    //        self.send_command(GET_MODEM_STATE_OF_HEALTH)?,
    //    ))
    //}

#[cfg(test)]
mod tests {
    mod get_uploaded_file_count {
        use commands::*;
        use nums_as_bytes::AsBytes;

        #[test]
        fn creates_a_command() {
            let cmd = GetUploadedFileCount{};
           assert_eq!(vec![0x47, 0x55, 0x47, 0x45, 0x54, 0x55, 0x46, 0x43], cmd.command_bytes());
        }

        #[test]
        fn processes_result() {
            let cmd = GetUploadedFileCount{};
            assert_eq!(1, cmd.process_response(&test_result()).unwrap());
        }

        fn test_result() -> Vec<u8> {
            let mut ret_msg = Vec::<u8>::new();
            ret_msg.extend(RESP_HEADER.as_bytes());
            ret_msg.push(1 as u8);
            ret_msg.push(0 as u8);
            ret_msg.push(0 as u8);
            ret_msg.push(0 as u8);
            ret_msg
        }
    }
}
