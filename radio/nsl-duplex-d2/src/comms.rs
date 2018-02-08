use radio_stub;

pub const GET_UPLOADED_FILE_COUNT: &'static str = "GUGETUFC";
pub const GET_UPLOADED_FILE: &'static str = "GUGET_UF";

/// Sends command requesting SOH and retrieves response
/// The GUGETSOH command requests a SOH record
/// Each SOH record begins with the sync bytes 0x4755
/// Followed by 35 bytes. All 37 bytes are Big Endian.
pub fn fetch_state_of_health() -> Result<(), String> {
    let soh_cmd = "GUGETSOH";
    let mut soh_resp: [u8; 37] = [0; 37];

    radio_stub::send_command(soh_cmd);

    Ok(())
}

pub fn get_uploaded_file_count() -> Result<u32, String> {
    let resp = radio_stub::send_command(GET_UPLOADED_FILE_COUNT).unwrap();

    if resp.len() != 6 {
        return Err(String::from("Wrong response length"));
    }

    // Check if resp header exists
    if (resp[0] == ('G' as u8)) && (resp[1] == ('U' as u8)) {
        let count = (resp[2] as u32) | (resp[3] as u32) << 8 | (resp[4] as u32) << 16
            | (resp[5] as u32) << 24;
        Ok(count)
    } else {
        Err(String::from("Invalid resp header"))
    }
}

pub struct UploadedFile {
    name: Vec<u8>,
    payload: Vec<u8>,
}

pub fn get_uploaded_file() -> Result<UploadedFile, String> {
    let resp = radio_stub::send_command(GET_UPLOADED_FILE).unwrap();

    if ((resp[0] != ('G' as u8)) && (resp[1] != ('U' as u8))) {
        return Err(String::from("Invalid resp header"));
    }

    println!("{:?}", resp);

    //let name_size =
    //     ((i32::from(resp[2])) * 100) + ((i32::from(resp[3])) * 10) + (i32::from(resp[4]));
    let name_size = String::from_utf8(resp[2..5].to_vec())
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let payload_size = String::from_utf8(resp[5..11].to_vec())
        .unwrap()
        .parse::<usize>()
        .unwrap();

    //let payload_size = ((resp[5] as usize) * 100000) + ((resp[6] as usize) * 10000)
    //  + ((resp[7] as usize) * 1000) + ((resp[8] as usize) * 100)
    //+ ((resp[9] as usize) * 10) + (resp[10] as usize);

    println!("name size {} payload size {}", name_size, payload_size);

    let name: Vec<u8> = resp[11..(11 + name_size)].to_vec();
    let payload: Vec<u8> = resp[(11 + name_size)..(11 + name_size + payload_size)].to_vec();

    Ok(UploadedFile {
        name: name,
        payload: payload,
    })
}

#[cfg(test)]
mod tests {
    use ::*;

    #[test]
    fn test_uploaded_file_count() {
        let count = comms::get_uploaded_file_count().unwrap();
        assert_eq!(count, 1, "File count should be one")
    }

    #[test]
    fn test_uploaded_file() {
        let file = comms::get_uploaded_file().unwrap();
        // check file name
        assert_eq!(file.name[0], 't' as u8);
        assert_eq!(file.name[1], 'e' as u8);
        assert_eq!(file.name[2], 's' as u8);
        assert_eq!(file.name[3], 't' as u8);
        // check payload
        assert_eq!(file.payload[0], 't' as u8);
        assert_eq!(file.payload[1], 'e' as u8);
        assert_eq!(file.payload[2], 's' as u8);
        assert_eq!(file.payload[3], 't' as u8);
    }
}
