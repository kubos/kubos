General Hardware
================

This is what is generally expected for any piece of hardware integrated, but doesn't fit into a predefined category. This should be augmented with commands/telemetry queries for the unit that are frequently used. 

Commands:
- No-op
- Power
  - On, Off, Reset
  - Reset is generally considered to be a soft reset for this implementation
- Configuration
  - Anything required to set up the until and have it perform nominally
- HW Test
  - Returns Pass/Fail and all telemetry gathered during test
  - Tests that HW is on and working 
- Passthrough
  - This is used to access any commands not implemented in the API

Telemetry:
- Power
  - on/off
  - Uptime
- Status
- Errors
- Nominal Telemetry
  - Telemetry that would normally be monitored to ascertain the health of the unit
- Debug Telemetry
  - Telemetry items that are only used when debugging issues encountered with the unit, and are normally uninteresting as long as the unit is operating as designed

Anything not implemented returns a :doc:`standard error code`. 
