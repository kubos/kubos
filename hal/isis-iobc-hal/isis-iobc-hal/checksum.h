/** @file checksum.h
 * Collection of checksum functions
 */

#pragma once

#include <stdbool.h>

#define CRC8_DEFAULT_STARTREMAINDER 0xFF
#define CRC8_POLYNOMIAL 0x07

/**
 * Generate a LUT for CRC 8 calculations with a certain polynomial
 *
 * @param[in] polynomial 8-bit CRC polynomial to be used
 * @param[out] LUT Pointer to memory block where LUT can be stored, needs to be at least 256 * sizeof(unsigned char)
 */
void checksum_prepare_LUTCRC8(unsigned char polynomial, unsigned char * LUT);

/**
 * Calculates a CRC 8 checksum according to CRC-8 CCITT, using a LUT
 *
 * @param[in] data Pointer to data to calculate the checksum for.
 * @param[in] length Length of the data in bytes.
 * @param[in] LUT Pointer to LUT to use for CRC calculations, prepared using checksum_prepareLUTCRC8()
 * @param[in] start_remainder Remainder to start CRC calculation with
 * @param[in] endofdata Indicates whether this is the end of larger datablock (TRUE) or not yet (FALSE)
 * @return 8-bit CRC checksum.
 */
unsigned char checksum_calculate_CRC8LUT(unsigned char * data, unsigned int length, unsigned char * LUT, unsigned char start_remainder, bool endofdata);

/**
 * Calculates a CRC 8 checksum according to CRC-8 CCITT, using bitwise calculation
 *
 * @param[in] data Pointer to data to calculate the checksum for.
 * @param[in] length Length of the data in bytes.
 * @param[in] polynomial 8-bit CRC polynomial to be used.
 * @param[in] start_remainder Remainder to start CRC calculation with
 * @param[in] endofdata Indicates whether this is the end of larger datablock (TRUE) or not yet (FALSE)
 * @return 8-bit CRC checksum.
 */
unsigned char checksum_calculate_CRC8(unsigned char * data, unsigned int length, unsigned char polynomial, unsigned char start_remainder, bool endofdata);

/**
 * Calculates a CRC checksum according to the algorithm used in the iOBC supervisor.
 * @param[in] data Pointer to data to calculate the checksum for.
 * @param[in] length Length of the data in bytes
 * @return 8-bit CRC checksum.
 */
unsigned char supervisor_calculate_CRC(unsigned char * data, unsigned int length);