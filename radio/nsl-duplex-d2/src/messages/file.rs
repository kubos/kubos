use nom::IResult;
use std::str;

#[derive(Debug, PartialEq)]
pub struct File {
    name: String,
    body: Vec<u8>,
}

impl File {
    pub fn parse(input: &[u8]) -> IResult<&[u8], File> {
        let (input, _) = tag!(input, b"GU")?;
        let (input, name_length) = length_header(input)?;
        let (input, body_length) = length_header(input)?;
        let (input, name) = take_str!(input, name_length)?;
        let name = String::from(name);
        let (input, body) = take!(input, body_length)?;
        let body = Vec::from(body);
        Ok((input, File { name, body }))
    }
}

// Take 4 bytes, convert to string and then parse as usize
named!(
    length_header<usize>,
    map_res!(take_str!(4), str::FromStr::from_str)
);

#[cfg(test)]
mod tests {
    use super::File;
    #[test]
    fn it_parses() {
        assert_eq!(
            Ok((
                b"extra" as &[u8],
                File {
                    name: String::from("test.txt"),
                    body: b"Hello World\n".to_vec(),
                }
            )),
            File::parse(b"GU00080012test.txtHello World\nextra")
        );
    }
}
