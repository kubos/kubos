use nom::{IResult, be_u32};

pub struct FileCount(u32);

impl FileCount {
    pub fn parse(input: &[u8]) -> IResult<&[u8], FileCount> {
        let (input, file_count) = be_u32(input)?;
        Ok((input, FileCount(file_count)))
    }
}
