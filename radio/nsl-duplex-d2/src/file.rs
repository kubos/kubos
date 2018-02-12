pub struct File {
    pub name: String,
    pub payload: Vec<u8>,
}

impl File {
    pub fn from_response(file_response: Vec<u8>) -> File {
        let name_size = size_from_utf_8(file_response[2..5].to_vec());
        let payload_size = size_from_utf_8(file_response[5..11].to_vec());

        let name = String::from_utf8(file_response[11..(11 + name_size)].to_vec()).unwrap();
        let payload = file_response[(11 + name_size)..(11 + name_size + payload_size)].to_vec();

        File {
            name: name,
            payload: payload,
        }
    }
}

pub fn process_file_count(file_count: Vec<u8>) -> u32 {
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

        let uploaded_file = File::from_response(ret_msg);
        // check file name
        assert_eq!(uploaded_file.name, String::from("test.txt"));
        // check payload
        assert_eq!(
            String::from_utf8(uploaded_file.payload).unwrap(),
            String::from("test")
        );
    }

    #[test]
    fn test_uploaded_file_count_one() {
        let mut ret_msg = Vec::<u8>::new();
        ret_msg.extend(RESP_HEADER.as_bytes());
        ret_msg.push(1 as u8);
        ret_msg.push(0 as u8);
        ret_msg.push(0 as u8);
        ret_msg.push(0 as u8);
        let count = process_file_count(ret_msg);
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
        let count = process_file_count(ret_msg);
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
        let count = process_file_count(ret_msg);
        assert_eq!(count, 16777216, "File count should be 16777216")
    }
}
