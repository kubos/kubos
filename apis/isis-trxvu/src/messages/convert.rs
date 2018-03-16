/**
 *  @name Telemetry Conversion Functions
 *  Convert raw ADC values into human-readable units
 */
/**@{*/
/**
 * @param [in] raw Raw ADC value
 * @return Voltage in volts
 */
pub fn get_voltage(raw: u16) -> f32 {
    (raw as f32) * 0.00488
}
/**
 * @param [in] raw Raw ADC value
 * @return Current in milliamps
 */
pub fn get_current(raw: u16) -> f32 {
    (raw as f32) * 0.16643964
}
/**
 * @param [in] raw Raw ADC value
 * @return Temperature in degrees Celsius
 */
pub fn get_temperature(raw: u16) -> f32 {
    (raw as f32) * -0.07669 + 195.6037
}
/**
 * @param [in] raw Raw ADC value
 * @return Doppler shift in hertz
 */
pub fn get_doppler_offset(raw: u16) -> f32 {
    (raw as f32) * 13.352 - 22300.0
}
/**
 * @param [in] raw Raw ADC value
 * @return Received signal strength power in decibel-milliwatts
 */
pub fn get_signal_strength(raw: u16) -> f32 {
    (raw as f32) * 0.03 - 152.0
}
/**
 * @param [in] raw Raw ADC value
 * @return RF reflected power in decibel-milliwatts
 */
pub fn get_rf_power_dbm(raw: u16) -> f32 {
    20.0 * ((raw as f32) * 0.00767).log10()
}
/**
 * @param [in] raw Raw ADC value
 * @return RF reflected power in milliwatts
 */
pub fn get_rf_power_mw(raw: u16) -> f32 {
    let ten: f32 = 10.0;
    (raw as f32) * (raw as f32) * ten.powf(-2.0) * 0.00005887
}
