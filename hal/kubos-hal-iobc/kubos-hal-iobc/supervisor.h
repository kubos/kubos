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
 * @defgroup iOBC-Supervisor iOBC Supervisor Interface
 * @addtogroup iOBC-Supervisor
 * @{
 */

#pragma once

#include <stdbool.h>
#include <stdint.h>

/** Length of emergency reset. */
#define LENGTH_EMERGENCY_RESET 10
/** Length of reset the IOBC PCU. */
#define LENGTH_RESET 3
/** Length of reset the IOBC PCU. */
#define LENGTH_POWER_CYCLE_IOBC 3
/** Length of the telemetry housekeeping request. */
#define LENGTH_TELEMETRY_HOUSEKEEPING 37
/** Length of the getting Version. */
#define LENGTH_TELEMETRY_GET_VERSION 34
/** Reply length of the telemetry housekeeping sample. */
#define LENGTH_TELEMETRY_SAMPLE_HOUSEKEEPING 3
/** Length of the sampling the Version. */
#define LENGTH_TELEMETRY_SAMPLE_VERSION 3
/** Length of the dummy. */
#define LENGTH_TELEMETRY_DUMMY 3
/** Length of the compile information. */
#define LENGTH_GENERIC_REPLY 3

/**
 * Generic reply from the Supervisor Controller.
 */
typedef union __attribute__((__packed__)) {
    /** Raw value of the generic reply bytes */
    uint8_t raw_value[LENGTH_GENERIC_REPLY];
    /** Individual reply fields */
    struct __attribute__((__packed__)) supervisor_generic_reply_fields_t
    {
        /** The first as always is a dummy byte */
        uint8_t dummy;
        /** The second is a SPI command status */
        uint8_t spi_command_status;
        /** CRC byte */
        uint8_t crc8;
    } /** Individual reply fields */ fields;
} supervisor_generic_reply_t;

/** Length of the compile information. */
#define LENGTH_COMPILE_INFORMATION 19

 

/**
 * Supervisor version and configuration bytes.
 *
 * Layout of fields:
 * @code
     _________________________________
    |                                 |
000 |             DUMMY               |
    |_________________________________|
    |                                 |
001 |        SPI COMMAND STATUS       |
    |_________________________________|
    |                                 |
002 |        INDEX OF SUBSYSTEM       |
    |_________________________________|
    |                                 |
003 |                                 |
... |           SW VERSION            |
005 |_________________________________|
    |                                 |
006 |                                 |
... |          HEAD REVISION          |
009 |                                 |
    |_________________________________|
    |                                 |
010 |          SERIAL NUMBER          |
    |_________________________________|
    |                                 |
    |                                 |
    |                                 |
012 |                                 |
... |       COMPILE INFORMATION       |
030 |                                 |
    |                                 |
    |                                 |
    |_________________________________|
    |                                 |
031 |           CLOCK SPEED           |
    |_________________________________|
    |                                 |
032 |         TEST / FLIGHT           |
    |_________________________________|
    |                                 |
033 |              CRC8               |
    |_________________________________|
 * @endcode
 */
typedef union __attribute__((__packed__)) {
    /** Raw value of the version configuration bytes */
    uint8_t raw_value[LENGTH_TELEMETRY_GET_VERSION];
    /** Individual version fields */
    struct __attribute__((__packed__)) supervisor_version_fields_t
    {
        /** The first as always is a dummy byte. */
        uint8_t dummy;
        /** The second is a SPI Command Status. */
        uint8_t spi_command_status;
        /** Index of ISIS Subsystem. */
        uint8_t index_of_subsystem;
        /** Software major version. */
        uint8_t major_version;
        /** Software minor version. */
        uint8_t minor_version;
        /** Software patch version. */
        uint8_t patch_version;
        /** Software git head version. */
        uint32_t git_head_version;
        /** Serial number. */
        uint16_t serial_number;
        /** Compile information (time and date). */
        int8_t compile_information[LENGTH_COMPILE_INFORMATION];
        /** Clock speed of the Supervisor Controller (in MHz). */
        uint8_t clock_speed;
        /** Code type. Whether flight or test. */
        int8_t code_type;
        /** CRC byte. */
        uint8_t crc8;
    } /** Individual version fields */ fields;
} supervisor_version_t;


/**
 * Enable status structure.
 *
 * Layout of fields:
 * @code
     ____________________________________________
    |                                            |
000 |                   DUMMY                    |
    |____________________________________________|
    |                                            |
001 |             SPI COMMAND STATUS             |
    |____________________________________________|
    |                                            |
002 |               ENABLE STATUS                |
    |____________________________________________|
    |                                            |
003 |                                            |
... |        SUPERVISOR CONTROLLER UPTIME        |
006 |                                            |
    |____________________________________________|
    |                                            |
007 |                                            |
... |                 IOBC UPTIME                |
010 |                                            |
    |____________________________________________|
    |                                            |
011 |                                            |
... |               IOBC RESET COUNT             |
014 |                                            |
    |____________________________________________|
    |                                            |
    |                                            |
015 |                                            |
... |                  ADC DATA                  |
034 |                                            |
    |                                            |
    |                                            |
    |____________________________________________|
    |                                            |
035 |               ADC UPDATE FLAG              |
    |____________________________________________|
    |                                            |
036 |                   CRC8                     |
    |____________________________________________|
 * @endcode
 */
typedef union __attribute__((__packed__)) {
    /** Raw value of the version configuration bytes. */
    uint8_t raw_value;
    /** Individual enable status fields */
    struct __attribute__((__packed__)) supervisor_enable_status_fields_t
    {
                
        uint8_t
                /** OBC Power. */
                power_obc : 1, 
                /** Output power to the RTC. */
                power_rtc : 1,
                /** Supervisor mode. */
                is_in_supervisor_mode : 1, 
                : 1, : 1,
                /** RTC is busy. */
                busy_rtc : 1,
                /** RTC is doing power off. */
                power_off_rtc : 1,
                : 1;
    } /** Individual enable status fields */ fields;
} supervisor_enable_status_t;

/** The number of channels used in the Supervisor Controller. */
#define SUPERVISOR_NUMBER_OF_ADC_CHANNELS 10

/**
 * Supervisor housekeeping.
 */
typedef union __attribute__((__packed__))  {
    /** Raw value of the version configuration bytes */
    uint8_t raw_value[LENGTH_TELEMETRY_HOUSEKEEPING];
    /** Individual housekeeping fields */
    struct __attribute__((__packed__)) supervisor_housekeeping_fields_t
    {
        /** The first as always is a dummy byte. */
        uint8_t dummy;
        /** The second is a SPI Command Status. */
        uint8_t spi_command_status;
        /** Enable status of the Supervisor Controller. */
        supervisor_enable_status_t enable_status;
        /** Supervisor Controller Uptime. */
        uint32_t supervisor_uptime;
        /** IOBC Uptime as measured by Supervisor Controller. */
        uint32_t iobc_uptime;
        /** IOBC Reset Count. */
        uint32_t iobc_reset_count; 
        /** ADC Data. */
        uint16_t adc_data[SUPERVISOR_NUMBER_OF_ADC_CHANNELS];
        /** ADC Update Flag. */
        uint8_t adc_update_flag;
        /** CRC byte. */
        uint8_t crc8;
    } /** Individual housekeeping fields */ fields;
} supervisor_housekeeping_t;

/**
 * @brief Performs a software reset of the microcontroller directly without shutting down its components.
 * As this command is considered unsafe for the hardware and the software of the IOBC-S, use supervisor_reset() instead.
 *
 * @return true if command is sent successfully, otherwise false
 */
bool supervisor_emergency_reset();

/**
 * @brief Assert a reset to the IOBC-S and make sure that the conditions prior to reset operations are met.
 *
 * @return true if command is sent successfully, otherwise false
 */
bool supervisor_reset();

/**
 * @brief Let the IOBC be power-cycled for around 4-5 seconds.
 * Supervisor Controller power is not affected by this command.
 *
 * @return true if command is sent successfully, otherwise false
 */
bool supervisor_powercycle();

/**
 * @brief Getting Version from Supervisor Controller.
 *
 * @param[out] version Version and configuration read back from the Supervisor Controller.
 * @return true if command is sent and response has valid CRC, otherwise false
 */
bool supervisor_get_version(supervisor_version_t * version);

/**
 * @brief Getting Housekeeping from Supervisor Controller.
 *
 * @param[out] housekeeping Housekeeping read back from the Supervisor Controller.
 * @return true if command is sent and response has valid CRC, otherwise false
 */
bool supervisor_get_housekeeping(supervisor_housekeeping_t * housekeeping);

/* @} */