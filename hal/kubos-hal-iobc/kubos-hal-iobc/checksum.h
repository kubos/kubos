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

/**
 * @defgroup iOBC-Checksum iOBC Supervisor CRC functions
 * @addtogroup iOBC-Checksum
 * @{
 */


#pragma once

#include <stdbool.h>
#include <stdint.h>

/**
 * Generate a LUT for CRC 8 calculations with a certain polynomial
 *
 * @param[in] polynomial 8-bit CRC polynomial to be used
 * @param[out] LUT Pointer to memory block where LUT can be stored, needs to be at least 256 * sizeof(uint8_t)
 */
void checksum_prepare_LUTCRC8(uint8_t polynomial, uint8_t * LUT);

/**
 * Calculates a CRC 8 checksum according to CRC-8 CCITT, using a LUT
 *
 * @param[in] data Pointer to data to calculate the checksum for.
 * @param[in] length Length of the data in bytes.
 * @param[in] LUT Pointer to LUT to use for CRC calculations, prepared using checksum_prepareLUTCRC8()
 * @param[in] start_remainder Remainder to start CRC calculation with
 * @param[in] endofdata Indicates whether this is the end of larger datablock (TRUE) or not yet (FALSE)
 * @return uint8_t 8-bit CRC checksum.
 */
uint8_t checksum_calculate_CRC8LUT(const uint8_t * data, unsigned int length, const uint8_t * LUT, uint8_t start_remainder, bool endofdata);

/**
 * Calculates a CRC 8 checksum according to CRC-8 CCITT, using bitwise calculation
 *
 * @param[in] data Pointer to data to calculate the checksum for.
 * @param[in] length Length of the data in bytes.
 * @param[in] polynomial 8-bit CRC polynomial to be used.
 * @param[in] start_remainder Remainder to start CRC calculation with
 * @param[in] endofdata Indicates whether this is the end of larger datablock (TRUE) or not yet (FALSE)
 * @return uint8_t 8-bit CRC checksum.
 */
uint8_t checksum_calculate_CRC8(const uint8_t * data, unsigned int length, uint8_t polynomial, uint8_t start_remainder, bool endofdata);

/**
 * Calculates a CRC checksum according to the algorithm used in the iOBC supervisor.
 * @param[in] data Pointer to data to calculate the checksum for.
 * @param[in] length Length of the data in bytes
 * @return uint8_t 8-bit CRC checksum.
 */
uint8_t supervisor_calculate_CRC(const uint8_t * data, unsigned int length);

/* @} */