use command::Command;

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
    use commands::*;

    #[test]
    fn creates_a_command() {
        let cmd = GetUploadedFileCount::new();
        assert_eq!(0x4755_4745_5455_4643, cmd.command_bytes);
    }
}
