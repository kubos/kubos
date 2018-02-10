pub const GET_UPLOADED_FILE_COUNT: &str = "GUGETUFC";
pub const GET_MODEM_HEALTH_STATE: &str = "GUGETHS";
pub const GET_UPLOADED_FILE: &str = "GUGET_UF";
pub const RESP_HEADER: &str = "GU";

pub struct UploadedFile {
    pub name: String,
    pub payload: Vec<u8>,
}

impl UploadedFile {
    pub fn from_response(file_response: Vec<u8>) -> UploadedFile {
        let name_size = UploadedFile::size_from_utf_8(file_response[2..5].to_vec());
        let payload_size = UploadedFile::size_from_utf_8(file_response[5..11].to_vec());

        let name = String::from_utf8(file_response[11..(11 + name_size)].to_vec()).unwrap();
        let payload = file_response[(11 + name_size)..(11 + name_size + payload_size)].to_vec();

        UploadedFile {
            name: name,
            payload: payload,
        }
    }

    fn size_from_utf_8(utf8_size: Vec<u8>) -> usize {
        String::from_utf8(utf8_size).unwrap().parse::<usize>().unwrap()
    }
}
