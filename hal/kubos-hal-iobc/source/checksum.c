/*
 * Copyright (C) 2014 Innovative Solution In Space B.V.
 * Copyright (C) 2017 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "kubos-hal-iobc/checksum.h"

#define CRC8_POLYNOMIAL 0x07

static uint8_t supervisor_crctable[256];

void checksum_prepare_LUTCRC8(uint8_t polynomial, uint8_t * LUT)
{
    unsigned short i;
    uint8_t data;

    for (i = 0; i < 256; i++)
    {
        data = i;
        LUT[i] = checksum_calculate_CRC8(&data, 1, polynomial, 0x00, true);
    }
}

uint8_t checksum_calculate_CRC8LUT(const uint8_t * data, unsigned int length, const uint8_t * LUT, uint8_t start_remainder, bool endofdata)
{
    unsigned int i;

    for (i = 0; i < length; i++)
    {
        start_remainder = LUT[start_remainder] ^ data[i];
    }

    if (endofdata)
    {
        start_remainder = LUT[start_remainder] ^ 0x00;
    }

    return start_remainder;
}

uint8_t checksum_calculate_CRC8(const uint8_t * data, unsigned int length, uint8_t polynomial, uint8_t start_remainder, bool endofdata)
{
    uint8_t bit_mask, byte; // bit counter, XOR flag, bit mask, current byte
    uint8_t xor_flag;
    unsigned int i; // byte counter
    unsigned int total_length = length; // original length + one 0x00 bytes for end of data

    if (endofdata)
    {
        total_length += 1;
    }

    // Process all bytes
    for (i = 0; i < total_length; i++)
    {
        // Set bit mask for next byte
        bit_mask = 0x80;

        // Add two bytes with 0x00 after original data
        byte = 0x00;
        if (i < length)
        {
            byte = data[i];
        }

        // Process all bits
        while (bit_mask)
        {
            // Remember if MSB was a 1
            xor_flag = start_remainder & 0x80;

            // Left-shift remainder
            start_remainder = start_remainder << 1;

            // If current bit is a 1
            if (byte & bit_mask)
            {
                // Insert a 1 at right-most position of remainder
                start_remainder |= 0x01;
            }

            // If MSB was a 1
            if (xor_flag)
            {
                // XOR remainder with polynomial
                start_remainder ^= polynomial;
            }

            // Shift mask to process next bit
            bit_mask = bit_mask >> 1;
        }
    }

    // Return remainder
    return start_remainder;
}

uint8_t supervisor_calculate_CRC(const uint8_t * data, unsigned int length)
{
    unsigned int i = 0;
    uint8_t crcvalue = 0;
    checksum_prepare_LUTCRC8(CRC8_POLYNOMIAL, supervisor_crctable);

    for (i = 0; i < length; i++)
    {
        crcvalue = supervisor_crctable[data[i] ^ crcvalue];
    }
    return crcvalue;
}