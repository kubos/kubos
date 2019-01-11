fn escape(buf: &[u8]) -> Vec<u8> {
    let mut new_buf = vec![];
    for (_, e) in buf.iter().enumerate() {
        match e {
            0xC0 => {
                new_buf.push(0xDB);
                new_buf.push(0xDC);
            }
            0xDB => {
                new_buf.push(0xDB);
                new_buf.push(0xDD);
            }
            _ => new_buf.push(*e),
        };
    }
    new_buf
}

fn unescape(buf: &[u8]) -> (Vec<u8>, bool) {
    let mut new_buf = vec![];

    let mut i = 0;
    while i < buf.len() {
        let e = buf[i];
        match e {
            0xDB => {
                new_buf.push(match buf.get(i + 1) {
                    Some(0xDC) => 0xC0,
                    Some(0xDD) => 0xDB,
                    _ => {
                        return (vec![], false);
                    }
                });
                i += 1;
            }
            _ => new_buf.push(e),
        }
        i += 1;
    }

    (new_buf, true)
}

pub fn encode(frame: &[u8]) -> Result<Vec<u8>, String> {
    let mut buff = vec![0xC0, 0x00];

    buff.extend(escape(frame).iter().clone());
    buff.push(0xC0);

    Ok(buff)
}

pub fn decode(chunk: &[u8]) -> Result<Vec<u8>, String> {
    let mut frame = vec![];
    let mut index_l = 0;
    let mut valid = false;

    if chunk.len() < 2 {
        return Err(String::from("Kiss frame start not found"));
    }

    while !valid {
        frame.clear();
        let mut index_a = 0;
        let mut index_b;

        // Search for first full kiss frame
        for (i, e) in chunk.iter().skip(index_l).enumerate() {
            if *e == 0xC0 {
                if let Some(piece) = chunk.get(i + 1) {
                    if *piece == 0x00 {
                        index_a = i + index_l + 1;
                        break;
                    }
                }
            }
        }
        if index_a == 0 {
            return Err(String::from("Kiss frame start not found"));
        }

        index_b = 0;
        // Search for end sequence?
        for (i, e) in chunk.iter().skip(index_a).enumerate() {
            if *e == 0xC0 {
                index_b = i + index_a + 1;
                break;
            }
        }
        if index_b == 0 {
            return Err(String::from("Kiss frame end not found"));
        }

        // Extract the frame payload
        frame.extend(chunk[index_a + 1..index_b - 1].iter().clone());
        index_l = index_b;

        // Unescape KISS control characters
        let (un_frame, check) = unescape(&frame);
        valid = check;
        frame = un_frame;
    }

    Ok(frame)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unescapes() {
        assert_eq!(unescape(&vec![0xDB, 0xDC]), (vec![0xC0], true));

        assert_eq!(unescape(&vec![0xDB, 0xDD]), (vec![0xDB], true));

        assert_eq!(
            unescape(&vec![0x1, 0xDB, 0xDC, 0x1]),
            (vec![0x1, 0xC0, 0x1], true)
        );

        assert_eq!(
            unescape(&vec![0x1, 0xDB, 0xDC, 0x2, 0xDB, 0xDD, 0x3]),
            (vec![0x1, 0xC0, 0x2, 0xDB, 0x3], true)
        );

        assert_eq!(unescape(&vec![0xDB, 0x11]), (vec![], false));
    }

    #[test]
    fn test_encode() {
        let encoded = encode(&vec![0x00, 0x01, 0x02, 0x03]).unwrap();

        assert_eq!(encoded, vec![0xC0, 0x00, 0x00, 0x01, 0x02, 0x03, 0xC0]);
    }

    #[test]
    fn test_encode_with_escape() {
        let encoded = encode(&vec![0x01, 0x02, 0xC0, 0x04]).unwrap();

        assert_eq!(
            encoded,
            vec![0xC0, 0x00, 0x01, 0x02, 0xDB, 0xDC, 0x04, 0xC0]
        );
    }

    #[test]
    fn test_encode_with_n_escapes() {
        let encoded = encode(&vec![0x01, 0xDB, 0x02, 0xC0, 0x04, 0xDB, 0xC0]).unwrap();

        assert_eq!(
            encoded,
            vec![
                0xC0, 0x00, 0x01, 0xDB, 0xDD, 0x02, 0xDB, 0xDC, 0x04, 0xDB, 0xDD, 0xDB, 0xDC, 0xC0
            ]
        );
    }

    #[test]
    fn test_decode_frame() {
        let decoded = decode(&vec![0xC0, 0x00, 0x01, 0x01, 0x01, 0xC0]).unwrap();
        assert_eq!(decoded, vec![0x01, 0x1, 0x1]);
    }

    #[test]
    fn test_decode_frame_pre_junk() {
        let decoded = decode(&vec![0xFF, 0xBB, 0xCC, 0xC0, 0x00, 0x01, 0x02, 0x03, 0xC0]).unwrap();
        assert_eq!(decoded, vec![0x01, 0x2, 0x3]);
    }

    #[test]
    fn test_decode_frame_post_junk() {
        let decoded = decode(&vec![0xC0, 0x00, 0x03, 0x02, 0x01, 0xC0, 0xFF, 0xBB, 0xCC]).unwrap();
        assert_eq!(decoded, vec![0x03, 0x2, 0x1]);
    }

    #[test]
    fn test_decode_frame_junk_surround() {
        let decoded = decode(&vec![
            0xFF, 0xBB, 0xCC, 0xC0, 0x00, 0x03, 0x04, 0x05, 0xC0, 0xFF, 0xBB, 0xCC,
        ])
        .unwrap();
        assert_eq!(decoded, vec![0x03, 0x4, 0x5]);
    }

    #[test]
    fn test_decode_frame_escapes() {
        let decoded = decode(&vec![
            0xC0, 0x00, 0x03, 0xDB, 0xDC, 0x04, 0xDB, 0xDD, 0x05, 0xC0,
        ])
        .unwrap();
        assert_eq!(decoded, vec![0x03, 0xC0, 0x4, 0xDB, 0x5]);
    }

    #[test]
    fn test_decode_frame_escapes_junks() {
        let decoded = decode(&vec![
            0x1, 0xF, 0xC0, 0x00, 0x03, 0xDB, 0xDC, 0x04, 0xDB, 0xDD, 0x05, 0xC0, 0x1, 0xF, 0x2,
        ])
        .unwrap();
        assert_eq!(decoded, vec![0x03, 0xC0, 0x4, 0xDB, 0x5]);
    }

    #[test]
    fn test_decode_frame_no_start() {
        assert_eq!(
            decode(&vec![
                0x1, 0xF, 0xC1, 0x00, 0x03, 0xDB, 0xDC, 0x04, 0xDB, 0xDD, 0x05, 0xC0, 0x1, 0xF, 0x2
            ],),
            Err(String::from("Kiss frame start not found"))
        );
    }

    #[test]
    fn test_decode_frame_no_end() {
        assert_eq!(
            decode(&vec![
                0x1, 0xF, 0xC0, 0x00, 0x03, 0xDB, 0xDC, 0x04, 0xDB, 0xDD, 0x05, 0x10, 0x1, 0xF, 0x2
            ],),
            Err(String::from("Kiss frame end not found"))
        );
    }

    #[test]
    fn test_decode_test_data() {
        assert_eq!(
            decode(&vec![
                192, 000, 027, 88, 143, 61, 000, 98, 85, 98, 000, 130, 026, 000, 004, 124, 82, 245,
                192,
            ],),
            Ok(vec![
                27, 88, 143, 61, 0, 98, 85, 98, 0, 130, 26, 0, 4, 124, 82, 245
            ])
        );
    }

    #[test]
    fn test_encode_decode() {
        let orig = vec![0, 130, 26, 0, 1, 218, 134, 245];

        let encoded = encode(&orig).unwrap();
        let decoded_dat = decode(&encoded).unwrap();

        assert_eq!(decoded_dat, orig);
    }
}
