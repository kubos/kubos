# Pumpkin MCU API 

API for interaction with Pumpkin MCUs. See the Firmware Reference Manual from Pumpkin for more hardware information. 

## Configuration

To communicate, a module must be contained in the configuration data within the API. This data contains the module's name, telemetry items that are available for the module, and parsing instructions for the module telemetry. 

The "supervisor" portion of the telemetry configuration is common to all MCUs. 

If read_telemetry function is returning items with a timestamp of 0, this means the Data Ready field was 0, and the data for that item is invalid. It must be re-requested with a longer "DELAY" (delay time between requesting the data and reading it). This is set by default to 200 ms, but if it consistently getting data that isn't ready, it is recommended to set it to a full second. Reference the Firmware Reference Manual for more details on this. 

## Usage

Look at the pumpkin-mcu-api-example in the examples folder for information on usage. 

## References

This api is compatible with the Pumpkin Firmware Reference Manual version 3.5 
