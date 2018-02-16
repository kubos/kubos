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
 * @defgroup TRXVU_RADIO_API ISIS TRXVU Radio API
 * @addtogroup TRXVU_RADIO_API
 * @{
 */

#pragma once

#include <math.h>

/**
 *  @name Radio config.json configuration options and default values
 */
/**@{*/
/**
 * I2C bus the TRXVU radio is connected to
 */
#ifdef YOTTA_CFG_RADIO_TRXVU_I2C_BUS
#define TRXVU_I2C_BUS YOTTA_CFG_RADIO_TRXVU_I2C_BUS
#else
#define TRXVU_I2C_BUS K_I2C1
#endif

/**
 * Transmitter I2C address
 */
#ifdef YOTTA_CFG_RADIO_TRXVU_TX_ADDR
#define RADIO_TX_ADDR YOTTA_CFG_RADIO_TRXVU_TX_ADDR
#else
#define RADIO_TX_ADDR 0x61
#endif

/**
 * Receiver I2C address
 */
#ifdef YOTTA_CFG_RADIO_TRXVU_RX_ADDR
#define RADIO_RX_ADDR YOTTA_CFG_RADIO_TRXVU_RX_ADDR
#else
#define RADIO_RX_ADDR 0x60
#endif

/**
 * Transmitter maximum message size
 */
#ifdef YOTTA_CFG_RADIO_TRXVU_TX_MAX_PAYLOAD
#define TX_MAX_SIZE YOTTA_CFG_RADIO_TRXVU_TX_MAX_PAYLOAD
#else
#define TX_MAX_SIZE 235
#endif

/**
 * Receiver maximum message size
 */
#ifdef YOTTA_CFG_RADIO_TRXVU_RX_MAX_PAYLOAD
#define RX_MAX_SIZE YOTTA_CFG_RADIO_TRXVU_RX_MAX_PAYLOAD
#else
#define RX_MAX_SIZE 200
#endif

/**
 * Transmitter buffer slots
 */
#ifdef YOTTA_CFG_RADIO_TRXVU_TX_MAX_FRAMES
#define TX_MAX_FRAMES YOTTA_CFG_RADIO_TRXVU_TX_MAX_FRAMES
#else
#define TX_MAX_FRAMES 40
#endif

/**
 * Receive buffer slots
 */
#ifdef YOTTA_CFG_RADIO_TRXVU_RX_MAX_FRAMES
#define RX_MAX_FRAMES YOTTA_CFG_RADIO_TRXVU_RX_MAX_FRAMES
#else
#define RX_MAX_FRAMES 40
#endif

/**
 * Watchdog timeout (in seconds)
 */
#ifdef YOTTA_CFG_RADIO_TRXVU_WATCHDOG_TIMEOUT
#define TRXVU_WD_TIMEOUT YOTTA_CFG_RADIO_TRXVU_WATCHDOG_TIMEOUT
#else
#define TRXVU_WD_TIMEOUT 60
#endif
/**@}*/

/** \cond WE DO NOT WANT TO HAVE THESE IN OUR GENERATED DOCS */
/* Radio command values */
/* Note: There are some duplicate command values between the TX and RX MCUs */
#define SEND_FRAME                  0x10
#define SEND_AX25_OVERRIDE          0x11
#define SET_BEACON                  0x14
#define SET_AX25_BEACON_OVERRIDE    0x15
#define GET_RX_ALL_TELEMETRY        0x1A
#define CLEAR_BEACON                0x1F
#define GET_RX_FRAME_COUNT          0x21
#define GET_RX_FRAME                0x22
#define SET_DEFAULT_AX25_TO         0x22
#define SET_DEFAULT_AX25_FROM       0x23
#define REMOVE_RX_FRAME             0x24
#define SET_IDLE_STATE              0x24
#define GET_TX_ALL_TELEMETRY        0x25
#define GET_LAST_TRANS_TELEM        0x26
#define SET_TX_RATE                 0x28
#define GET_UPTIME                  0x40
#define GET_TX_STATE                0x41
#define SOFT_RESET                  0xAA
#define HARD_RESET                  0xAB
#define WATCHDOG_RESET              0xCC
/** \endcond */

/**
 * Radio function return values
 */
typedef enum {
    /** Function call completed successfully */
    RADIO_OK = 0,
    /** Radio receive buffer is empty */
    RADIO_RX_EMPTY,
    /** Generic radio error */
    RADIO_ERROR,
    /** Function input parameter is invalid */
    RADIO_ERROR_CONFIG
} KRadioStatus;

/**
 * Radio reset types
 */
typedef enum {
    /** Perform hardware-level radio reset */
    RADIO_HARD_RESET,
    /** Perform software radio reset */
    RADIO_SOFT_RESET
} KRadioReset;

/**
 * Flags used to set radio transmission data rate
 */
typedef enum {
    RADIO_TX_RATE_1200        =  0x01,    /**< Transmitter data rate 1200bps */
    RADIO_TX_RATE_2400        =  0x02,    /**< Transmitter data rate 2400bps */
    RADIO_TX_RATE_4800        =  0x04,    /**< Transmitter data rate 4800bps */
    RADIO_TX_RATE_9600        =  0x08     /**< Transmitter data rate 9600bps */
} RadioTXRate;

/**
 * Flags used to set transmitter's idle state
 */
typedef enum {
    RADIO_IDLE_UNKNOWN = 0, /**< Dummy value to indicate no change should be made */
    RADIO_IDLE_OFF,         /**< Transmitter should turn off while idle */
    RADIO_IDLE_ON           /**< Transmitter should remain on while idle */
} RadioIdleState;

/**
 * Telemetry request types
 */
typedef enum {
    RADIO_TX_TELEM_ALL,     /**< Returns the current measurements of all the transmitter's telemetry channels */
    RADIO_TX_TELEM_LAST,    /**< Returns the telemetry channels that were sampled during the last frame transmission */
    RADIO_TX_UPTIME,        /**< Returns the amount of time, in seconds, that the transmitter portion of the radio has been active */
    RADIO_TX_STATE,         /**< Returns the current state of the transmitter */
    RADIO_RX_TELEM_ALL,     /**< Returns the current measurements of all the receiver's telemetry channels */
    RADIO_RX_UPTIME         /**< Returns the amount of time, in seconds, that the receiver portion of the radio has been active */
} RadioTelemType;

/**
 * Radio TX state flags returned by ::RADIO_TX_STATE request
 */
typedef enum {
    RADIO_STATE_IDLE_OFF      =  0x00,    /**< Transmitter will turn off when idle */
    RADIO_STATE_IDLE_ON       =  0x01,    /**< Transmitter will remain on when idle */
    RADIO_STATE_BEACON_ACTIVE =  0x02,    /**< Transmitter's beacon is enabled */
    RADIO_STATE_RATE_1200     =  0x00,    /**< Transmitter sending at 1200bps */
    RADIO_STATE_RATE_2400     =  0x01,    /**< Transmitter sending at 2400bps */
    RADIO_STATE_RATE_4800     =  0x02,    /**< Transmitter sending at 4800bps */
    RADIO_STATE_RATE_9600     =  0x03     /**< Transmitter sending at 9600bps */
} RadioTXState;

/**
 * Transmitter raw telemetry fields returned from ::RADIO_TX_TELEM_ALL and ::RADIO_TX_TELEM_LAST requests
 */
typedef struct
{
    uint16_t inst_RF_reflected;     /**< Instantaneous RF reflected power at transmitter port */
    uint16_t inst_RF_forward;       /**< Instantaneous RF forward power at transmitter port */
    uint16_t supply_voltage;        /**< Power bus voltage */
    uint16_t supply_current;        /**< Total supply current */
    uint16_t temp_power_amp;        /**< Power amplifier temperature */
    uint16_t temp_oscillator;       /**< Local oscillator temperature */
} trxvu_tx_telem_raw;

/**
 * AX.25 call-sign structure
 */
typedef struct
{
    /**
     * Six character station call-sign
     */
    uint8_t ascii[6];
    /**
     * One byte station SSID value
     */
    uint8_t ssid;
} ax25_callsign;

/**
 * Receiver raw telemetry fields returned from ::RADIO_RX_TELEM_ALL telemetry request
 */
typedef struct
{
    uint16_t inst_doppler_offset;   /**< Instantaneous Doppler offset of signal at receiver port */
    uint16_t supply_current;        /**< Total supply current */
    uint16_t supply_voltage;        /**< Power bus voltage */
    uint16_t temp_oscillator;       /**< Local oscillator temperature */
    uint16_t temp_power_amp;        /**< Power amplifier temperature */
    uint16_t inst_signal_strength;  /**< Instantaneous signal strength of the signal at the receiver */
} trxvu_rx_telem_raw;

/**
 * Transmitter or receiver uptime value (in seconds)
 */
typedef uint32_t trxvu_uptime;

/**
 * High-level Unifying Radio Telemetry Structure
 */
typedef union
{
    uint8_t             tx_state;   /**< Returned by ::RADIO_TX_STATE */
    trxvu_uptime        uptime;     /**< Returned by ::RADIO_TX_UPTIME and ::RADIO_RX_UPTIME */
    trxvu_tx_telem_raw  tx_telem;   /**< Returned by ::RADIO_TX_TELEM_ALL and ::RADIO_TX_TELEM_LAST */
    trxvu_rx_telem_raw  rx_telem;   /**< Returned by ::RADIO_RX_TELEM_ALL */
} radio_telem;

/**
 * Transmitter automatic periodic beacon configuration
 */
typedef struct
{
    uint16_t interval;  /**< Interval (in seconds) at which the beacon message should be sent */
    char *   msg;       /**< Pointer to beacon payload message to be sent */
    uint8_t  len;       /**< Length of beacon payload message */
} radio_tx_beacon;

/**
 * Radio transmitter configuration options
 */
typedef struct
{
    RadioTXRate     data_rate;      /**< Transmission data rate flag */
    RadioIdleState  idle;           /**< Transmitter's state when idle */
    radio_tx_beacon beacon;         /**< Transmitter beacon configuration */
    ax25_callsign   to;             /**< Transmitter AX.25 sender call-sign */
    ax25_callsign   from;           /**< Transmitter AX.25 destination call-sign */
} radio_config;

/**
 * Radio receive frame structure
 */
typedef struct
{
    uint16_t msg_size;              /**< Size of the frame payload */
    uint16_t doppler_offset;        /**< ADC value of doppler shift at receive time (convert with ::get_doppler_offset)*/
    uint16_t signal_strength;       /**< ADC value of signal strength at receive time (convert with ::get_signal_strength)*/
    uint8_t message[RX_MAX_SIZE];   /**< Frame payload */
} radio_rx_message;

/*
 * Public Functions
 */
/**
 * Kick the radio's watchdogs once
 * @return KRadioStatus `RADIO_OK` if OK, error otherwise
 */
KRadioStatus k_radio_watchdog_kick(void);

/**
 * Start a thread to kick the radio's watchdogs at an interval of (::TRXVU_WD_TIMEOUT/3) seconds
 * @return KRadioStatus `RADIO_OK` if OK, error otherwise
 */
KRadioStatus k_radio_watchdog_start(void);
/**
 * Stop the watchdog thread
 * @return KRadioStatus `RADIO_OK` if OK, error otherwise
 */
KRadioStatus k_radio_watchdog_stop(void);

/**
 * Send a message to the transmit buffer, but use the specified call-signs instead of the defaults
 * @param [in] to AX.25 call-sign for message sender
 * @param [in] from AX.25 call-sign for message destination
 * @param [in] buffer Pointer to message to send
 * @param [in] len Length of message to send
 * @param [out] response Pointer to storage area for response byte
 * @return KRadioStatus `RADIO_OK` on success, otherwise error
 */
KRadioStatus k_radio_send_override(ax25_callsign to, ax25_callsign from, char * buffer, int len, uint8_t * response);

/**
 * Set the automatic periodic beacon, but use the specified call-signs instead of the defaults
 * @param [in] to AX.25 call-sign for message sender
 * @param [in] from AX.25 call-sign for message destination
 * @param [in] beacon ::radio_tx_beacon to send
 * @return KRadioStatus `RADIO_OK` on success, otherwise error
 */
KRadioStatus k_radio_set_beacon_override(ax25_callsign to, ax25_callsign from, radio_tx_beacon beacon);

/**
 * Clear/deactivate the automatic periodic beacon
 * @return KRadioStatus `RADIO_OK` on success, otherwise error
 */
KRadioStatus k_radio_clear_beacon(void);

/**
 *  @name Telemetry Conversion Functions
 *  Convert raw ADC values into human-readable units
 */
/**@{*/
/**
 * @param [in] raw Raw ADC value
 * @return Voltage in volts
 */
inline float get_voltage(uint16_t raw) {return raw * 0.00488;}
/**
 * @param [in] raw Raw ADC value
 * @return Current in milliamps
 */
inline float get_current(uint16_t raw) {return raw * 0.16643964;}
/**
 * @param [in] raw Raw ADC value
 * @return Temperature in degrees Celsius
 */
inline float get_temperature(uint16_t raw) {return raw * -0.07669 + 195.6037;}
/**
 * @param [in] raw Raw ADC value
 * @return Doppler shift in hertz
 */
inline float get_doppler_offset(uint16_t raw) {return raw * 13.352 - 22300;}
/**
 * @param [in] raw Raw ADC value
 * @return Received signal strength power in decibel-milliwatts
 */
inline float get_signal_strength(uint16_t raw) {return raw * 0.03 - 152;}
/**
 * @param [in] raw Raw ADC value
 * @return RF reflected power in decibel-milliwatts
 */
inline float get_rf_power_dbm(uint16_t raw) {return 20 * log10(raw * 0.00767);}
/**
 * @param [in] raw Raw ADC value
 * @return RF reflected power in milliwatts
 */
inline float get_rf_power_mw(uint16_t raw) {return raw * raw * powf(10, -2) * 0.00005887;}
/**@}*/

/*
 * Public Functions
 */

/**
 * Initialize the radio interface
 * @return KRadioStatus RADIO_OK if OK, error otherwise
 */
KRadioStatus k_radio_init(void);
/**
 * Terminate the radio interface
 */
void k_radio_terminate(void);
/**
 * Configure the radio
 * @note This function might not be implemented for all radios. See specific radio API documentation for configuration availability and structure
 * @param [in] config Pointer to the radio configuration structure
 * @return KRadioStatus RADIO_OK if OK, error otherwise
 */
KRadioStatus k_radio_configure(radio_config * config);
/**
 * Reset the radio
 * @note This function might not be implemented for all radios
 * @param [in] type Type of reset to perform (hard, soft, etc)
 * @return KRadioStatus RADIO_OK if OK, error otherwise
 */
KRadioStatus k_radio_reset(KRadioReset type);
/**
 * Send a message to the radio's transmit buffer
 * @param [in] buffer Pointer to the message to send
 * @param [in] len Length of the message to send
 * @param [out] response Response value from radio (if supported)
 * @return KRadioStatus RADIO_OK if OK, error otherwise
 */
KRadioStatus k_radio_send(char * buffer, int len, uint8_t * response);
/**
 * Receive a message from the radio's receive buffer
 * @param [in] buffer Pointer where the message should be copied to
 * @param [out] len Length of the received message
 * @return KRadioStatus RADIO_OK if a message was received successfully, RADIO_RX_EMPTY if there are no messages to receive, error otherwise
 */
KRadioStatus k_radio_recv(radio_rx_message * buffer, uint8_t * len);
/**
 * Read radio telemetry values
 * @note See specific radio API documentation for available telemetry types
 * @param [in] buffer Pointer to structure which data should be copied to
 * @param [in] type Telemetry packet to read
 * @return KRadioStatus RADIO_OK if OK, error otherwise
 */
KRadioStatus k_radio_get_telemetry(radio_telem * buffer, RadioTelemType type);

/*
 * Internal Functions
 */

/**
 * Thread which kicks the radio's watchdogs every (::TRXVU_WD_TIMEOUT/3) seconds
 */
void * kprv_radio_watchdog_thread(void * args);

/**
 * Set the transmitter beacon's interval and message
 *
 * @param rate Interval (in seconds) at which to send beacon message
 * @param buffer Pointer to beacon payload message
 * @param len Length of beacon payload message
 * @return KRadioStatus `RADIO_OK` on success, otherwise error
 */
KRadioStatus kprv_radio_tx_set_beacon(uint16_t rate, char * buffer, int len);
/**
 * Set the transmitter's default AX.25 sender call-sign
 *
 * @param to Pointer to AX.25 call-sign structure
 * @return KRadioStatus `RADIO_OK` on success, otherwise error
 */
KRadioStatus kprv_radio_tx_set_default_to(ax25_callsign to);
/**
 * Set the transmitter's default AX.25 destination call-sign
 *
 * @param from Pointer to AX.25 call-sign structure
 * @return KRadioStatus `RADIO_OK` on success, otherwise error
 */
KRadioStatus kprv_radio_tx_set_default_from(ax25_callsign from);
/**
 * Set the transmitter's idle state
 *
 * @param state Idle state. Should be ::RADIO_IDLE_ON or ::RADIO_IDLE_OFF
 * @return KRadioStatus `RADIO_OK` on success, otherwise error
 */
KRadioStatus kprv_radio_tx_set_idle(RadioIdleState state);
/**
 * Set the transmitter's data rate
 *
 * @param rate Data rate. Should be `RADIO_TX_RATE_1200`, `RADIO_TX_RATE_2400`, `RADIO_TX_RATE_4800`, or `RADIO_TX_RATE_9600`
 * @return KRadioStatus `RADIO_OK` on success, otherwise error
 */
KRadioStatus kprv_radio_tx_set_rate(RadioTXRate rate);
/**
 * Get telemetry from transmitter
 *
 * @param[out] buffer Pointer to storage area. Should point to a ::trxvu_tx_telem_raw structure if ::RADIO_TX_TELEM_ALL or ::RADIO_TX_TELEM_LAST is being requested
 * @param[in] type Telemetry type to fetch. Should be ::RADIO_TX_TELEM_ALL, ::RADIO_TX_TELEM_LAST, ::RADIO_TX_UPTIME, or ::RADIO_TX_STATE
 * @return KRadioStatus `RADIO_OK` on success, otherwise error
 */
KRadioStatus kprv_radio_tx_get_telemetry(radio_telem * buffer, RadioTelemType type);
/**
 * Kick the transmitter's watchdog
 * @return KRadioStatus `RADIO_OK` if OK, error otherwise
 */
KRadioStatus kprv_radio_tx_watchdog_kick(void);
/**
 * Reset the transmitter
 * @param type Type of reset to be performed. Should be `RADIO_RESET_SOFT` or `RADIO_RESET_HARD`
 * @return KRadioStatus `RADIO_OK` if OK, error otherwise
 */
KRadioStatus kprv_radio_tx_reset(KRadioReset type);
/**
 * Get number of frames in receive buffer
 * @param[out] count Pointer to storage area for count value
 * @return KRadioStatus `RADIO_OK` if OK, error otherwise
 */
KRadioStatus kprv_radio_rx_get_count(uint8_t * count);
/**
 * Delete oldest frame from receive buffer
 * @return KRadioStatus `RADIO_OK` if OK, error otherwise
 */
KRadioStatus kprv_radio_rx_remove_frame(void);
/**
 * Retrieve oldest frame from receive buffer
 * @param[out] buffer Pointer to storage area for frame
 * @param[out] len Pointer to storage are for length of frame payload
 * @return KRadioStatus `RADIO_OK` if OK, error otherwise
 */
KRadioStatus kprv_radio_rx_get_frame(radio_rx_message * buffer, uint8_t * len);
/**
 * Get telemetry from receiver
 *
 * @param[out] buffer Pointer to storage area. Should point to a ::trxvu_rx_telem_raw structure if ::RADIO_RX_TELEM_ALL is being requested
 * @param[in] type Telemetry type to fetch. Should be ::RADIO_RX_TELEM_ALL or ::RADIO_RX_UPTIME
 * @return KRadioStatus `RADIO_OK` on success, otherwise error
 */
KRadioStatus kprv_radio_rx_get_telemetry(radio_telem * buffer, RadioTelemType type);
/**
 * Kick the receiver's watchdog
 * @return KRadioStatus `RADIO_OK` if OK, error otherwise
 */
KRadioStatus kprv_radio_rx_watchdog_kick(void);
/**
 * Reset the receiver
 * @param type Type of reset to be performed. Should be `RADIO_RESET_SOFT` or `RADIO_RESET_HARD`
 * @return KRadioStatus `RADIO_OK` if OK, error otherwise
 */
KRadioStatus kprv_radio_rx_reset(KRadioReset type);

/* @} */
