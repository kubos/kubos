use mai400::*;
use super::*;

#[test]
fn get_info_good() {
    let mock = mock_new!();

    mock.write.return_value_for(
        (vec![0x90, 0xEB, 0, 0, 0x1D, 0, 0x12, 0x53]),
        Ok(()),
    );

    let mai = MAI400 { conn: Connection { stream: Box::new(mock) } };

    assert_eq!(mai.get_info().unwrap(), ());
}

#[test]
fn get_info_bad() {
    let mock = mock_new!();

    let mai = MAI400 { conn: Connection { stream: Box::new(mock) } };

    assert_eq!(mai.get_info().unwrap_err(), MAIError::GenericError);
}

#[test]
fn set_mode_good() {
    let mock = mock_new!();

    mock.write.return_value_for(
        (vec![
            0x90,
            0xEB,
            0x11,
            0x00,
            0x00,
            0x00,
            0x01,
            0x02,
            0x00,
            0x00,
            0x00,
            0x03,
            0x00,
            0x00,
            0x00,
            0x04,
            0x00,
            0x00,
            0x00,
            0x05,
            0x00,
            0x00,
            0x00,
            0x24,
            0xBF,
        ]),
        Ok(()),
    );

    let mai = MAI400 { conn: Connection { stream: Box::new(mock) } };

    assert_eq!(mai.set_mode(0x01, 0x02, 0x03, 0x04, 0x05).unwrap(), ());
}

#[test]
fn set_mode_bad() {
    let mock = mock_new!();

    let mai = MAI400 { conn: Connection { stream: Box::new(mock) } };

    assert_eq!(
        mai.set_mode(0x01, 0x02, 0x03, 0x04, 0x05).unwrap_err(),
        MAIError::GenericError
    );
}
