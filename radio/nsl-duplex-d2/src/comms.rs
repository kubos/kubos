pub const GET_UPLOADED_FILE_COUNT: &'static str = "GUGETUFC";
pub const GET_UPLOADED_FILE: &'static str = "GUGET_UF";
pub const RESP_HEADER: &'static str = "GU";

pub struct UploadedFile {
    pub name: String,
    pub payload: Vec<u8>,
}
