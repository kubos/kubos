# Pumpkin MCU API 

API for interaction with Pumpkin MCUs. See the Firmware Reference Manual from Pumpkin for more hardware information. 

## Configuration

To communicate, a module must be defined in the configuration data within the mcu_api.py file. This data contains the module's name, telemetry items that are available for the module, and parsing instructions for the module telemetry in the following format:

.. code::

	"modulename":{
	        "fieldname1": {"command":"SCPI Telem Request", "length":int, "parsing":fmtstr},
	        "fieldname2": {"command":"SCPI Telem Request", "length":int, "parsing":fmtstr},
	        "fieldname3": {"command":"SCPI Telem Request", "length":int, "parsing":fmtstr, "names":["subfield1","subfield2"]}
	    } 

Field names, module names, and subfield names must be strings (fieldname cannot be 'all'). The SCPI Telem Request is the string SCPI command recorded in the Firmware Reference Manual. The length field is an integer and matches the telemetry definitions table in the Firmware Reference Manual. The parsing field has 3 options currently:

  - "hex" reformats the data returned to be a hex string, eg: "34f3a30a". 
  - "str" is for ascii return, cutting off the reply at the null terminator per the Firmware Reference Manual
  - The last option is any valid struct.unpack format string. 

If the struct.unpack format string indicates there are multiple values unpacked, the item must have the "names" field with an array of subfield names equal to the number of items unpacked. 

All MCUs have SUP:TEL? ... commands. These commands are encapsulated in the existing "supervisor" entry in the TELEMETRY definition in mcu_api.py and therefore do not need to be re-defined when adding new modules. 

Modules currently configured: 

  - sim
  - gpsrm
  - aim2
  - bim
  - pim
  - rhm
  - bsm
  - bm2

If read_telemetry function is returning items with a timestamp of 0, this means the Data Ready field was 0, and the data for that item is invalid. It must be re-requested with a longer "DELAY" (delay time between requesting the data and reading it). This is set by default to 200 ms, but if it consistently getting data that isn't ready, it is recommended to set it to a full second. Reference the Firmware Reference Manual for more details on this. 

## Usage

Look at the pumpkin-mcu-api-example in the examples folder for information on usage. 

## References

This api is compatible with the Pumpkin Firmware Reference Manual version 3.5 
