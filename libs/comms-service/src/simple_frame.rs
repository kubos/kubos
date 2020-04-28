//! A simple framing definition.
//! Inspired by the PPP framing (http://www.tcpipguide.com/free/t_PPPGeneralFrameFormat.htm)
//!
//! Layout will be as follows:
//! Start Flag - 1 byte - Fixed at 0x7E
//! Frame Count - 2 bytes
//! Frame ID - 1 Byte
//! Frame Part - 1 byte
//! Payload - n bytes
//! End Flag - 1 byte - Fixed at 0x7E

use crate::frame::LinkFrame;
use crate::CommsResult;

const START_END_FLAG: u8 = 0x7E;

struct Header {
    /// Frame Count
    count: u16,
    /// Frame ID
    id: u8,
    /// Frame Part
    part: u8,
}

pub struct SimpleFrame {
    header: Header,
    payload: Vec<u8>,
}

impl LinkFrame for SimpleFrame {
    /// Construct a LinkFrame (or series) from info + payload
    /// Chunking up will be handled here
    fn build(count: u16, id: u8, max_size: u32, payload: &[u8]) -> CommsResult<Vec<Box<Self>>> {
        let part = 0;
        let base_frame = SimpleFrame {
            header: Header { count, id, part },
            payload: payload.to_vec(),
        };
        // if frame size <= max_size
        let frames = vec![Box::new(base_frame)];
        // else
        // do some base_frame.to_chunks() and store in frames vec

        Ok(frames)
    }

    /// Create a bytes representation of the frame
    fn to_bytes(&self) -> CommsResult<Vec<u8>> {
        Ok(vec![])
    }

    /// Attempt to find and parse frame from bytes. Return frame and/or remaining bytes.
    /// Should this handle de-chunking??
    fn from_bytes(raw: &[u8]) -> CommsResult<(Option<Box<Self>>, Vec<u8>)> {
        let frame = Some(Box::new(SimpleFrame {
            header: Header {
                count: 0,
                id: 0,
                part: 0,
            },
            payload: vec![],
        }));
        let remainder = vec![];
        Ok((frame, remainder))
    }

    /// Chunks up the frame into smaller frames
    fn to_chunks(&self, chunk_size: u32) -> CommsResult<Vec<Box<Self>>> {
        Ok(vec![Box::new(SimpleFrame {
            header: Header {
                count: 0,
                id: 0,
                part: 0,
            },
            payload: vec![],
        })])
    }

    /// Attempts to assemble frame from chunk frames
    fn from_chunks(parts: Vec<Box<Self>>) -> CommsResult<Box<Self>> {
        Ok(Box::new(SimpleFrame {
            header: Header {
                count: 0,
                id: 0,
                part: 0,
            },
            payload: vec![],
        }))
    }

    /// The unique count of the frame.
    fn frame_count(&self) -> u16 {
        self.header.count
    }

    /// The ID of the frame. ID is duplicated across parts when a frame is chunked.
    fn frame_id(&self) -> u8 {
        self.header.id
    }

    /// The part of the frame. Used when a frame is chunked.
    fn frame_part(&self) -> u8 {
        self.header.part
    }

    /// The payload of the frame
    fn payload(&self) -> Vec<u8> {
        self.payload
    }
}
