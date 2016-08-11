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
  ADDED: soft reset function
 ***************************************************************************/

#ifdef YOTTA_CFG_SENSORS_BME280

#include "kubos-core/modules/sensors/bme280.h"
#include "kubos-hal/gpio.h"
#include "FreeRTOS.h"
#include "task.h"

#ifdef TARGET_LIKE_STM32
#include <math.h> /* include math.h for pow */
#endif

#ifndef SPI_BUS
#define SPI_BUS YOTTA_CFG_SENSORS_BME280_SPI_BUS
#endif

#ifndef CS
#define CS YOTTA_CFG_SENSORS_BME280_CS
//#define CS YOTTA_CFG_HARDWARE_PINS_SPI_CS
#endif

/* globals */
bme280_calib_data _bme280_calib;
int32_t t_fine = 0;

/* static functions */
static void bme280_read_coefficients(void);

static KSensorStatus write_byte(uint8_t reg, uint8_t value);
static KSensorStatus read_byte(uint8_t reg, uint8_t * value);
static KSensorStatus read_16_bit(uint8_t reg, uint16_t * value);
static KSensorStatus read_24_bit(uint8_t reg, uint32_t * value);
static KSensorStatus read_signed_16_bit(uint8_t reg, int16_t * value);
static KSensorStatus read_16_bit_LE(uint8_t reg, uint16_t * value);
static KSensorStatus read_signed_16_bit_LE(uint8_t reg, int16_t * value);

KSensorStatus bme280_setup(void)
{
    /* init chip select output, pull-up */
    k_gpio_init(CS, K_GPIO_OUTPUT, K_GPIO_PULL_UP);
    k_gpio_write(CS, 1); /* drive CS high */

    /* init SPI */
    KSPIConf conf = {
        .role = K_SPI_MASTER,
        .direction = K_SPI_DIRECTION_2LINES,
        .data_size = K_SPI_DATASIZE_8BIT,
        .speed = 10000
    };
    k_spi_init(SPI_BUS, &conf);

    /* soft reset and wait */
    if (bme280_soft_reset() != SENSOR_OK)
    {
        return SENSOR_WRITE_ERROR;
    }
    vTaskDelay(100);

    /* check if sensor is present */
    int timeout = 20;
    uint8_t chip_id = 0;
    read_byte(BME280_REGISTER_CHIPID, &chip_id);

    /* not working? */
    while (chip_id != 0x60)
    {
        vTaskDelay(50);

        /* try again */
        read_byte(BME280_REGISTER_CHIPID, &chip_id);

        /* timed out? */
        if (timeout <= 0)
        {
            return SENSOR_NOT_FOUND;
        }
        timeout--;
    }

    /* load coefficients */
    bme280_read_coefficients();

    /* (DS sec 5.4.3) */
    write_byte(BME280_REGISTER_CONTROLHUMID, 0x05); /* 16x oversampling */

    write_byte(BME280_REGISTER_CONTROL, 0xB7); /* 16x ovesampling, normal mode */

    return SENSOR_OK;
}

KSensorStatus bme280_soft_reset(void)
{
    /* send reset command and return status */
    return write_byte(BME280_REGISTER_SOFTRESET, 0xB6);
}

void bme280_read_coefficients(void)
{
    /* load all coefficients */
    _bme280_calib.dig_T1 = 0;
    read_16_bit_LE(BME280_REGISTER_DIG_T1, &_bme280_calib.dig_T1);
    _bme280_calib.dig_T2 = 0;
    read_signed_16_bit_LE(BME280_REGISTER_DIG_T2, &_bme280_calib.dig_T2);
    _bme280_calib.dig_T3 = 0;
    read_signed_16_bit_LE(BME280_REGISTER_DIG_T3, &_bme280_calib.dig_T3);

    _bme280_calib.dig_P1 = 0;
    read_16_bit_LE(BME280_REGISTER_DIG_P1, &_bme280_calib.dig_P1);
    _bme280_calib.dig_P2 = 0;
    read_signed_16_bit_LE(BME280_REGISTER_DIG_P2, &_bme280_calib.dig_P2);
    _bme280_calib.dig_P3 = 0;
    read_signed_16_bit_LE(BME280_REGISTER_DIG_P3, &_bme280_calib.dig_P3);
    _bme280_calib.dig_P4 = 0;
    read_signed_16_bit_LE(BME280_REGISTER_DIG_P4, &_bme280_calib.dig_P4);
    _bme280_calib.dig_P5 = 0;
    read_signed_16_bit_LE(BME280_REGISTER_DIG_P5, &_bme280_calib.dig_P5);
    _bme280_calib.dig_P6 = 0;
    read_signed_16_bit_LE(BME280_REGISTER_DIG_P6, &_bme280_calib.dig_P6);
    _bme280_calib.dig_P7 = 0;
    read_signed_16_bit_LE(BME280_REGISTER_DIG_P7, &_bme280_calib.dig_P7);
    _bme280_calib.dig_P8 = 0;
    read_signed_16_bit_LE(BME280_REGISTER_DIG_P8, &_bme280_calib.dig_P8);
    _bme280_calib.dig_P9 = 0;
    read_signed_16_bit_LE(BME280_REGISTER_DIG_P9, &_bme280_calib.dig_P9);

    _bme280_calib.dig_H1 = 0;
    read_byte(BME280_REGISTER_DIG_H1, &_bme280_calib.dig_H1);
    _bme280_calib.dig_H2 = 0;
    read_signed_16_bit_LE(BME280_REGISTER_DIG_H2, &_bme280_calib.dig_H2);
    _bme280_calib.dig_H3 = 0;
    read_byte(BME280_REGISTER_DIG_H3, &_bme280_calib.dig_H3);

    uint8_t dig_H4_0 = 0;
    uint8_t dig_H4_1 = 0;
    read_byte(BME280_REGISTER_DIG_H4, &dig_H4_0);
    read_byte(BME280_REGISTER_DIG_H4+1, &dig_H4_1);
    _bme280_calib.dig_H4 = (dig_H4_0 << 4) | (dig_H4_1 & 0xF);

    uint8_t dig_H5_0 = 0;
    uint8_t dig_H5_1 = 0;
    read_byte(BME280_REGISTER_DIG_H5+1, &dig_H5_1);
    read_byte(BME280_REGISTER_DIG_H5, &dig_H5_0);
    _bme280_calib.dig_H5 = (dig_H5_1 << 4) | (dig_H5_0 >> 4);

    uint8_t dig_H6_0 = 0;
    read_byte(BME280_REGISTER_DIG_H6, &dig_H6_0);
    _bme280_calib.dig_H6 = (int8_t)dig_H6_0;
}

KSensorStatus bme280_read_temperature(float * temp)
{
    int32_t var1, var2;

    int32_t adc_T = 0;
    if (read_24_bit(BME280_REGISTER_TEMPDATA, (uint32_t*)&adc_T) != SENSOR_OK)
    {
        return SENSOR_READ_ERROR;
    }
    adc_T >>= 4;

    var1  = ((((adc_T >> 3) - ((int32_t)_bme280_calib.dig_T1 << 1))) *
      ((int32_t)_bme280_calib.dig_T2)) >> 11;

    var2  = (((((adc_T >> 4) - ((int32_t)_bme280_calib.dig_T1)) *
      ((adc_T >> 4) - ((int32_t)_bme280_calib.dig_T1))) >> 12) *
      ((int32_t)_bme280_calib.dig_T3)) >> 14;

    t_fine = var1 + var2;

    float T  = (t_fine * 5 + 128) >> 8;
    *temp = T/100;

    return SENSOR_OK;
}

KSensorStatus bme280_read_pressure(float * press)
{
    int64_t var1, var2, p;

    float temp = 0;

#ifdef TARGET_LIKE_MSP430
    return SENSOR_ERROR; /* 64 bit int not supported on MSP */
#else
    bme280_read_temperature(&temp); /* get up to date t_fine */

    uint32_t adc_temp = 0;
    if (read_24_bit(BME280_REGISTER_PRESSUREDATA, &adc_temp) != SENSOR_OK)
    {
        return SENSOR_READ_ERROR;
    }
    int32_t adc_P = (int32_t)adc_temp;
    adc_P >>= 4;

    var1 = ((int64_t)t_fine) - 128000;
    var2 = var1 * var1 * (int64_t)_bme280_calib.dig_P6;
    var2 = var2 + ((var1 * (int64_t)_bme280_calib.dig_P5) << 17);
    var2 = var2 + (((int64_t)_bme280_calib.dig_P4) << 35);
    var1 = ((var1 * var1 * (int64_t)_bme280_calib.dig_P3) >> 8) +
      ((var1 * (int64_t)_bme280_calib.dig_P2) << 12);
    var1 = (((((int64_t)1) << 47) + var1)) * ((int64_t)_bme280_calib.dig_P1) >> 33;

    if (var1 == 0) {
        return SENSOR_ERROR;  /* avoid exception caused by division by zero */
    }

    p = 1048576 - adc_P;
    p = (((p << 31) - var2) * 3125) / var1;
    var1 = (((int64_t)_bme280_calib.dig_P9) * (p >> 13) * (p >> 13)) >> 25;
    var2 = (((int64_t)_bme280_calib.dig_P8) * p) >> 19;

    p = ((p + var1 + var2) >> 8) + (((int64_t)_bme280_calib.dig_P7)<<4);
    *press = (float)p/256;

    return SENSOR_OK;
#endif
}

KSensorStatus bme280_read_humidity(float * hum)
{
    float temp = 0;
    bme280_read_temperature(&temp); /* get up to date t_fine */

    uint16_t adc_temp = 0;
    if (read_16_bit(BME280_REGISTER_HUMIDDATA, &adc_temp) != SENSOR_OK)
    {
        return SENSOR_READ_ERROR;
    }
    int32_t adc_H = (int32_t)adc_temp;

    int32_t v_x1_u32r;

    v_x1_u32r = (t_fine - ((int32_t)76800));

    v_x1_u32r = (((((adc_H << 14) - (((int32_t)_bme280_calib.dig_H4) << 20) -
      (((int32_t)_bme280_calib.dig_H5) * v_x1_u32r)) + ((int32_t)16384)) >> 15) *
      (((((((v_x1_u32r * ((int32_t)_bme280_calib.dig_H6)) >> 10) *
      (((v_x1_u32r * ((int32_t)_bme280_calib.dig_H3)) >> 11) + ((int32_t)32768))) >> 10) +
      ((int32_t)2097152)) * ((int32_t)_bme280_calib.dig_H2) + 8192) >> 14));

    v_x1_u32r = (v_x1_u32r - (((((v_x1_u32r >> 15) * (v_x1_u32r >> 15)) >> 7) *
      ((int32_t)_bme280_calib.dig_H1)) >> 4));

    v_x1_u32r = (v_x1_u32r < 0) ? 0 : v_x1_u32r;
    v_x1_u32r = (v_x1_u32r > 419430400) ? 419430400 : v_x1_u32r;
    float h = (v_x1_u32r>>12);
    *hum =  h / 1024.0;

    return SENSOR_OK;
}


KSensorStatus bme280_read_altitude(float seaLevel, float * alt)
{
    KSensorStatus ret;
    /*
     * Equation taken from BMP180 datasheet (page 16):
     * http://www.adafruit.com/datasheets/BST-BMP180-DS000-09.pdf

     * Note that using the equation from wikipedia can give bad results
     * at high altitude.  See this thread for more information:
     * http://forums.adafruit.com/viewtopic.php?f=22&t=58064
     */
    #ifdef TARGET_LIKE_MSP430
        return SENSOR_ERROR; /* pow not supported on MSP */
    #else
        float atmospheric = 0;
        ret = bme280_read_pressure(&atmospheric);
        atmospheric /= 100.0F;
        *alt = 44330.0 * (1.0 - pow(atmospheric / seaLevel, 0.1903));

        return ret;
    #endif
}


static KSensorStatus write_byte(uint8_t reg, uint8_t value)
{
    uint8_t shift_reg = reg & ~0x80; /* write, bit 7 low */

    k_gpio_write(CS, 0); /* drive CS low */
    if (k_spi_write(SPI_BUS, &shift_reg, 1) != SPI_OK)
    {
        return SENSOR_WRITE_ERROR;
    }
    if (k_spi_write(SPI_BUS, &value, 1) != SPI_OK)
    {
        return SENSOR_WRITE_ERROR;
    }
    k_gpio_write(CS, 1); /* drive CS high */

    return SENSOR_OK;
}

static KSensorStatus read_byte(uint8_t reg, uint8_t * value)
{
    uint8_t shift_reg = reg | 0x80; /* read, bit 7 high */

    k_gpio_write(CS, 0); /* drive CS low */
    if (k_spi_write(SPI_BUS, &shift_reg, 1) != SPI_OK)
    {
        return SENSOR_WRITE_ERROR;
    }
    if (k_spi_read(SPI_BUS, value, 1) != SPI_OK)
    {
        return SENSOR_READ_ERROR;
    }
    k_gpio_write(CS, 1); /* drive CS high */

    return SENSOR_OK;
}

static KSensorStatus read_16_bit(uint8_t reg, uint16_t * value)
{
    uint8_t values[2]; /* 2 bytes */
    uint8_t* pValues;
    /* set pointer */
    pValues = values;
    /* return var */
    uint16_t shifted_values;

    uint8_t shift_reg = reg | 0x80; /* read, bit 7 high */

    k_gpio_write(CS, 0); /* drive CS low */
    if (k_spi_write(SPI_BUS, &shift_reg, 1) != SPI_OK)
    {
        return SENSOR_WRITE_ERROR;
    }
    if (k_spi_read(SPI_BUS, pValues, 2) != SPI_OK) /* 2 bytes */
    {
        return SENSOR_READ_ERROR;
    }
    k_gpio_write(CS, 1); /* drive CS high */

    /* shift bits and return as unsigned 16 */
    shifted_values = values[0];
    shifted_values <<= 8;
    shifted_values |= values[1];

    *value = shifted_values;

    return SENSOR_OK;
}

static KSensorStatus read_24_bit(uint8_t reg, uint32_t * value)
{
    uint8_t values[3]; /* 3 bytes */
    uint8_t* pValues;
    /* set pointer */
    pValues = values;
    /* return var */
    uint32_t shifted_values;

    uint8_t shift_reg = reg | 0x80; /* read, bit 7 high */

    k_gpio_write(CS, 0); /* drive CS low */
    if (k_spi_write(SPI_BUS, &shift_reg, 1) != SPI_OK)
    {
        return SENSOR_WRITE_ERROR;
    }
    if (k_spi_read(SPI_BUS, pValues, 3) != SPI_OK) /* 3 bytes */
    {
        return SENSOR_READ_ERROR;
    }
    k_gpio_write(CS, 1); /* drive CS high */

    /* shift bits and return as unsigned 32 */
    shifted_values = values[0];
    shifted_values <<= 8;
    shifted_values |= values[1];
    shifted_values <<= 8;
    shifted_values |= values[2];

    *value = shifted_values;

    return SENSOR_OK;
}

static KSensorStatus read_16_bit_LE(uint8_t reg, uint16_t * value)
{
    KSensorStatus ret = SENSOR_ERROR;
    uint16_t temp = 0;

    ret = read_16_bit(reg, &temp);
    /* shift bits and return */
    *value = (temp >> 8) | (temp << 8);

    return ret;
}

static KSensorStatus read_signed_16_bit(uint8_t reg, int16_t * value)
{
    KSensorStatus ret = SENSOR_ERROR;
    uint16_t temp = 0;

    ret = read_16_bit(reg, &temp);
    *value = (int16_t)temp;

    return ret;
}

static KSensorStatus read_signed_16_bit_LE(uint8_t reg, int16_t * value)
{
    KSensorStatus ret = SENSOR_ERROR;
    uint16_t temp = 0;

    ret = read_16_bit_LE(reg, &temp);
    *value = (int16_t)temp;

    return ret;
}

#endif
