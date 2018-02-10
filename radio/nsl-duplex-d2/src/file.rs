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

fn size_from_utf_8(utf8_size: Vec<u8>) -> usize {
    String::from_utf8(utf8_size).unwrap().parse::<usize>().unwrap()
}
