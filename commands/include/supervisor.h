#pragma once

#include <stdint.h>

#define SUPERVISOR_NUMBER_OF_ADC_CHANNELS            10

#define LENGTH_GENERIC_REPLY                         3
#define LENGTH_TELEMETRY_HOUSEKEEPING                37
#define LENGTH_TELEMETRY_GET_VERSION                 34
#define LENGTH_POWER_CYCLE_IOBC                      3
#define LENGTH_COMPILE_INFORMATION                   19


/**
 * Generic reply from the Supervisor Controller.
 */
typedef union __attribute__ ((__packed__)) _supervisor_generic_reply_t
{
    uint8_t rawValue[LENGTH_GENERIC_REPLY]; //!< Raw value of the version configuration bytes..
    struct __attribute__ ((__packed__)) _fields_supervisor_generic_reply_t
    {
        uint8_t dummy; //!< The first as always is a dummy byte.
        uint8_t spiCommandStatus; //!< The second is a SPI Command Status.
        uint8_t crc8;  //!< CRC byte.
    } fields;
} supervisor_generic_reply_t;


/**
 * Enable status.
 */
typedef union __attribute__ ((__packed__)) _supervisor_enable_status_t
{
    uint8_t rawValue; //!< Raw value of the version configuration bytes.
    struct __attribute__ ((__packed__)) _fields_supervisor_enable_status_t
    {
        uint8_t powerObc : 1,   //!< /** OBC Power. */
                powerRtc : 1,           //!< Output power to the RTC.
                isInSupervisorMode : 1, //!< Supervisor mode.
                :1,
                :1,
                busyRtc : 1,            //!< RTC is busy.
                poweroffRtc : 1,        //!< RTC is doing power off.
                :1;
    } fields;
} supervisor_enable_status_t;


/**
 * Supervisor version and configuration bytes.
 */
typedef union __attribute__ ((__packed__)) _supervisor_version_configuration_t
{
    uint8_t rawValue[LENGTH_TELEMETRY_GET_VERSION]; //!< Raw value of the version configuration bytes..
    struct __attribute__ ((__packed__)) _fields_supervisor_version_configuration_t
    {
        uint8_t dummy;             //!< The first as always is a dummy byte.
        uint8_t spiCommandStatus;  //!< The second is a SPI Command Status.
        uint8_t indexOfSubsystem;  //!< Index of ISIS Subsystem.
        uint8_t majorVersion;      //!< Software major version.
        uint8_t minorVersion;      //!< Software minor version.
        uint8_t patchVersion;      //!< Software patch version.
        uint32_t gitHeadVersion;   //!< Software git head version.
        uint16_t serialNumber;     //!< Serial number.
        int8_t compileInformation[LENGTH_COMPILE_INFORMATION]; //!< Compile information (time and date).
        uint8_t clockSpeed;        //!< Clock speed of the Supervisor Controller (in MHz).
        int8_t codeType;           //!< Code type. Whether flight or test.
        uint8_t crc8;              //!< CRC byte.
    } fields;
} supervisor_version_configuration_t;


/**
 * supervisor housekeeping.
 */
typedef union __attribute__ ((__packed__)) _supervisor_housekeeping_t
{
    uint8_t rawvalue[SUPERVISOR_NUMBER_OF_ADC_CHANNELS];  //!< raw value of the version configuration bytes..
    struct __attribute__ ((__packed__)) _fields_supervisor_housekeeping_t
    {
        uint8_t dummy;                          //!< the first as always is a dummy byte.
        uint8_t spicommandstatus;               //!< the second is a spi command status.
        supervisor_enable_status_t enablestatus;    //!< enable status of the supervisor controller.
        uint32_t supervisoruptime;              //!< supervisor controller uptime.
        uint32_t iobcuptime;                    //!< iobc uptime as measured by supervisor controller.
        uint32_t iobcresetcount;                //!< iobc reset count.
        uint16_t adcdata[SUPERVISOR_NUMBER_OF_ADC_CHANNELS];  //!< adc data.
        uint8_t adcupdateflag;                  //!< adc update flag.
        uint8_t crc8;                           //!< crc byte.
    } fields;
} supervisor_housekeeping_t;


/*Initialize the supervisor*/
int supervisor_init();

int supervisor_get_version(supervisor_version_configuration_t* versionReply);

int supervisor_power_cycle_iobc(supervisor_generic_reply_t* reply);

