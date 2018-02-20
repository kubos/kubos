/*
 * Copyright (C) 2018 Kubos Corporation
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
 * @defgroup NANOPOWER_API GOMspace NanoPower API
 * @addtogroup NANOPOWER_API
 * @{
 */

#pragma once

#include <stdbool.h>
#include <stdint.h>

/**
 *  @name GOMspace NanoPower config.json configuration options and default
 * 		  values
 */
/**@{*/
/**
 * I2C bus the EPS is connected to
 */
#ifdef YOTTA_CFG_EPS_NANOPOWER_I2C_BUS
#define EPS_I2C_BUS YOTTA_CFG_EPS_NANOPOWER_I2C_BUS
#else
#define EPS_I2C_BUS K_I2C1
#endif

/**
 * Primary antenna controller's I2C address
 */
#ifdef YOTTA_CFG_EPS_NANOPOWER_ADDR
#define EPS_ADDR YOTTA_CFG_EPS_NANOPOWER_ADDR
#else
#define EPS_ADDR 0x02
#endif

/**
 * Watchdog timeout (in hours)
 */
#ifdef YOTTA_CFG_EPS_NANOPOWER_WATCHDOG_TIMEOUT
#define EPS_WD_TIMEOUT YOTTA_CFG_EPS_NANOPOWER_WATCHDOG_TIMEOUT
#else
#define EPS_WD_TIMEOUT 60
#endif
/**@}*/

/** \cond WE DO NOT WANT TO HAVE THESE IN OUR GENERATED DOCS */
/* EPS command values */
#define PING                0x01
#define REBOOT              0x04
#define GET_HOUSEKEEPING    0x08
#define SET_OUTPUT          0x09
#define SET_SINGLE_OUTPUT   0x10
#define SET_PV_VOLT         0x11
#define SET_PV_AUTO         0x12
#define SET_HEATER          0x13
#define RESET_COUNTERS      0x15
#define RESET_WDT           0x16 /* Reset DEDICATED WDT (not I2C WDT) */
#define CMD_CONFIG1         0x17 /* Currently only used for resetting config */
#define GET_CONFIG1         0x18
#define SET_CONFIG1         0x19
#define HARD_RESET          0x20 /* 400ms delay after reset */
#define CMD_CONFIG2         0x21 /* Reset default config or confirm current config */
#define GET_CONFIG2         0x22
#define SET_CONFIG2         0x23

/** \endcond */

/**
 * EPS function return values
 */
typedef enum {
    EPS_OK,                     /**< Requested function completed successfully */
    EPS_ERROR,                  /**< Generic error */
    EPS_ERROR_CONFIG,           /**< Configuration error */
    EPS_ERROR_NO_RESPONSE,      /**< No response received from subsystem */
    EPS_ERROR_INTERNAL,         /**< An error was thrown by the subsystem */
    EPS_ERROR_NOT_IMPLEMENTED   /**< Requested function has not been implemented for the subsystem */
} KEPSStatus;

typedef enum {
    HK_PU31_6,
    HK_PU31_8,
    HK_VOLTAGE_CURRENT,
    HK_OUTPUT_SWITCH,
    HK_WATCHDOG,
    HK_BASIC
} KEPSHousekeepingType;

/**
 * Response header structure
 */
typedef struct
{
    uint8_t cmd;                /**< Command which produced this response */
    uint8_t status;             /**< Status/Error byte */
} __attribute__((packed)) eps_resp_header;

/* System Config */
typedef struct
{
    uint8_t  ppt_mode;                      /**< Mode for PPT [1 = AUTO, 2 = FIXED] */
    uint8_t  battheater_mode;               /**< Mode for battheater [0 = Manual, 1 = Auto] */
    int8_t   battheater_low;                /**< Turn heater on at [degC] */
    int8_t   battheater_high;               /**< Turn heater off at [degC] */
    uint8_t  output_normal_value[8];        /**< Nominal mode output value */
    uint8_t  output_safe_value[8];          /**< Safe mode output value */
    uint16_t output_initial_on_delay[8];    /**< Output switches: init with these on delays [s] */
    uint16_t output_initial_off_delay[8];   /**< Output switches: init with these off delays [s] */
    uint16_t vboost[3];                     /**< Fixed PPT point for boost converters [mV] */
} __attribute__((packed)) eps_system_config_t;

/* Battery mode configuration (Config2) */
typedef struct
{
    uint16_t batt_maxvoltage;               /**< Voltage threshold to be in FULL mode */
    uint16_t batt_safevoltage;              /**< Voltage threshold to trigger NORMAL -> SAFE mode */
    uint16_t batt_criticalvoltage;          /**< Lowest allowable voltage (-> CRITICAL mode) */
    uint16_t batt_normalvoltage;            /**< Voltage threshold to trigger SAFE -> NORMAL mode */
    uint32_t reserved1[2];
    uint8_t  reserved2[4];
} __attribute__((packed)) eps_battery_config_t;

// TODO: check if this should be packed (it's not in their docs)
/**
 *  @name Housekeeping - P31u-6
 */
/**@{*/
/**
 * Backwards-compatible housekeeping
 */
typedef struct
{
    uint16_t pv[3];                         //Photo-voltaic input voltage [mV]
    uint16_t pc;                            //Total photo current [mA]
    uint16_t bv;                            //Battery voltage [mV]
    uint16_t sc;                            //Total system current [mA]
    int16_t temp[4];                        //Temp. of boost converters (1,2,3) and onboard battery [degC]
    int16_t batt_temp[2];                   //External board battery temperatures [degC];
    /**
     * Number of latch-ups on each output 5V and +3V3 channel
     * Order[5V1 5V2 5V3 3.3V1 3.3V2 3.3V3]
     * Transmit as 5V1 first and 3.3V3 last
     */
    uint16_t latchup[6];
    uint8_t  reset;                         //Cause of last EPS reset
    uint16_t bootcount;                     //Number of EPS reboots
    uint16_t sw_errors;                     //Number of errors in the eps software
    uint8_t  ppt_mode;                      //1 = MPPT, 2 = Fixed SW PPT.
    /**
     * Mask of output channel status, 1=on, 0=off
     * MSB - [QH QS 3.3V3 3.3V2 3.3V1 5V3 5V2 5V1] - LSB
     * QH = Quadbat heater, QS = Quadbat switch
     */
    uint8_t channel_status;
} hkparam_t;
/**@}*/

/**
 *  @name Housekeeping - P31u-8 and P31u-9
 */
/**@{*/
/**
 *
 */
/**
 * P31u-8 housekeeping
 */
typedef struct
{
    uint16_t vboost[3];                     //! Voltage of boost converters [mV] [PV1, PV2, PV3]
    uint16_t vbatt;                         //! Voltage of battery [mV]
    uint16_t curin[3];                      //! Current in [mA]
    uint16_t cursun;                        //! Current from boost converters [mA]
    uint16_t cursys;                        //! Current out of battery [mA]
    uint16_t reserved1;                     //! Reserved for future use
    uint16_t curout[6];                     //! Current out (switchable outputs) [mA]
    uint8_t output[8];                      //! Status of outputs**
    uint16_t output_on_delta[8];            //! Time till power on** [s]
    uint16_t output_off_delta[8];           //! Time till power off** [s]
    uint16_t latchup[6];                    //! Number of latch-ups
    uint32_t wdt_i2c_time_left;             //! Time left on I2C wdt [s]
    uint32_t wdt_gnd_time_left;             //! Time left on I2C wdt [s]
    uint8_t wdt_csp_pings_left[2];          //! Pings left on CSP wdt
    uint32_t counter_wdt_i2c;               //! Number of WDT I2C reboots
    uint32_t counter_wdt_gnd;               //! Number of WDT GND reboots
    uint32_t counter_wdt_csp[2];            //! Number of WDT CSP reboots
    uint32_t counter_boot;                  //! Number of EPS reboots
    int16_t temp[6];                        //! Temperatures [degC] [0 = TEMP1, TEMP2, TEMP3, TEMP4, BP4a, BP4b]
    uint8_t bootcause;                      //! Cause of last EPS reset
    uint8_t battmode;                       //! Mode for battery [0 = initial, 1 = undervoltage, 2 = safemode, 3 = nominal, 4=full]
    uint8_t pptmode;                        //! Mode of PPT tracker [1=MPPT, 2=FIXED]
    uint16_t reserved2;
} __attribute__((packed)) eps_hk_t;

/**
 * Voltage and current housekeeping
 */
typedef struct
{
    uint16_t vboost[3];                     //! Voltage of boost converters [mV] [PV1, PV2,PV3]
    uint16_t vbatt;                         //! Voltage of battery [mV]
    uint16_t curin[3];                      //! Current in [mA]
    uint16_t cursun;                        //! Current from boost converters [mA]
    uint16_t cursys;                        //! Current out of battery [mA]
    uint16_t reserved1;                     //! Reserved for future use
} __attribute__((packed)) eps_hk_vi_t;

/**
 * Output switch housekeeping
 */
typedef struct
{
    uint16_t curout[6];                     //! Current out (switchable outputs) [mA]
    uint8_t output[8];                      //! Status of outputs**
    uint16_t output_on_delta[8];            //! Time till power on** [s]
    uint16_t output_off_delta[8];           //! Time till power off** [s]
    uint16_t latchup[6];                    //! Number of latch-ups
} __attribute__((packed)) eps_hk_out_t;

/**
 * Watchdog housekeeping
 */
typedef struct
{
    uint32_t wdt_i2c_time_left;             //! Time left on I2C wdt [s]
    uint32_t wdt_gnd_time_left;             //! Time left on I2C wdt [s]
    uint8_t wdt_csp_pings_left[2];          //! Pings left on CSP wdt
    uint32_t counter_wdt_i2c;               //! Number of WDT I2C reboots
    uint32_t counter_wdt_gnd;               //! Number of WDT GND reboots
	uint32_t counter_wdt_csp[2];            //! Number of WDT CSP reboots
} __attribute__((packed)) eps_hk_wdt_t;

/**
 * Basic housekeeping
 */
typedef struct
{
    uint32_t counter_boot;                  //! Number of EPS reboots
    int16_t temp[6];                        //! Temperatures [degC] [0 = TEMP1, TEMP2, TEMP3, TEMP4, BATT0, BATT1]
    uint8_t bootcause;                      //! Cause of last EPS reset
    uint8_t battmode;                       //! Mode for battery [0 = initial, 1 = undervoltage, 2 = safemode, 3 = nominal, 4=full]
    uint8_t pptmode;                        //! Mode of PPT tracker [1=MPPT, 2=FIXED]
    uint16_t reserved2;
} __attribute__((packed)) eps_hk_basic_t;

/**@}*/

/*
 * Public Functions
 */
/**
 * Initialize the antenna interface
 * @return KEPSStatus EPS_OK if OK, error otherwise
 */
KEPSStatus k_eps_init(void);
/**
 * Terminate the antenna interface
 */
void k_eps_terminate(void);
/**
 * Configure the antenna
 * @param [in] config Pointer to EPS configuration values
 * @return KEPSStatus EPS_OK if OK, error otherwise
 */
KEPSStatus k_eps_configure_system(const eps_system_config_t * config);
KEPSStatus k_eps_configure_battery(const eps_battery_config_t * config);
KEPSStatus k_eps_ping(void);
/**
 * Hard reset the NanoPower
 * It will wait 400ms before repowering
 * @return KEPSStatus EPS_OK if OK, error otherwise
 */
KEPSStatus k_eps_reset(void);
KEPSStatus k_eps_reboot(void);
KEPSStatus k_eps_set_output(uint8_t channel_mask);
KEPSStatus k_eps_set_single_output(uint8_t channel, uint8_t value,
                                   int16_t delay);
KEPSStatus k_eps_set_input_value(uint16_t in1_voltage, uint16_t in2_voltage,
                                 uint16_t in3_voltage);
KEPSStatus k_eps_set_input_mode(uint8_t mode);
KEPSStatus k_eps_set_heater(uint8_t cmd, uint8_t header, uint8_t mode);
KEPSStatus k_eps_get_housekeeping(eps_hk_t * buff);
KEPSStatus k_eps_get_system_config(eps_system_config_t * buff);
KEPSStatus k_eps_get_battery_config(eps_battery_config_t * buff);
KEPSStatus k_eps_get_heater(uint8_t * bp4, uint8_t * onboard);
/**
 * Kick the EPS's watchdogs once
 * @return KEPSStatus `EPS_OK` if OK, error otherwise
 */
KEPSStatus k_eps_watchdog_kick(void);
/**
 * Start a thread to kick the EPS's watchdogs at an interval of
 * (::EPS_WD_TIMEOUT/2) hours
 * @return KEPSStatus `EPS_OK` if OK, error otherwise
 */
KEPSStatus k_eps_watchdog_start(void);
/**
 * Stop the watchdog thread
 * @return KEPSStatus `EPS_OK` if OK, error otherwise
 */
KEPSStatus k_eps_watchdog_stop(void);
/**
 * Pass a command packet directly through to the EPS.
 * Useful for executing commands which have not been implemented in either the
 * generic or specific antenna APIs.
 * @param [in]  tx      Pointer to command packet to send
 * @param [in]  tx_len  Size of command packet
 * @param [out] rx      Pointer to storage for command response
 * @param [in]  rx_len  Expected length of command response
 * @return KEPSStatus EPS_OK if OK, error otherwise
 */
KEPSStatus k_eps_passthrough(const uint8_t * tx, int tx_len, uint8_t * rx,
                             int rx_len);

/*
 * Internal Functions
 */

KEPSStatus kprv_eps_transfer(const uint8_t * tx, int tx_len, uint8_t * rx,
                             int rx_len);

/* @} */
