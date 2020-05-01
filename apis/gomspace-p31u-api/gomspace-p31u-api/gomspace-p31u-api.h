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

#include <i2c.h>
#include <stdbool.h>
#include <stdint.h>

/** \cond We don't really need to have these in our docs */
/* EPS command values */
#define PING                1
#define REBOOT              4
#define GET_HOUSEKEEPING    8
#define SET_OUTPUT          9
#define SET_SINGLE_OUTPUT   10
#define SET_PV_VOLT         11
#define SET_PV_AUTO         12
#define SET_HEATER          13
#define RESET_COUNTERS      15
#define RESET_WDT           16 /* Reset DEDICATED WDT (not I2C WDT) */
#define CMD_CONFIG1         17 /* Currently only used for resetting config */
#define GET_CONFIG1         18
#define SET_CONFIG1         19
#define HARD_RESET          20 /* 400ms delay after reset */
#define CMD_CONFIG2         21 /* Reset default config or confirm current config */
#define GET_CONFIG2         22
#define SET_CONFIG2         23

/** \endcond */

/**
 * EPS function return values
 */
typedef enum {
    EPS_OK,                                 /**< Requested function completed successfully */
    EPS_ERROR,                              /**< Generic error */
    EPS_I2C_ERROR,                          /**< I2C error */
    EPS_ERROR_CONFIG,                       /**< Configuration error */
    EPS_ERROR_INTERNAL                      /**< An error was thrown by the subsystem */
} KEPSStatus;

/**
 * Kubos -> EPS Configuration
 */
typedef struct
{
    char * bus;                             /**< I2C bus device EPS is connected to */
    uint8_t addr;                           /**< EPS I2C slave address */
} KEPSConf;

/**
 * Response header structure
 */
typedef struct
{
    uint8_t cmd;                            /**< Command which produced this response */
    uint8_t status;                         /**< Status/Error byte */
} __attribute__((packed)) eps_resp_header;

/**
 *  System configuration
 */
typedef struct
{
    uint8_t  ppt_mode;                      /**< Mode for power-point tracking [1 = Automatic maximum, 2 = Fixed] */
    uint8_t  battheater_mode;               /**< Mode for battery heater activation [0 = Manual, 1 = Auto] */
    int8_t   battheater_low;                /**< Turn heater on at [degC] (auto mode) */
    int8_t   battheater_high;               /**< Turn heater off at [degC] (auto mode) */
    uint8_t  output_normal_value[8];        /**< Normal mode output values [0 = Off, 1 = On] */
    uint8_t  output_safe_value[8];          /**< Safe mode output values [0 = Off, 1 = On] */
    uint16_t output_initial_on_delay[8];    /**< Default output power on delays [seconds] */
    uint16_t output_initial_off_delay[8];   /**< Default output power off delays [seconds] */
    uint16_t vboost[3];                     /**< Fixed PPT point for boost converters [mV] */
} __attribute__((packed)) eps_system_config_t;

/**
 *  Battery mode configuration (Config2)
 */
typedef struct
{
    uint16_t batt_maxvoltage;               /**< Voltage threshold to be in FULL mode */
    uint16_t batt_safevoltage;              /**< Voltage threshold to trigger NORMAL -> SAFE mode */
    uint16_t batt_criticalvoltage;          /**< Lowest allowable voltage (below -> CRITICAL mode) */
    uint16_t batt_normalvoltage;            /**< Voltage threshold to trigger SAFE -> NORMAL mode */
    uint32_t reserved1[2];                  /**< Reserved */
    uint8_t  reserved2[4];                  /**< Reserved */
} __attribute__((packed)) eps_battery_config_t;

/**
 * P31u-8 housekeeping
 * 
 * NOTE that some changes have been made from the GomSpace datasheet
 * due to the complier mis-matches the bytes in the data structure 
 * 1) Changed counter_wdt_gnd and counter_boot from uint32_t to uint16_t
 * 2) The __attribute__((packed)) attribute is removed 
 * 
 * The data sizes at other parts of this API are changed accordingly 
 * 
 */
typedef struct
{
    uint16_t vboost[3];                     /**< Voltage of input voltage boost converters [mV]*/
    uint16_t vbatt;                         /**< Voltage of battery [mV] */
    uint16_t curin[3];                      /**< Input currents [mA] */
    uint16_t cursun;                        /**< Current from boost converters [mA] */
    uint16_t cursys;                        /**< Current out of battery [mA] */
    uint16_t reserved1;                     /**< Reserved for future use */
    uint16_t curout[6];                     /**< Output currents [mA] */
    uint8_t  output[8];                     /**< Output statuses [0 = Off, 1 = On] */
    uint16_t output_on_delta[8];            /**< Time until output power on [seconds] */
    uint16_t output_off_delta[8];           /**< Time until output power off [seconds] */
    uint16_t latchup[6];                    /**< Number of output latch-up events */
    uint32_t wdt_i2c_time_left;             /**< Time left for I2C watchdog [seconds] */
    uint32_t wdt_gnd_time_left;             /**< Time left for dedicated watchdog [seconds] */
    uint8_t  wdt_csp_pings_left[2];         /**< Pings left for CSP watchdog */
    uint32_t counter_wdt_i2c;               /**< Number of I2C watchdog reboots */
    uint16_t counter_wdt_gnd;               /**< Number of dedicated watchdog reboots */
    uint32_t counter_wdt_csp[2];            /**< Number of CSP watchdog reboots */
    uint16_t counter_boot;                  /**< Number of EPS reboots */
    int16_t  temp[6];                       /**< Temperatures [degC] [0 = Temp1, Temp2, Temp3, Temp4, BP4a, BP4b] */
    uint8_t  boot_cause;                    /**< Cause of last EPS reset */
    uint8_t  batt_mode;                     /**< Mode for battery [0 = Initial, 1 = Critical, 2 = Safe, 3 = Normal, 4 = Full] */
    uint8_t  ppt_mode;                      /**< Mode of power-point tracker [1 = Automatic maximum, 2 = Fixed] */
    uint16_t reserved2;                     /**< Reserved */
} eps_hk_t;

/*
 * Public Functions
 */
/**
 * Initialize the interface with the EPS
 * @param [in] config Interface configuration values
 * @return KEPSStatus EPS_OK if OK, error otherwise
 */
KEPSStatus k_eps_init(KEPSConf config);
/**
 * Terminate the EPS interface
 */
void k_eps_terminate(void);
/**
 * Ping the EPS
 * @return KEPSStatus EPS_OK if OK, error otherwise
 */
KEPSStatus k_eps_ping(void);
/**
 * Hard reset the EPS
 * @note It will wait 400ms before re-powering
 * @return KEPSStatus EPS_OK if OK, error otherwise
 */
KEPSStatus k_eps_reset(void);
/**
 * Reboot the EPS (soft reset)
 * @note Output power will not be affected
 * @return KEPSStatus EPS_OK if OK, error otherwise
 */
KEPSStatus k_eps_reboot(void);
/**
 * Configure the NanoPower's system
 * @param [in] config Pointer to system configuration values
 * @return KEPSStatus EPS_OK if OK, error otherwise
 */
KEPSStatus k_eps_configure_system(const eps_system_config_t * config);
/**
 * Configure the NanoPower's battery
 * @note Configuration must be saved using ::k_eps_save_battery_config between 1 and 30 seconds after calling
 * this function, otherwise the changes will be erased.
 * @param [in] config Pointer to battery configuration values
 * @return KEPSStatus EPS_OK if OK, error otherwise
 */
KEPSStatus k_eps_configure_battery(const eps_battery_config_t * config);
/**
 * Save the current battery configuration to EEPROM
 * @return KEPSStatus EPS_OK if OK, error otherwise
 */
KEPSStatus k_eps_save_battery_config(void);
/**
 * Turn on/off the NanoPower outputs
 * @note Cannot be used to control heaters
 * @param [in] channel_mask Bitmask for output channels (_MSB [X X 3.3V 3.3V 3.3V 5V 5V 5V] LSB_). 0 = Off, 1 = On
 * @return KEPSStatus EPS_OK if OK, error otherwise
 */
KEPSStatus k_eps_set_output(uint8_t channel_mask);
/**
 * Turn on/off a single output
 * @param [in] channel Output to control.
 *              0-5 = Output channels,
 *              6 = BP4 heater,
 *              7 = BP4 switch
 * @param [in] value Desired output value. 0 = Off, 1 = On
 * @param [in] delay Amount of time, in seconds, to wait before changing the output's value
 * @return KEPSStatus EPS_OK if OK, error otherwise
 */
KEPSStatus k_eps_set_single_output(uint8_t channel, uint8_t value,
                                   int16_t delay);
/**
 * Set photovoltaic input voltages
 * @note Only used when ::eps_hk_t.ppt_mode is `Fixed`
 * @param [in] in1_voltage Voltage value for PV1 [mV]
 * @param [in] in2_voltage Voltage value for PV2 [mV]
 * @param [in] in3_voltage Voltage value for PV3 [mV]
 * @return KEPSStatus EPS_OK if OK, error otherwise
 */
KEPSStatus k_eps_set_input_value(uint16_t in1_voltage, uint16_t in2_voltage,
                                 uint16_t in3_voltage);
/**
 * Set photovoltaic inputs' power-point tracking mode
 * @param [in] mode PPT mode.
 *              0 = hardware default power points,
 *              1 = maximum power-point tracking,
 *              2 = software-defined power points (set with ::k_eps_set_input_value)
 * @return KEPSStatus EPS_OK if OK, error otherwise
 */
KEPSStatus k_eps_set_input_mode(uint8_t mode);
/**
 * Control heaters
 * @param [in] cmd Heater control command (should always be `0` for now)
 * @param [in] heater Heater to control
 * @param [in] mode State to set heater to. 0 = Off, 1 = On
 * @return KEPSStatus EPS_OK if OK, error otherwise
 */
KEPSStatus k_eps_set_heater(uint8_t cmd, uint8_t heater, uint8_t mode);
/**
 * Reset system configuration to default values
 * @return KEPSStatus EPS_OK if OK, error otherwise
 */
KEPSStatus k_eps_reset_system_config(void);
/**
 * Reset battery configuration to default values
 * @note Values must be saved using ::k_eps_save_battery_config between 1 and 30 seconds after calling
 * this function, otherwise the changes will be erased
 * @return KEPSStatus EPS_OK if OK, error otherwise
 */
KEPSStatus k_eps_reset_battery_config(void);
/**
 * Reset system counters (boot count, watchdog reboot counts, etc)
 * @return KEPSStatus EPS_OK if OK, error otherwise
 */
KEPSStatus k_eps_reset_counters(void);
/**
 * Get system housekeeping data
 * @param [out] buff Pointer to storage structure
 * @return KEPSStatus EPS_OK if OK, error otherwise
 */
KEPSStatus k_eps_get_housekeeping(eps_hk_t * buff);
/**
 * Get system configuration values
 * @param [out] buff Pointer to storage structure
 * @return KEPSStatus EPS_OK if OK, error otherwise
 */
KEPSStatus k_eps_get_system_config(eps_system_config_t * buff);
/**
 * Get battery configuration values
 * @param [out] buff Pointer to storage structure
 * @return KEPSStatus EPS_OK if OK, error otherwise
 */
KEPSStatus k_eps_get_battery_config(eps_battery_config_t * buff);
/**
 * Get heaters' statuses
 * @param [out] bp4 Status of BP4 heater. 0 = Off, 1 = On
 * @param [out] onboard Status of onboard heater.  0 = Off, 1 = On
 * @return KEPSStatus EPS_OK if OK, error otherwise
 */
KEPSStatus k_eps_get_heater(uint8_t * bp4, uint8_t * onboard);
/**
 * Kick the EPS's watchdog once
 * @return KEPSStatus `EPS_OK` if OK, error otherwise
 */
KEPSStatus k_eps_watchdog_kick(void);
/**
 * Start a thread to kick the EPS's watchdog
 * @note The watchdog kick requires a write to EEPROM, which has a limited lifespan.
 * It is recommended that the watchdog interval be very large (ex. 48 **hours**)
 * @param [in] interval Time to sleep in between kicks [seconds]
 * @return KEPSStatus `EPS_OK` if OK, error otherwise
 */
KEPSStatus k_eps_watchdog_start(uint32_t interval);
/**
 * Stop the watchdog thread
 * @return KEPSStatus `EPS_OK` if OK, error otherwise
 */
KEPSStatus k_eps_watchdog_stop(void);
/**
 * Pass a command packet directly through to the EPS.
 * Useful for executing commands which have not been implemented in this API
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
/**
 * Write command to EPS and read back a response
 * @param [in]  tx      Pointer to command packet to send
 * @param [in]  tx_len  Size of command packet
 * @param [out] rx      Pointer to storage for command response
 * @param [in]  rx_len  Expected length of command response
 * @return KEPSStatus EPS_OK if OK, error otherwise
 */
KEPSStatus kprv_eps_transfer(const uint8_t * tx, int tx_len, uint8_t * rx,
                             int rx_len);

/* @} */
