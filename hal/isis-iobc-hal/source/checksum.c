#include "isis-iobc-hal/checksum.h"

static unsigned char supervisor_crctable[256];

void checksum_prepare_LUTCRC8(unsigned char polynomial, unsigned char * LUT)
{
    unsigned short i;
    unsigned char data;

    for (i = 0; i < 256; i++)
    {
        data = i;
        LUT[i] = checksum_calculate_CRC8(&data, 1, polynomial, 0x00, true);
    }
}

unsigned char checksum_calculate_CRC8LUT(unsigned char * data, unsigned int length, unsigned char * LUT, unsigned char start_remainder, bool endofdata)
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

unsigned char checksum_calculate_CRC8(unsigned char * data, unsigned int length, unsigned char polynomial, unsigned char start_remainder, bool endofdata)
{
    unsigned char bit_mask, byte; // bit counter, XOR flag, bit mask, current byte
    unsigned char xor_flag;
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

unsigned char supervisor_calculate_CRC(unsigned char * data, unsigned int length)
{
    unsigned int i = 0;
    unsigned char crcvalue = 0;
    checksum_prepare_LUTCRC8(CRC8_POLYNOMIAL, supervisor_crctable);

    for (i = 0; i < length; i++)
    {
        crcvalue = supervisor_crctable[data[i] ^ crcvalue];
    }
    return crcvalue;
}