pub struct File {
    pub name: String,
    pub data: Vec<u8>,
}

impl File {
    fn name_from_response(file_response: &[u8], name_size: usize) -> Result<String, String> {
        let name = file_response
            .get(11..(11 + name_size))
            .ok_or("Invalid buffer length")?;

        String::from_utf8(name.to_vec()).or(Err(String::from("Invalid buffer size")))
    }

    fn data_from_response(
        file_response: &[u8],
        name_size: usize,
        data_size: usize,
    ) -> Result<Vec<u8>, String> {
        let data = file_response
            .get((11 + name_size)..(11 + name_size + data_size))
            .ok_or("Invalid buffer length")?;

        Ok(data.to_vec())
    }

    pub fn from_response(file_response: &[u8]) -> Result<File, String> {
        let name_size = size_from_utf_8(file_response[2..5].to_vec());
        let data_size = size_from_utf_8(file_response[5..11].to_vec());

        Ok(File {
            name: File::name_from_response(file_response, name_size)?,
            data: File::data_from_response(file_response, name_size, data_size)?,
        })
    }
}

pub fn process_file_count(file_count: &[u8]) -> u32 {
    u32::from(file_count[2]) | u32::from(file_count[3]) << 8 | u32::from(file_count[4]) << 16
        | u32::from(file_count[5]) << 24
}

fn size_from_utf_8(utf8_size: Vec<u8>) -> usize {
    String::from_utf8(utf8_size)
        .unwrap()
        .parse::<usize>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use file::*;
    use comms::*;
    use nums_as_bytes::AsBytes;

    #[test]
    fn test_file_from_response() {
        let mut ret_msg = Vec::<u8>::new();
        ret_msg.extend(RESP_HEADER.as_bytes());
        let name_size = String::from("008");
        let size = String::from("000004");
        let name = String::from("test.txt");
        let data = String::from("test");
        let crc = String::from("44");

        ret_msg.extend(name_size.as_bytes().iter().cloned());
        ret_msg.extend(size.as_bytes().iter().cloned());
        ret_msg.extend(name.as_bytes().iter().cloned());
        ret_msg.extend(data.as_bytes().iter().cloned());
        ret_msg.extend(crc.as_bytes().iter().cloned());

        let uploaded_file = File::from_response(&ret_msg);
        assert!(uploaded_file.is_ok(), "Should get valid response");
        let uploaded_file = uploaded_file.unwrap();
        // check file name
        assert_eq!(uploaded_file.name, String::from("test.txt"));
        // check data
        assert_eq!(
            String::from_utf8(uploaded_file.data).unwrap(),
            String::from("test")
        );
    }

    #[test]
    fn test_file_from_response_size_too_big() {
        let mut ret_msg = Vec::<u8>::new();
        ret_msg.extend(RESP_HEADER.as_bytes());
        let name_size = String::from("111");
        let size = String::from("000111");
        let name = String::from("test.txt");
        let data = String::from("test");
        let crc = String::from("44");

        ret_msg.extend(name_size.as_bytes().iter().cloned());
        ret_msg.extend(size.as_bytes().iter().cloned());
        ret_msg.extend(name.as_bytes().iter().cloned());
        ret_msg.extend(data.as_bytes().iter().cloned());
        ret_msg.extend(crc.as_bytes().iter().cloned());

        let uploaded_file = File::from_response(&ret_msg);
        assert!(uploaded_file.is_err(), "Should get an error result");
    }

    #[test]
    fn test_uploaded_file_count_one() {
        let mut ret_msg = Vec::<u8>::new();
        ret_msg.extend(RESP_HEADER.as_bytes());
        ret_msg.push(1 as u8);
        ret_msg.push(0 as u8);
        ret_msg.push(0 as u8);
        ret_msg.push(0 as u8);
        let count = process_file_count(&ret_msg);
        assert_eq!(count, 1, "File count should be one")
    }

    #[test]
    fn test_uploaded_file_count_zero() {
        let mut ret_msg = Vec::<u8>::new();
        ret_msg.extend(RESP_HEADER.as_bytes());
        ret_msg.push(0 as u8);
        ret_msg.push(0 as u8);
        ret_msg.push(0 as u8);
        ret_msg.push(0 as u8);
        let count = process_file_count(&ret_msg);
        assert_eq!(count, 0, "File count should be zero")
    }

    #[test]
    fn test_uploaded_file_count_many() {
        let mut ret_msg = Vec::<u8>::new();
        ret_msg.extend(RESP_HEADER.as_bytes());
        ret_msg.push(0 as u8);
        ret_msg.push(0 as u8);
        ret_msg.push(0 as u8);
        ret_msg.push(1 as u8);
        let count = process_file_count(&ret_msg);
        assert_eq!(count, 16777216, "File count should be 16777216")
    }
}
