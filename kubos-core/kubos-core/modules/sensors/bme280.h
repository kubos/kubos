/***************************************************************************
  This is a library for the BME280 humidity, temperature & pressure sensor

  Designed specifically to work with the Adafruit BME280 Breakout
  ----> http://www.adafruit.com/products/2650

  These sensors use I2C or SPI to communicate, 2 or 4 pins are required
  to interface.

  Adafruit invests time and resources providing this open source code,
  please support Adafruit andopen-source hardware by purchasing products
  from Adafruit!

  Written by Limor Fried & Kevin Townsend for Adafruit Industries.
  BSD license, all text above must be included in any redistribution

  Modified by KubOS Corporation 2016 for integration into Kubos Core
 ***************************************************************************/
/**
 * @defgroup KUBOS_CORE_BME280 Kubos Core BME280 Sensor Interface
 * @addtogroup KUBOS_CORE_BME280
 * @{
 */

/*
 * Enabling this sensor code requires certain configuration values to be present
 * in the application's configuration json. An example is given below:
 *
 *  {
 *      "sensors": {
 *          "BME280": {
 *              "spi_bus": "K_SPI1",
 *              "CS":"PA4"
 *          }
 *      }
 *  }
 *
 * This would enable the BME280 sensor code and configure it for the SPI bus
 * K_SPI1 with chip select PA4.
 */

#ifdef YOTTA_CFG_SENSORS_BME280

#ifndef BME280_H
#define BME280_H

#include <stdint.h>
#include "kubos-hal/spi.h"
#include "kubos-core/modules/sensors/sensors.h"

/**
 * Register map for BME280 sensor
 */
typedef enum
{
    /** @cond */
    BME280_REGISTER_DIG_T1              = 0x88,
    BME280_REGISTER_DIG_T2              = 0x8A,
    BME280_REGISTER_DIG_T3              = 0x8C,

    BME280_REGISTER_DIG_P1              = 0x8E,
    BME280_REGISTER_DIG_P2              = 0x90,
    BME280_REGISTER_DIG_P3              = 0x92,
    BME280_REGISTER_DIG_P4              = 0x94,
    BME280_REGISTER_DIG_P5              = 0x96,
    BME280_REGISTER_DIG_P6              = 0x98,
    BME280_REGISTER_DIG_P7              = 0x9A,
    BME280_REGISTER_DIG_P8              = 0x9C,
    BME280_REGISTER_DIG_P9              = 0x9E,

    BME280_REGISTER_DIG_H1              = 0xA1,
    BME280_REGISTER_DIG_H2              = 0xE1,
    BME280_REGISTER_DIG_H3              = 0xE3,
    BME280_REGISTER_DIG_H4              = 0xE4,
    BME280_REGISTER_DIG_H5              = 0xE5,
    BME280_REGISTER_DIG_H6              = 0xE7,

    BME280_REGISTER_CHIPID             = 0xD0,
    BME280_REGISTER_VERSION            = 0xD1,
    BME280_REGISTER_SOFTRESET          = 0xE0,

    /* calibration stored in 0xE1-0xF0 */
    BME280_REGISTER_CAL26              = 0xE1,

    BME280_REGISTER_CONTROLHUMID       = 0xF2,
    BME280_REGISTER_CONTROL            = 0xF4,
    BME280_REGISTER_CONFIG             = 0xF5,
    BME280_REGISTER_PRESSUREDATA       = 0xF7,
    BME280_REGISTER_TEMPDATA           = 0xFA,
    BME280_REGISTER_HUMIDDATA          = 0xFD,
} bme280_register;

/**
  * @brief Calibration data structure to hold coefficients
  *
  * This structure is used internally when reading in data.
 */
typedef struct
{
    uint16_t dig_T1;
    int16_t  dig_T2;
    int16_t  dig_T3;

    uint16_t dig_P1;
    int16_t  dig_P2;
    int16_t  dig_P3;
    int16_t  dig_P4;
    int16_t  dig_P5;
    int16_t  dig_P6;
    int16_t  dig_P7;
    int16_t  dig_P8;
    int16_t  dig_P9;

    uint8_t  dig_H1;
    int16_t  dig_H2;
    uint8_t  dig_H3;
    int16_t  dig_H4;
    int16_t  dig_H5;
    int8_t   dig_H6;
} bme280_calib_data;

/**
 * Setup the SPI interface for talking with the BME280 and init sensor
 * @return KSensorStatus SENSOR_OK on success, SENSOR_WRITE_ERROR or
 * SENSOR_NOT_FOUND on error
 */
KSensorStatus bme280_setup(void);

/**
 * Reset the BME280 to default conditions
 * @return KSensorStatus SENSOR_OK on success, SENSOR_WRITE_ERROR on error
 */
KSensorStatus bme280_soft_reset(void);

/**
 * Sends temperature command and reads back temperature data
 * @param temp Pointer to temperature data in celsius (-40.0 to 85.0)
 * @return KSensorStatus SENSOR_OK on success, SENSOR_ERROR or
 * SENSOR_READ_ERROR on error
 */
KSensorStatus bme280_read_temperature(float * temp);

/**
 * Sends pressure command and reads back pressure data
 * @param press Pointer to pressure data in Pa (101325.0 - 0.0)
 * @return KSensorStatus SENSOR_OK on success, SENSOR_ERROR or
 * SENSOR_READ_ERROR on error
 */
KSensorStatus bme280_read_pressure(float * press);

/**
 * Sends humidity command and reads back humidity
 * @param hum Pointer to humidity in percentage (0.0 - 100.0)
 * @return KSensorStatus SENSOR_OK on success,SENSOR_ERROR or
 * SENSOR_READ_ERROR on error
 */
KSensorStatus bme280_read_humidity(float * hum);

/**
 * converts pressure to absolute altitude
 * @param sea_level in hPa (1013.25 recommended)
 * @param alt Pointer to altitude in meters (m)
 * @return KSensorStatus SENSOR_OK on success, SENSOR_ERROR or
 * SENSOR_READ_ERROR on error
 */
KSensorStatus bme280_read_altitude(float sea_level, float * alt);


#endif
#endif

/* @} */
