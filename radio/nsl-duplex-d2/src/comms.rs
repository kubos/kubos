pub const GET_UPLOADED_FILE_COUNT: u64 = 0x4755474554554643;
pub const GET_UPLOADED_FILE: u64 = 0x47554745545F5546;
pub const RESP_HEADER: u32 = 0x4755;
pub const ACK: u32 = 0x475506;
pub const NACK: u32 = 0x47550F;

pub struct UploadedFile {
    pub name: String,
    pub payload: Vec<u8>,
}
