use nom::{IResult, be_u32};

#[derive(Debug, PartialEq)]
pub struct FileCount(u32);

impl FileCount {
    pub fn parse(input: &[u8]) -> IResult<&[u8], FileCount> {
        let (input, _) = tag!(input, b"GU")?;
        let (input, file_count) = be_u32(input)?;
        Ok((input, FileCount(file_count)))
    }
}

#[cfg(test)]
mod tests {
    use super::FileCount;
    #[test]
    fn it_parses() {
        assert_eq!(
            Ok((&b"\x9a\xbc"[..], FileCount(0x12345678))),
            FileCount::parse(b"GU\x12\x34\x56\x78\x9a\xbc")
        );
    }
}
