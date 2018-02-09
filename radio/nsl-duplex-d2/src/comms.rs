pub const GET_UPLOADED_FILE_COUNT: &str = "GUGETUFC";
pub const GET_UPLOADED_FILE: &str = "GUGET_UF";
pub const RESP_HEADER: &str = "GU";

pub struct UploadedFile {
    pub name: String,
    pub payload: Vec<u8>,
}
