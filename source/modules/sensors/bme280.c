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

#include "kubos-core/modules/sensors/bme280.h"
#include "kubos-hal/gpio.h"
#include "FreeRTOS.h"
#include "task.h"

#ifndef SPI_BUS
#define SPI_BUS YOTTA_CFG_SENSORS_BME280_SPI_BUS
#endif

#ifndef CS
#define CS YOTTA_CFG_SENSORS_BME280_CS
#endif

/* globals */
bme280_calib_data _bme280_calib;
int32_t t_fine = 0;

/* static functions */
static void bme280_read_coefficients(void);

static KSensorStatus write_byte(uint8_t reg, uint8_t value);
static uint8_t read_byte(uint8_t reg);
static uint16_t read_16_bit(uint8_t reg);
static uint32_t read_24_bit(uint8_t reg);
static int16_t read_signed_16_bit(uint8_t reg);
static uint16_t read_16_bit_LE(uint8_t reg);
static int16_t read_signed_16_bit_LE(uint8_t reg);

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
    while (read_byte(BME280_REGISTER_CHIPID) != 0x60)
    {
        vTaskDelay(50);

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
    _bme280_calib.dig_T1 = read_16_bit_LE(BME280_REGISTER_DIG_T1);
    _bme280_calib.dig_T2 = read_signed_16_bit_LE(BME280_REGISTER_DIG_T2);
    _bme280_calib.dig_T3 = read_signed_16_bit_LE(BME280_REGISTER_DIG_T3);

    _bme280_calib.dig_P1 = read_16_bit_LE(BME280_REGISTER_DIG_P1);
    _bme280_calib.dig_P2 = read_signed_16_bit_LE(BME280_REGISTER_DIG_P2);
    _bme280_calib.dig_P3 = read_signed_16_bit_LE(BME280_REGISTER_DIG_P3);
    _bme280_calib.dig_P4 = read_signed_16_bit_LE(BME280_REGISTER_DIG_P4);
    _bme280_calib.dig_P5 = read_signed_16_bit_LE(BME280_REGISTER_DIG_P5);
    _bme280_calib.dig_P6 = read_signed_16_bit_LE(BME280_REGISTER_DIG_P6);
    _bme280_calib.dig_P7 = read_signed_16_bit_LE(BME280_REGISTER_DIG_P7);
    _bme280_calib.dig_P8 = read_signed_16_bit_LE(BME280_REGISTER_DIG_P8);
    _bme280_calib.dig_P9 = read_signed_16_bit_LE(BME280_REGISTER_DIG_P9);

    _bme280_calib.dig_H1 = read_byte(BME280_REGISTER_DIG_H1);
    _bme280_calib.dig_H2 = read_signed_16_bit_LE(BME280_REGISTER_DIG_H2);
    _bme280_calib.dig_H3 = read_byte(BME280_REGISTER_DIG_H3);
    _bme280_calib.dig_H4 = (read_byte(BME280_REGISTER_DIG_H4) << 4) |
      (read_byte(BME280_REGISTER_DIG_H4+1) & 0xF);
    _bme280_calib.dig_H5 = (read_byte(BME280_REGISTER_DIG_H5+1) << 4) |
      (read_byte(BME280_REGISTER_DIG_H5) >> 4);
    _bme280_calib.dig_H6 = (int8_t)read_byte(BME280_REGISTER_DIG_H6);
}

float bme280_read_temperature(void)
{
    int32_t var1, var2;

    int32_t adc_T = read_24_bit(BME280_REGISTER_TEMPDATA);
    adc_T >>= 4;

    var1  = ((((adc_T >> 3) - ((int32_t)_bme280_calib.dig_T1 << 1))) *
      ((int32_t)_bme280_calib.dig_T2)) >> 11;

    var2  = (((((adc_T >> 4) - ((int32_t)_bme280_calib.dig_T1)) *
      ((adc_T >> 4) - ((int32_t)_bme280_calib.dig_T1))) >> 12) *
      ((int32_t)_bme280_calib.dig_T3)) >> 14;

    t_fine = var1 + var2;

    float T  = (t_fine * 5 + 128) >> 8;
    return T/100;
}

float bme280_read_pressure(void)
{
    int64_t var1, var2, p;

    bme280_read_temperature(); /* get up to date t_fine */

    int32_t adc_P = read_24_bit(BME280_REGISTER_PRESSUREDATA);
    adc_P >>= 4;

    var1 = ((int64_t)t_fine) - 128000;
    var2 = var1 * var1 * (int64_t)_bme280_calib.dig_P6;
    var2 = var2 + ((var1 * (int64_t)_bme280_calib.dig_P5) << 17);
    var2 = var2 + (((int64_t)_bme280_calib.dig_P4) << 35);
    var1 = ((var1 * var1 * (int64_t)_bme280_calib.dig_P3) >> 8) +
      ((var1 * (int64_t)_bme280_calib.dig_P2) << 12);
    var1 = (((((int64_t)1) << 47) + var1)) * ((int64_t)_bme280_calib.dig_P1) >> 33;

    if (var1 == 0) {
        return 0;  /* avoid exception caused by division by zero */
    }

    p = 1048576 - adc_P;
    p = (((p << 31) - var2) * 3125) / var1;
    var1 = (((int64_t)_bme280_calib.dig_P9) * (p >> 13) * (p >> 13)) >> 25;
    var2 = (((int64_t)_bme280_calib.dig_P8) * p) >> 19;

    p = ((p + var1 + var2) >> 8) + (((int64_t)_bme280_calib.dig_P7)<<4);
    return (float)p/256;
}

float bme280_read_humidity(void)
{
    bme280_read_temperature(); /* get up to date t_fine */

    int32_t adc_H = read_16_bit(BME280_REGISTER_HUMIDDATA);

    int32_t v_x1_u32r;

    #ifdef TARGET_LIKE_MSP430
        return -1.00; /* 64 bit int not supported on MSP */
    #else
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

    return  h / 1024.0;
    #endif
}


float bme280_read_altitude(float seaLevel)
{
    /*
     * Equation taken from BMP180 datasheet (page 16):
     * http://www.adafruit.com/datasheets/BST-BMP180-DS000-09.pdf

     * Note that using the equation from wikipedia can give bad results
     * at high altitude.  See this thread for more information:
     * http://forums.adafruit.com/viewtopic.php?f=22&t=58064
     */
    #ifdef TARGET_LIKE_MSP430
        return -1.00; /* pow not supported on MSP */
    #else
        float atmospheric = bme280_read_pressure() / 100.0F;
        return 44330.0 * (1.0 - pow(atmospheric / seaLevel, 0.1903));
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

static uint8_t read_byte(uint8_t reg)
{
    uint8_t value = 0;
    uint8_t shift_reg = reg | 0x80; /* read, bit 7 high */

    k_gpio_write(CS, 0); /* drive CS low */
    k_spi_write(SPI_BUS, &shift_reg, 1);
    k_spi_read(SPI_BUS, &value, 1);
    k_gpio_write(CS, 1); /* drive CS high */

    return value;
}

static uint16_t read_16_bit(uint8_t reg)
{
    uint8_t values[2]; /* 2 bytes */
    uint8_t* pValues;
    /* set pointer */
    pValues = values;
    /* return var */
    uint16_t shifted_values;

    uint8_t shift_reg = reg | 0x80; /* read, bit 7 high */

    k_gpio_write(CS, 0); /* drive CS low */
    k_spi_write(SPI_BUS, &shift_reg, 1);
    k_spi_read(SPI_BUS, pValues, 2); /* 2 bytes */
    k_gpio_write(CS, 1); /* drive CS high */

    /* shift bits and return as unsigned 16 */
    shifted_values = values[0];
    shifted_values <<= 8;
    shifted_values |= values[1];
    return shifted_values;
}

static uint32_t read_24_bit(uint8_t reg)
{
    uint8_t values[3]; /* 3 bytes */
    uint8_t* pValues;
    /* set pointer */
    pValues = values;
    /* return var */
    uint32_t shifted_values;

    uint8_t shift_reg = reg | 0x80; /* read, bit 7 high */

    k_gpio_write(CS, 0); /* drive CS low */
    k_spi_write(SPI_BUS, &shift_reg, 1);
    k_spi_read(SPI_BUS, pValues, 3); /* 3 bytes */
    k_gpio_write(CS, 1); /* drive CS high */

    /* shift bits and return as unsigned 32 */
    shifted_values = values[0];
    shifted_values <<= 8;
    shifted_values |= values[1];
    shifted_values <<= 8;
    shifted_values |= values[2];
    return shifted_values;
}

static uint16_t read_16_bit_LE(uint8_t reg)
{
    uint16_t temp;

    temp = read_16_bit(reg);
    /* shift bits and return */
    return (temp >> 8) | (temp << 8);
}

static int16_t read_signed_16_bit(uint8_t reg)
{
    return (int16_t)read_16_bit(reg);
}

static int16_t read_signed_16_bit_LE(uint8_t reg)
{
    return (int16_t)read_16_bit_LE(reg);
}
