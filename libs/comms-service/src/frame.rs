//! Frame layer definitions used by the communications service

use crate::CommsResult;

/// Generic LinkFrame trait which provides internal framing requirements
/// of the communications service
pub trait LinkFrame {
    /// Construct a LinkFrame (or series of)
    fn build(count: u16, id: u8, max_size: u32, payload: &[u8]) -> CommsResult<Vec<Box<Self>>>;
    /// Create a bytes representation of the frame
    fn to_bytes(&self) -> CommsResult<Vec<u8>>;
    /// Attempt to find and parse frame from bytes. Return frame and/or remaining bytes.
    fn from_bytes(raw: &[u8]) -> CommsResult<(Option<Box<Self>>, Vec<u8>)>;
    /// Chunks up the frame into smaller frames
    fn to_chunks(&self, chunk_size: u32) -> CommsResult<Vec<Box<Self>>>;
    /// Attempts to assemble frame from chunk frames
    fn from_chunks(parts: Vec<Box<Self>>) -> CommsResult<Box<Self>>;
    /// The unique count of the frame.
    fn frame_count(&self) -> u16;
    /// The ID of the frame. ID is duplicated across parts when a frame is chunked.
    fn frame_id(&self) -> u8;
    /// The part of the frame. Used when a frame is chunked.
    fn frame_part(&self) -> u8;
    /// The payload of the frame
    fn payload(&self) -> Vec<u8>;
}
