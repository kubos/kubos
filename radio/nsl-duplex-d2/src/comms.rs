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
    let resp = match radio_stub::send_command(GET_UPLOADED_FILE_COUNT) {
        Ok(r) => r,
        Err(e) => return Err(String::from("Failed to send command")),
    };

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
    name: String,
    payload: Vec<u8>,
}

pub fn get_uploaded_file() -> Result<UploadedFile, String> {
    let resp = radio_stub::send_command(GET_UPLOADED_FILE).unwrap();

    if ((resp[0] != ('G' as u8)) && (resp[1] != ('U' as u8))) {
        return Err(String::from("Invalid resp header"));
    }

    let name_size = String::from_utf8(resp[2..5].to_vec())
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let payload_size = String::from_utf8(resp[5..11].to_vec())
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let name = String::from_utf8(resp[11..(11 + name_size)].to_vec()).unwrap();
    let payload = resp[(11 + name_size)..(11 + name_size + payload_size)].to_vec();

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
        assert_eq!(file.name, String::from("test.txt"));
        // check payload
        assert_eq!(
            String::from_utf8(file.payload).unwrap(),
            String::from("test")
        );
    }
}
