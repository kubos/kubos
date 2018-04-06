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
