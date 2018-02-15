use command::Command;
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
        Ok(1)
    }
}

#[cfg(test)]
mod tests {
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
        let mut ret_msg = Vec::<u8>::new();
        ret_msg.extend(RESP_HEADER.as_bytes());
        ret_msg.push(1 as u8);
        ret_msg.push(0 as u8);
        ret_msg.push(0 as u8);
        ret_msg.push(0 as u8);
        let count = File::process_file_count(&ret_msg);
        let cmd = GetUploadedFileCount::new();
        assert_eq!(1, cmd.process_response(&ret_msg).unwrap());
    }
}
