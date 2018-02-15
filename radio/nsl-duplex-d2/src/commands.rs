use command::Command;
use file::File;

pub const RESP_HEADER: u32 = 0x4755;

struct GetUploadedFileCount {
    command_bytes: u64,
}

impl Command<u32> for GetUploadedFileCount {
    fn new() -> GetUploadedFileCount {
        GetUploadedFileCount {
            command_bytes: 0x4755_4745_5455_4643,
        }
    }

    fn command_bytes(&self) -> u64 {
        self.command_bytes
    }

    fn process_response(&self, response: &[u8]) -> Result<u32, String> {
        File::process_file_count(response)
    }
}

#[cfg(test)]
mod tests {
    mod get_uploaded_file_count {
        use ::*;
        use commands::*;
        use file::*;
        use nums_as_bytes::AsBytes;

        #[test]
        fn creates_a_command() {
            let cmd = GetUploadedFileCount::new();
            assert_eq!(0x4755_4745_5455_4643, cmd.command_bytes);
        }

        #[test]
        fn processes_result() {
            let cmd = GetUploadedFileCount::new();
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
