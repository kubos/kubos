#!/usr/bin/env python

# Copyright 2018 Kubos Corporation
# Licensed under the Apache License, Version 2.0
# See LICENSE file for details.

"""
API for interacting with all Pumpkin SupMCUs. 

Unit test module for the pumpkin mcu api
"""

import unittest
import mcu_api
import mock

############################
# Testing configuration data

mcu_api.DELAY = 0
mcu_api.HEADER_SIZE = 5
mcu_api.TELEMETRY = {
    "supervisor": {},
    "module_1": {
        "field_1": {"command": "TESTCOMMAND", "length": 2, "parsing": "hex"},
        "field_2": {"command": "TESTCOMMAND", "length": 2, "parsing": "str"},
        "field_3": {"command": "TESTCOMMAND", "length": 2, "parsing": "<H"},
        "field_4": {"command": "TESTCOMMAND", "length": 4, "parsing": "<HH",
                    "names": ["subfield_1", "subfield_2"]}
    }
}


class TestMCUAPI(unittest.TestCase):

    def setUp(self):
        self.mcu = mcu_api.MCU(address=0x20)

    def test_command_type(self):
        with self.assertRaises(TypeError):
            bad_command = 23  # Not a string
            self.mcu.write(command=bad_command)

    def test_stopbyte_appending(self):
        fake_command = "SUP:LED ON"
        with mock.patch('i2c.I2C.write') as mock_i2cwrite:
            self.mcu.write(command=fake_command)
            mock_i2cwrite.assert_called_with(
                data=fake_command + '\x0a',
                device=self.mcu.address)

    def test_read(self):
        read_count = 20
        with mock.patch('i2c.I2C.read') as mock_i2cread:
            self.mcu.read(count=read_count)
            mock_i2cread.assert_called_with(
                device=self.mcu.address,
                count=read_count)

    def test_build_telemetry_dict_modulechecking(self):
        bad_module = "notamodule"
        good_fields = ["field_1"]
        with self.assertRaises(KeyError):
            self.mcu._build_telemetry_dict(
                module=bad_module,
                fields=good_fields)

    def test_build_telemetry_dict_fieldchecking(self):
        bad_fields = ["notafieldname"]
        with self.assertRaises(KeyError):
            self.mcu._build_telemetry_dict(
                module="module_1",
                fields=bad_fields)

    def test_build_telemetry_dict_all(self):
        requests_assert = mcu_api.TELEMETRY['module_1']
        self.assertEqual(self.mcu._build_telemetry_dict(
            module="module_1"),
            requests_assert)

    def test_build_telemetry_dict_field(self):
        requests_assert = {}
        requests_assert['field_1'] = \
            mcu_api.TELEMETRY['module_1']['field_1']
        self.assertEqual(
            self.mcu._build_telemetry_dict(
                module="module_1",
                fields=["field_1"]),
            requests_assert)

    def test_header_parse_datareadyflag(self):
        notready_data = '\x00\x00\x00\x00\x00\x00'
        self.assertEqual(
            self.mcu._header_parse(
                data=notready_data)['timestamp'],
            0)

    def test_header_parse(self):
        data_ready = '\x01'
        timestamp = '\x02\x03\x04\x05'
        data = '\x06'
        inputdata = data_ready+timestamp+data
        output_assert = {
            'timestamp': 841489.94,
            'data': '\x06'
        }
        self.assertEqual(
            self.mcu._header_parse(
                data=inputdata),
            output_assert)

    def test_unpack_str(self):
        result_data = 'this should be included'
        input_data = result_data + '\0this part \0should be \0cut off'
        output_assert = (result_data,)
        self.assertEqual(
            self.mcu._unpack(
                parsing='str',
                data=input_data),
            output_assert)

    def test_unpack_hex(self):
        result_data = b'00010203040506'
        input_data = '\x00\x01\x02\x03\x04\x05\x06'
        output_assert = (result_data,)
        self.assertEqual(
            self.mcu._unpack(
                parsing='hex',
                data=input_data),
            output_assert)

    def test_format_data_oneitem(self):
        fake_telem_field = 'field_1'  # Single item
        fake_input_dict = mcu_api.TELEMETRY['module_1'][fake_telem_field]
        fake_timestamp = 100.00
        fake_read_data = {'timestamp': fake_timestamp, 'data': None}
        fake_data = 200
        fake_parsed_data = (fake_data,)
        output_assert = {fake_telem_field: {
            'timestamp': fake_timestamp,
            'data': fake_data}
        }
        self.assertEqual(
            self.mcu._format_data(
                telem_field=fake_telem_field,
                input_dict=fake_input_dict,
                read_data=fake_read_data,
                parsed_data=fake_parsed_data
            ),
            output_assert)

    def test_format_data_multiitem(self):
        fake_telem_field = 'field_4'  # Has subfields
        fake_input_dict = mcu_api.TELEMETRY['module_1'][fake_telem_field]
        fake_timestamp = 100.00
        fake_read_data = {'timestamp': fake_timestamp, 'data': None}
        fake_data1 = 100
        fake_data2 = 200
        fake_parsed_data = (fake_data1, fake_data2)
        output_assert = {
            'subfield_1': {
                'timestamp': fake_timestamp,
                'data': fake_data1},
            'subfield_2': {
                'timestamp': fake_timestamp,
                'data': fake_data2}
        }
        self.assertEqual(
            self.mcu._format_data(
                telem_field=fake_telem_field,
                input_dict=fake_input_dict,
                read_data=fake_read_data,
                parsed_data=fake_parsed_data
            ),
            output_assert)

    def test_format_data_parsingrejection(self):
        fake_telem_field = 'field_4'  # Has subfields
        fake_input_dict = mcu_api.TELEMETRY['module_1'][fake_telem_field]
        fake_read_data = {'timestamp': 100.00, 'data': None}
        bad_parsed_data = ('whatever stuff',)
        with self.assertRaises(KeyError):
            self.mcu._format_data(
                telem_field=fake_telem_field,
                input_dict=fake_input_dict,
                read_data=fake_read_data,
                parsed_data=bad_parsed_data)

    def test_format_data_namesrejection(self):
        bad_telem_field = 'field_1'  # Single item
        fake_input_dict = mcu_api.TELEMETRY['module_1'][bad_telem_field]
        fake_read_data = {'timestamp': 100.00, 'data': None}
        fake_parsed_data = (  # Multiple items
            'whatever stuff',
            'whatever other stuff'
        )
        with self.assertRaises(KeyError):
            self.mcu._format_data(
                telem_field=bad_telem_field,
                input_dict=fake_input_dict,
                read_data=fake_read_data,
                parsed_data=fake_parsed_data)

    def test_format_data_lengthrejection(self):
        fake_telem_field = 'field_4'  # Single item
        fake_input_dict = mcu_api.TELEMETRY['module_1'][fake_telem_field]
        fake_read_data = {'timestamp': 100.00, 'data': None}
        bad_parsed_data = (  # More than number of subfields
            'whatever stuff',
            'whatever other stuff',
            'and things'
        )
        with self.assertRaises(KeyError):
            self.mcu._format_data(
                telem_field=fake_telem_field,
                input_dict=fake_input_dict,
                read_data=fake_read_data,
                parsed_data=bad_parsed_data)

    def test_read_telemetry(self):
        module = 'module_1'
        field = 'field_1'
        fields = [field]
        input_assert = {}
        input_assert[field] = mcu_api.TELEMETRY[module][field]
        with mock.patch('mcu_api.MCU._read_telemetry_items') as mock_read_telemetry_items:
            self.mcu.read_telemetry(
                module=module,
                fields=fields)
            mock_read_telemetry_items.assert_called_with(
                dict=input_assert)

    def test_read_telemetry_items(self):
        module = 'module_1'
        field = 'field_2'
        fields = [field]
        input_dict = {}
        input_dict[field] = mcu_api.TELEMETRY[module][field]
        output_data = 'this should be returned'
        fake_timestamp = 841489.94
        return_data = '\x01\x02\x03\x04\x05' + output_data + \
            '\0this \0should be \0cut off'
        output_assert = {field: {
            'timestamp': fake_timestamp,
            'data': output_data
        }}
        with mock.patch('mcu_api.MCU.write') as mock_write, mock.patch('mcu_api.MCU.read') as mock_read:
            mock_read.return_value = return_data
            self.assertEqual(
                self.mcu._read_telemetry_items(dict=input_dict),
                output_assert)


if __name__ == '__main__':
    unittest.main()
