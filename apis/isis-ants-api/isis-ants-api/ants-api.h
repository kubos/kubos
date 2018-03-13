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
 * @defgroup ISIS_ANTS_API ISIS ISIS ANTS API
 * @addtogroup ISIS_ANTS_API
 * @{
 */

#pragma once

#include <stdbool.h>
#include <stdint.h>
#include <kubos-hal/i2c.h>

/**
 *  @name ISIS AntS config.json configuration options and default values
 */
/**@{*/
/**
 * I2C bus the ISIS AntS is connected to
 */
#define ANTS_I2C_BUS K_I2C1

/**
 * Primary antenna controller's I2C address
 */
#define ANTS_PRIMARY 0x31

/**
 * Secondary (redundant) antenna controller's I2C address
 */
#define ANTS_SECONDARY 0x00

/**
 * Number of deployable antennas
 */
#define ANT_COUNT 4

/**
 * Watchdog timeout (in seconds)
 */
#define ANTS_WD_TIMEOUT 60
/**@}*/

/** \cond WE DO NOT WANT TO HAVE THESE IN OUR GENERATED DOCS */
/* AntS command values */
#define SYSTEM_RESET                0xAA
#define WATCHDOG_RESET              0xCC
#define ARM_ANTS                    0xAD
#define DISARM_ANTS                 0xAC
#define DEPLOY_1                    0xA1
#define DEPLOY_2                    0xA2
#define DEPLOY_3                    0xA3
#define DEPLOY_4                    0xA4
#define AUTO_DEPLOY                 0xA5
#define DEPLOY_1_OVERRIDE           0xBA
#define DEPLOY_2_OVERRIDE           0xBB
#define DEPLOY_3_OVERRIDE           0xBC
#define DEPLOY_4_OVERRIDE           0xBD
#define CANCEL_DEPLOY               0xA9
#define GET_TEMP                    0xC0
#define GET_STATUS                  0xC3
#define GET_UPTIME_SYS              0xC6
#define GET_TELEMETRY               0xC7
#define GET_COUNT_1                 0xB0
#define GET_COUNT_2                 0xB1
#define GET_COUNT_3                 0xB2
#define GET_COUNT_4                 0xB3
#define GET_UPTIME_1                0xB4
#define GET_UPTIME_2                0xB5
#define GET_UPTIME_3                0xB6
#define GET_UPTIME_4                0xB7

/** \endcond */

/**
 *  @name Deployment Status Flags
 */
/**@{*/
#define SYS_BURN_ACTIVE             (1 << 4)    /**< Antenna system independent burn is active */
#define SYS_IGNORE_DEPLOY           (1 << 8)    /**< Antenna system is ignoring the deployment switches */
#define SYS_ARMED                   (1 << 0)    /**< Antenna system is armed */

#define ANT_1_NOT_DEPLOYED          (8 << 12)   /**< Antenna 1 is not deployed */
#define ANT_1_STOPPED_TIME          (4 << 12)   /**< Antenna 1 deployment time limit was reached */
#define ANT_1_ACTIVE                (2 << 12)   /**< Antenna 1 deployment system is active */

#if ANT_COUNT > 1
#define ANT_2_NOT_DEPLOYED          (8 << 8)    /**< Antenna 2 is not deployed */
#define ANT_2_STOPPED_TIME          (4 << 8)    /**< Antenna 2 deployment time limit was reached */
#define ANT_2_ACTIVE                (2 << 8)    /**< Antenna 2 deployment system is active */
#endif

#if ANT_COUNT > 2
#define ANT_3_NOT_DEPLOYED          (8 << 4)    /**< Antenna 3 is not deployed */
#define ANT_3_STOPPED_TIME          (4 << 4)    /**< Antenna 3 deployment time limit was reached */
#define ANT_3_ACTIVE                (2 << 4)    /**< Antenna 3 deployment system is active */
#endif

#if ANT_COUNT > 3
#define ANT_4_NOT_DEPLOYED          (8 << 0)    /**< Antenna 4 is not deployed */
#define ANT_4_STOPPED_TIME          (4 << 0)    /**< Antenna 4 deployment time limit was reached */
#define ANT_4_ACTIVE                (2 << 0)    /**< Antenna 4 deployment system is active */
#endif
/**@}*/

/**
 * Antenna function return values
 */
typedef enum {
    ANTS_OK,                     /**< Requested function completed successfully */
    ANTS_ERROR,                  /**< Generic error */
    ANTS_ERROR_CONFIG,           /**< Configuration error */
    ANTS_ERROR_NOT_IMPLEMENTED   /**< Requested function has not been implemented for the subsystem */
} KANTSStatus;

/**
 * Antenna microcontrollers
 */
typedef enum {
    PRIMARY,            /**< Issue commands using the primary microcontroller */
    SECONDARY,          /**< Issue commands using the secondary microcontroller (if available) */
} KANTSController;

/**
 * System Antennas
 */
typedef enum {
    ANT_1,              /**< Antenna 1 */
#if ANT_COUNT > 1
    ANT_2,              /**< Antenna 2 */
#endif
#if ANT_COUNT > 2
    ANT_3,              /**< Antenna 3 */
#endif
#if ANT_COUNT > 3
    ANT_4               /**< Antenna 4 */
#endif
} KANTSAnt;

/**
 * System telemetry fields returned from ::k_ants_get_system_telemetry
 */
typedef struct
{
    uint16_t raw_temp;      /**< Current temperature (raw value) */
    uint16_t deploy_status; /**< Current deployment status flags */
    uint32_t uptime;        /**< System uptime (in seconds) */
} __attribute__((packed)) ants_telemetry;

/*
 * Public Functions
 */
/**
 * Initialize the antenna interface
 * @return KANTSStatus ANTS_OK if OK, error otherwise
 */
KANTSStatus k_ants_init(KI2CNum bus, uint8_t primary, uint8_t secondary, uint8_t ant_count, uint32_t timeout);
/**
 * Terminate the antenna interface
 */
void k_ants_terminate(void);
/**
 * Configure the antenna
 * @param [in] config Microntroller to use for system commanding
 * @return KANTSStatus ANTS_OK if OK, error otherwise
 */
KANTSStatus k_ants_configure(KANTSController config);
/**
 * Reset both of the antenna's microcontrollers
 * @return KANTSStatus ANTS_OK if OK, error otherwise
 */
KANTSStatus k_ants_reset(void);
/**
 * Arm the antenna
 * @return KANTSStatus ANTS_OK if OK, error otherwise
 */
KANTSStatus k_ants_arm(void);
/**
 * Disarm the antenna
 * @return KANTSStatus ANTS_OK if OK, error otherwise
 */
KANTSStatus k_ants_disarm(void);
/**
 * Deploy an antenna
 * @param [in] antenna 	Antenna to deploy
 * @param [in] override Indicates whether system should ignore previous
 * 			   			successful deployment
 * @param [in] timeout 	Maximum time, in seconds, system should spend deploying
 * 						the antenna
 * @return KANTSStatus ANTS_OK if OK, error otherwise
 */
KANTSStatus k_ants_deploy(KANTSAnt antenna, bool override, uint8_t timeout);
/**
 * Automatically deploy each antenna in sequence
 * @param [in] timeout  Maximum time, in seconds, system should spend deploying
 * 						a single antenna
 * @return KANTSStatus ANTS_OK if OK, error otherwise
 */
KANTSStatus k_ants_auto_deploy(uint8_t timeout);
/**
 * Cancel all current deployment actions
 * @return KANTSStatus `ANTS_OK` if OK, error otherwise
 */
KANTSStatus k_ants_cancel_deploy(void);
/**
 * Get current deployment status
 * @param [out] resp Pointer to storage for data
 * @return KANTSStatus `ANTS_OK` if OK, error otherwise
 */
KANTSStatus k_ants_get_deploy_status(uint16_t * resp);
/**
 * Get system uptime
 * @param [out] uptime Pointer to storage for data
 * @return KANTSStatus `ANTS_OK` if OK, error otherwise
 */
KANTSStatus k_ants_get_uptime(uint32_t * uptime);
/**
 * Get the current system telemetry
 * @param [out] telem Pointer to ::ants_telemetry structure
 * @return KANTSStatus `ANTS_OK` if OK, error otherwise
 */
KANTSStatus k_ants_get_system_telemetry(ants_telemetry * telem);
/**
 * Get an antenna's activation count
 * @param [in]  antenna Antenna to query
 * @param [out] count   Number of times antenna deployment has been attempted
 * @return KANTSStatus `ANTS_OK` if OK, error otherwise
 */
KANTSStatus k_ants_get_activation_count(KANTSAnt antenna, uint8_t * count);
/**
 * Get an antenna's activation time
 * @param [in]  antenna Antenna to query
 * @param [out] time    Amount of time spent deploying antenna in 50ms steps
 * @return KANTSStatus `ANTS_OK` if OK, error otherwise
 */
KANTSStatus k_ants_get_activation_time(KANTSAnt antenna, uint16_t * time);
/**
 * Kick the AntS's watchdogs once
 * @return KANTSStatus `ANTS_OK` if OK, error otherwise
 */
KANTSStatus k_ants_watchdog_kick(void);
/**
 * Start a thread to kick the AntS's watchdogs at an interval of
 * (::ANTS_WD_TIMEOUT/3) seconds
 * @return KANTSStatus `ANTS_OK` if OK, error otherwise
 */
KANTSStatus k_ants_watchdog_start(void);
/**
 * Stop the watchdog thread
 * @return KANTSStatus `ANTS_OK` if OK, error otherwise
 */
KANTSStatus k_ants_watchdog_stop(void);
/**
 * Pass a command packet directly through to the antenna.
 * Useful for executing commands which have not been implemented in either the
 * generic or specific antenna APIs.
 * @param [in]  tx      Pointer to command packet to send
 * @param [in]  tx_len  Size of command packet
 * @param [out] rx      Pointer to storage for command response
 * @param [in]  rx_len  Expected length of command response
 * @return KANTSStatus ANTS_OK if OK, error otherwise
 */
KANTSStatus k_ants_passthrough(const uint8_t * tx, int tx_len, uint8_t * rx,
                               int rx_len);

/* @} */
