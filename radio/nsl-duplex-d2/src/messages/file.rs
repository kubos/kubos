use nom::IResult;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct File {
    name: String,
    body: Vec<u8>,
}

impl File {
    pub fn parse(input: &[u8]) -> IResult<&[u8], File> {
        let (input, _) = tag!(input, b"GU")?;
        let (input, name_length) = map_res!(input, take_str!(3), usize::from_str)?;
        let (input, body_length) = map_res!(input, take_str!(6), usize::from_str)?;
        let (input, name) = take_str!(input, name_length)?;
        let name = String::from(name);
        let (input, body) = take!(input, body_length)?;
        let body = Vec::from(body);
        Ok((input, File { name, body }))
    }
}

#[cfg(test)]
mod tests {
    use super::File;
    #[test]
    fn it_parses() {
        assert_eq!(
            Ok((
                &b"extra"[..],
                File {
                    name: String::from("test.txt"),
                    body: b"Hello World\n".to_vec(),
                }
            )),
            File::parse(b"GU008000012test.txtHello World\nextra")
        );
    }
}
