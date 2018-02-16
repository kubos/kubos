/*
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
 * @addtogroup IMTQ_API
 * @{
 */

#pragma once

#include <json/json.h>
#include "imtq-core.h"

/** \cond WE DO NOT WANT TO HAVE THESE IN OUR GENERATED DOCS */
/* Data Request Commands */
#define GET_STATE       0x41
#define GET_MTM_RAW     0x42
#define GET_MTM_CALIB   0x43
#define GET_CURRENT     0x44
#define GET_TEMPS       0x45
#define GET_DIPOLE      0x46
#define GET_TEST        0x47
#define GET_DETUMBLE    0x48
#define GET_HOUSE_RAW   0x49
#define GET_HOUSE_ENG   0x4A
/** \endcond */

/**
 *  @name Self-Test Error Byte Flags
 */
/**@{*/
#define TEST_ERROR_I2C  0x01    /**< I<sup>2</sup>C Failure */
#define TEST_ERROR_SPI  0x02    /**< SPI Failure (MTM connectivity) */
#define TEST_ERROR_ADC  0x04    /**< ADC Failure (current/temp measurement) */
#define TEST_ERROR_PWM  0x08    /**< PWM Failure (coil actuation) */
#define TEST_ERROR_TC   0x10    /**< System Failure */
#define TEST_ERROR_MTM  0x20    /**< MTM values outside of expected range */
#define TEST_ERROR_COIL 0x40    /**< Coil currents outside of expected range */
/**@}*/

/**
 * Categories of telemetry which can be returned by `k_adcs_get_telemetry`
 */
typedef enum {
    NOMINAL,                    /**< System state, all system measurements */
    DEBUG                       /**< System state, current configuration, last test results */
} ADCSTelemType;

/**
 * System uptime returned by `k_adcs_get_power_status`
 */
typedef uint32_t   adcs_power_status;

/**
 * iMTQ system state data returned by ::k_imtq_get_system_state
 */
typedef struct {
    imtq_resp_header hdr;       /**< Response message header */
    uint8_t mode;               /**< Current system mode */
    uint8_t error;              /**< Error encountered during previous interation */
    uint8_t config;             /**< Parameter updated since system startup? 0 - No, 1 - Yes */
    uint32_t uptime;            /**< System uptime in seconds */
} __attribute__((packed)) imtq_state;

/**
 * MTM measurements data structure
 */
typedef struct {
    int32_t x;                  /**< X-axis */
    int32_t y;                  /**< Y-axis */
    int32_t z;                  /**< Z-axis */
} __attribute__((packed)) imtq_mtm_data;

/**
 * MTM measurement returned by ::k_imtq_get_raw_mtm and ::k_imtq_get_calib_mtm
 */
typedef struct {
    imtq_resp_header hdr;        /**< Response message header */
    imtq_mtm_data data;          /**< MTM measurement data. Units dependent on function used */
    uint8_t act_status;          /**< Coils actuation status during measurement. 0 - Not actuating, 1 - Actuating */
} __attribute__((packed)) imtq_mtm_msg;

/**
 * Generic structure for messages relating to the axes
 */
typedef struct {
    imtq_resp_header hdr;                /**< Response message header */
    imtq_axis_data data;                 /**< Axes data */
} __attribute__((packed)) imtq_axis_msg;

typedef imtq_axis_msg imtq_coil_current;  /**< Coil currents in [10<sup>-4</sup> A] returned by ::k_imtq_get_coil_current */
typedef imtq_axis_msg imtq_coil_temp;     /**< Coil temperatures in [<sup>o</sup>C] returned by ::k_imtq_get_coil_temps */
typedef imtq_axis_msg imtq_dipole;        /**< Commanded actuation dipole in [10<sup>-4</sup> Am<sup>2</sup>] returned by ::k_imtq_get_dipole */

/**
 * Self-test step result structure
 */
typedef struct {
    imtq_resp_header hdr;                /**< Response message header */
    uint8_t error;                       /**< Return code for the step */
    uint8_t step;                        /**< Axis being tested */
    imtq_mtm_data mtm_raw;               /**< Raw MTM data in [7.5*10<sup>-9</sup> T] per count */
    imtq_mtm_data mtm_calib;             /**< Calibrated MTM data in [10<sup>-9</sup> T] */
    imtq_axis_data coil_current;         /**< Coil currents in [10<sup>-4</sup> A] */
    imtq_axis_data coil_temp;            /**< Coil temperatures in [<sup>o</sup>C] */
} __attribute__((packed)) imtq_test_result;

/**
 * Self-test single-axis result structure returned by
 * ::k_imtq_get_test_results_single
 */
typedef struct {
    imtq_test_result init;               /**< Measurements before actuation */
    imtq_test_result step;               /**< Measurements during actuation of requested axis */
    imtq_test_result final;              /**< Measurements after actuation */
} __attribute__((packed)) imtq_test_result_single;

/**
 * Self-test all-axes result structure returned by
 * ::k_imtq_get_test_results_all
 */
typedef struct {
    imtq_test_result init;               /**< Measurements before actuation */
    imtq_test_result x_pos;              /**< Measurements during actuation of positive x-axis */
    imtq_test_result x_neg;              /**< Measurements during actuation of negative x-axis */
    imtq_test_result y_pos;              /**< Measurements during actuation of positive y-axis */
    imtq_test_result y_neg;              /**< Measurements during actuation of negative y-axis */
    imtq_test_result z_pos;              /**< Measurements during actuation of positive z-axis */
    imtq_test_result z_neg;              /**< Measurements during actuation of negative z-axis */
    imtq_test_result final;              /**< Measurements after actuation */
} __attribute__((packed)) imtq_test_result_all;

/**
 * Detumble data returned by ::k_imtq_get_detumble
 */
typedef struct {
    imtq_resp_header hdr;                /**< Response message header */
    imtq_mtm_data mtm_calib;             /**< Calibrated MTM data in [10<sup>-9</sup> T] */
    imtq_mtm_data mtm_filter;            /**< Filtered MTM data in [10<sup>-9</sup> T] */
    imtq_mtm_data bdot;                  /**< B-Dot in [10<sup>-9</sup> T*s<sup>-1</sup>] */
    imtq_axis_data dipole;               /**< Commanded actuation dipole in [10<sup>-4</sup> Am<sup>2</sup>] */
    imtq_axis_data cmd_current;          /**< Command current in [10<sup>-4</sup> A] */
    imtq_axis_data coil_current;         /**< Coil currents in [10<sup>-4</sup> A] */
} __attribute__((packed)) imtq_detumble;

/**
 * Generic structure for raw ADC data relating to the axes
 */
typedef struct {
    int16_t x;                          /**< X-axis */
    int16_t y;                          /**< Y-axis */
    int16_t z;                          /**< Z-axis */
} __attribute__((packed)) imtq_axis_data_raw;

/**
 * Housekeeping data (raw ADC values) returned by
 * ::k_imtq_get_raw_housekeeping
 */
typedef struct {
    imtq_resp_header hdr;               /**< Response message header */
    uint16_t voltage_d;                 /**< Digital supply voltage */
    uint16_t voltage_a;                 /**< Analog supply voltage */
    uint16_t current_d;                 /**< Digital supply current */
    uint16_t current_a;                 /**< Analog supply current */
    imtq_axis_data_raw coil_current;    /**< Coil currents */
    imtq_axis_data_raw coil_temp;       /**< Coil temperatures */
    uint16_t mcu_temp;                  /**< MCU temperature */
} __attribute__((packed)) imtq_housekeeping_raw;

/**
 * Housekeeping data (engineering values) returned by
 * ::k_imtq_get_eng_housekeeping
 */
typedef struct {
    imtq_resp_header hdr;               /**< Response message header */
    uint16_t voltage_d;                 /**< Digital supply voltage in [mV] */
    uint16_t voltage_a;                 /**< Analog supply voltage in [mV] */
    uint16_t current_d;                 /**< Digital supply current in [10<sup>-4</sup> A] */
    uint16_t current_a;                 /**< Analog supply current in [10<sup>-4</sup> A] */
    imtq_axis_data coil_current;        /**< Coil currents in [10<sup>-4</sup> A] */
    imtq_axis_data coil_temp;           /**< Coil temperatures in [<sup>o</sup>C] */
    int16_t mcu_temp;                   /**< MCU temperature in [<sup>o</sup>C] */
} __attribute__((packed)) imtq_housekeeping_eng;

/* Data Request Commands */
/**
 * Get iMTQ system state
 * @param [out] state Pointer to storage for state data
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus k_imtq_get_system_state(imtq_state * state);
/**
 * Get raw data values from MTM
 *
 * Measurement units are in [7.5 * 10<sup>-9</sup> T], as documented in the <a href="http://www.compotrade.ru/Components/Sensixs/XEN1210.pdf">XEN1210 datasheet</a>
 * @note The ::k_imtq_start_measurement function must have been called in 
 * order for this function to be able to retrieve data
 * @param [out] data Pointer to storage for data
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus k_imtq_get_raw_mtm(imtq_mtm_msg * data);
/**
 * Get calibrated data values from MTM
 *
 * Measurement units are in [10<sup>-9</sup> T]
 * @note The ::k_imtq_start_measurement function must have been called in
 * order for this function to be able to retrieve data
 * @param [out] data Pointer to storage for data
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus k_imtq_get_calib_mtm(imtq_mtm_msg * data);
/**
 * Get coil currents
 * @param [out] data Pointer to storage for data
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus k_imtq_get_coil_current(imtq_coil_current * data);
/**
 * Get coil temperatures
 * @param [out] data Pointer to storage for data
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus k_imtq_get_coil_temps(imtq_coil_temp * data);
/**
 * Get commanded actuation dipole
 * @note Only applies to coil actuations started with
 * ::k_imtq_start_actuation_dipole.
 * Can also be used in detumble mode to retrieve the latest actuation dipole
 * @param [out] data Pointer to storage for data
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus k_imtq_get_dipole(imtq_dipole * data);
/**
 * Get results from a single-axis self-test
 *
 * The test can be started with ::k_imtq_start_test
 * @param [out] data Pointer to storage for data
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus k_imtq_get_test_results_single(imtq_test_result_single * data);
/**
 * Get results from an all-axes self-test
 *
 * The test can be started with ::k_imtq_start_test by specifying ::TEST_ALL
 * @param [out] data Pointer to storage for data
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus k_imtq_get_test_results_all(imtq_test_result_all * data);
/**
 * Get detumble data
 * @param [out] data Pointer to storage for data
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus k_imtq_get_detumble(imtq_detumble * data);
/**
 * Get the housekeeping data of the iMTQ as raw ADC values
 * @param [out] data Pointer to storage for data
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus k_imtq_get_raw_housekeeping(imtq_housekeeping_raw * data);
/**
 * Get the housekeeping data of the iMTQ in interpreted engineering units
 * @param [out] data Pointer to storage for data
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus k_imtq_get_eng_housekeeping(imtq_housekeeping_eng * data);

/* Private functions */
/**
 * Get the current system status and add it to the telemetry JSON
 * @param [out] buffer Pointer to telemetry JSON structure
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus kprv_adcs_get_status_telemetry(JsonNode * buffer);
/**
 * Get the current system measurements and add them to the telemetry JSON
 * @param [out] buffer Pointer to telemetry JSON structure
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus kprv_adcs_get_nominal_telemetry(JsonNode * buffer);
/**
 * Get the current system configuration and last self-test results and add them to the telemetry JSON
 * @param [out] buffer Pointer to telemetry JSON structure
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
KADCSStatus kprv_adcs_get_debug_telemetry(JsonNode * buffer);
/**
 * Add a self-test result to the requested JSON structure
 * @param [out] parent Pointer to JSON structure results should be added to
 * @param [in] test Self-test results to be parsed into JSON
 * @return KADCSStatus `ADCS_OK` if OK, error otherwise
 */
void kprv_adcs_process_test(JsonNode * parent, imtq_test_result test);

/* @} */
