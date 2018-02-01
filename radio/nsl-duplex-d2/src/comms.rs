/// Sends command requesting SOH and retrieves response
/// The GUGETSOH command requests a SOH record
/// Each SOH record begins with the sync bytes 0x4755
/// Followed by 35 bytes. All 37 bytes are Big Endian.
pub fn fetch_state_of_health() -> Result<(), String> {
    soh_cmd = "GUGETSOH";
    let mut soh_resp = [u8; 37];

    // send_command(soh_cmd)
    // read_response(&soh_resp);

    Ok(())
}
