/***************************************************************************
 This is a library for the BNO055 orientation sensor

 Designed specifically to work with the Adafruit BNO055 Breakout.

 Pick one up today in the adafruit shop!
 ------> http://www.adafruit.com/products

 These sensors use I2C to communicate, 2 pins are required to interface.

 Adafruit invests time and resources providing this open source code,
 please support Adafruit andopen-source hardware by purchasing products
 from Adafruit!

 Written by KTOWN for Adafruit Industries.

 MIT license, all text above must be included in any redistribution
 ***************************************************************************/
/**
 * This library has been modified by Kubos to utilize the Kubos-HAL I2C interface
 */

/**
 * @defgroup BNO055 BNO055 IMU Sensor
 * @addgroup BNO055
 * @{
 */

#ifdef YOTTA_CFG_SENSORS_BNO055
#ifndef BNO055_H
#define BNO055_H

#include <stdint.h>
#include "kubos-hal/i2c.h"
#include "kubos-core/modules/sensors/sensors.h"

typedef enum
{
    /* Page id register definition */
    BNO055_PAGE_ID_ADDR = 0x07,

    /* PAGE0 REGISTER DEFINITION START*/
    BNO055_CHIP_ID_ADDR = 0x00,
    BNO055_ACCEL_REV_ID_ADDR = 0x01,
    BNO055_MAG_REV_ID_ADDR = 0x02,
    BNO055_GYRO_REV_ID_ADDR = 0x03,
    BNO055_SW_REV_ID_LSB_ADDR = 0x04,
    BNO055_SW_REV_ID_MSB_ADDR = 0x05,
    BNO055_BL_REV_ID_ADDR = 0x06,

    /* Accel data register */
    BNO055_ACCEL_DATA_X_LSB_ADDR = 0x08,

    /* Mag data register */
    BNO055_MAG_DATA_X_LSB_ADDR = 0x0E,

    /* Gyro data registers */
    BNO055_GYRO_DATA_X_LSB_ADDR = 0x14,

    /* Euler data registers */
    BNO055_EULER_H_LSB_ADDR = 0x1A,

    /* Quaternion data registers */
    BNO055_QUATERNION_DATA_W_LSB_ADDR = 0x20,

    /* Linear acceleration data registers */
    BNO055_LINEAR_ACCEL_DATA_X_LSB_ADDR = 0x28,

    /* Gravity data registers */
    BNO055_GRAVITY_DATA_X_LSB_ADDR = 0x2E,

    /* Temperature data register */
    BNO055_TEMP_ADDR = 0x34,

    /* Status registers */
    BNO055_CALIB_STAT_ADDR = 0x35,
    BNO055_SELFTEST_RESULT_ADDR = 0x36,
    BNO055_INTR_STAT_ADDR = 0x37,

    BNO055_SYS_CLK_STAT_ADDR = 0x38,
    BNO055_SYS_STAT_ADDR = 0x39,
    BNO055_SYS_ERR_ADDR = 0x3A,

    /* Unit selection register */
    BNO055_UNIT_SEL_ADDR = 0x3B,
    BNO055_DATA_SELECT_ADDR = 0x3C,

    /* Mode registers */
    BNO055_OPR_MODE_ADDR = 0x3D,
    BNO055_PWR_MODE_ADDR = 0x3E,

    BNO055_SYS_TRIGGER_ADDR = 0x3F,
    BNO055_TEMP_SOURCE_ADDR = 0x40,

    /* Axis remap registers */
    BNO055_AXIS_MAP_CONFIG_ADDR = 0x41,
    BNO055_AXIS_MAP_SIGN_ADDR = 0x42,


    /* Accelerometer Offset registers */
    ACCEL_OFFSET_X_LSB_ADDR = 0x55,
    ACCEL_OFFSET_X_MSB_ADDR = 0x56,
    ACCEL_OFFSET_Y_LSB_ADDR = 0x57,
    ACCEL_OFFSET_Y_MSB_ADDR = 0x58,
    ACCEL_OFFSET_Z_LSB_ADDR = 0x59,
    ACCEL_OFFSET_Z_MSB_ADDR = 0x5A,

    /* Magnetometer Offset registers */
    MAG_OFFSET_X_LSB_ADDR = 0x5B,
    MAG_OFFSET_X_MSB_ADDR = 0x5C,
    MAG_OFFSET_Y_LSB_ADDR = 0x5D,
    MAG_OFFSET_Y_MSB_ADDR = 0x5E,
    MAG_OFFSET_Z_LSB_ADDR = 0x5F,
    MAG_OFFSET_Z_MSB_ADDR = 0x60,

    /* Gyroscope Offset register s*/
    GYRO_OFFSET_X_LSB_ADDR = 0x61,
    GYRO_OFFSET_X_MSB_ADDR = 0x62,
    GYRO_OFFSET_Y_LSB_ADDR = 0x63,
    GYRO_OFFSET_Y_MSB_ADDR = 0x64,
    GYRO_OFFSET_Z_LSB_ADDR = 0x65,
    GYRO_OFFSET_Z_MSB_ADDR = 0x66,

    /* Radius registers */
    ACCEL_RADIUS_LSB_ADDR = 0x67,
    ACCEL_RADIUS_MSB_ADDR = 0x68,
    MAG_RADIUS_LSB_ADDR = 0x69,
    MAG_RADIUS_MSB_ADDR = 0x6A
} bno055_reg_t;

typedef enum {
    POWER_MODE_NORMAL = 0x00,
    POWER_MODE_LOWPOWER = 0x01,
    POWER_MODE_SUSPEND = 0x02
} bno055_powermode_t;

typedef enum {
    /* Operation mode settings*/
    OPERATION_MODE_CONFIG = 0x00,
    OPERATION_MODE_ACCONLY = 0x01,
    OPERATION_MODE_MAGONLY = 0x02,
    OPERATION_MODE_GYRONLY = 0x03,
    OPERATION_MODE_ACCMAG = 0x04,
    OPERATION_MODE_ACCGYRO = 0x05,
    OPERATION_MODE_MAGGYRO = 0x06,
    OPERATION_MODE_AMG = 0x07,
    OPERATION_MODE_IMUPLUS = 0x08,
    OPERATION_MODE_COMPASS = 0x09,
    OPERATION_MODE_M4G = 0x0A,
    OPERATION_MODE_NDOF_FMC_OFF = 0x0B,
    OPERATION_MODE_NDOF = 0x0C,
    OPERATION_MODE_INVALID = 0x99
} bno055_opmode_t;

typedef enum {
    REMAP_CONFIG_P0 = 0x21,
    REMAP_CONFIG_P1 = 0x24, // default
    REMAP_CONFIG_P2 = 0x24,
    REMAP_CONFIG_P3 = 0x21,
    REMAP_CONFIG_P4 = 0x24,
    REMAP_CONFIG_P5 = 0x21,
    REMAP_CONFIG_P6 = 0x21,
    REMAP_CONFIG_P7 = 0x24
} bno055_axis_remap_config_t;

typedef enum {
    REMAP_SIGN_P0 = 0x04, REMAP_SIGN_P1 = 0x00, // default
    REMAP_SIGN_P2 = 0x06,
    REMAP_SIGN_P3 = 0x02,
    REMAP_SIGN_P4 = 0x03,
    REMAP_SIGN_P5 = 0x01,
    REMAP_SIGN_P6 = 0x07,
    REMAP_SIGN_P7 = 0x05
} bno055_axis_remap_sign_t;

typedef struct {
    uint8_t accel_rev;
    uint8_t mag_rev;
    uint8_t gyro_rev;
    uint16_t sw_rev;
    uint8_t bl_rev;
} bno055_rev_info_t;

typedef struct {
    uint16_t accel_offset_x;
    uint16_t accel_offset_y;
    uint16_t accel_offset_z;
    uint16_t gyro_offset_x;
    uint16_t gyro_offset_y;
    uint16_t gyro_offset_z;
    uint16_t mag_offset_x;
    uint16_t mag_offset_y;
    uint16_t mag_offset_z;

    uint16_t accel_radius;
    uint16_t mag_radius;
} bno055_offsets_t;

typedef enum
{
    VECTOR_ACCELEROMETER = BNO055_ACCEL_DATA_X_LSB_ADDR,
    VECTOR_MAGNETOMETER = BNO055_MAG_DATA_X_LSB_ADDR,
    VECTOR_GYROSCOPE = BNO055_GYRO_DATA_X_LSB_ADDR,
    VECTOR_EULER = BNO055_EULER_H_LSB_ADDR,
    VECTOR_LINEARACCEL = BNO055_LINEAR_ACCEL_DATA_X_LSB_ADDR,
    VECTOR_GRAVITY = BNO055_GRAVITY_DATA_X_LSB_ADDR
} vector_type_t;

typedef struct
{
    double w;
    double x;
    double y;
    double z;
} bno055_quat_data_t;

typedef struct
{
    double x;
    double y;
    double z;
} bno055_vector_data_t;

typedef struct
{
    uint8_t status;
    uint8_t self_test;
    uint8_t error;
} bno055_system_status_t;

typedef struct
{
    uint8_t sys;
    uint8_t gyro;
    uint8_t accel;
    uint8_t mag;
} bno055_calibration_data_t;

/* config functions */
KSensorStatus bno055_setup(bno055_opmode_t mode);
KSensorStatus bno055_init(bno055_opmode_t mode);
KSensorStatus bno055_set_mode(bno055_opmode_t mode);
KSensorStatus bno055_get_mode(uint8_t * value);

KSensorStatus bno055_get_rev_info(bno055_rev_info_t * info);
KSensorStatus bno055_set_ext_crystal_use(int use);
KSensorStatus bno055_get_system_status(bno055_system_status_t * status);
KSensorStatus bno055_get_calibration(bno055_calibration_data_t * data);

/* data functions */
KSensorStatus bno055_get_single_data(bno055_reg_t reg, uint8_t * value);
KSensorStatus bno055_get_data_vector(vector_type_t type, bno055_vector_data_t * vector);
KSensorStatus bno055_get_position(bno055_quat_data_t * quat);
KSensorStatus bno055_get_temperature(int8_t * temp);

/* Functions to deal with raw calibration data */
KSensorStatus bno055_get_sensor_offset_struct(bno055_offsets_t * offsets_type);
KSensorStatus bno055_set_sensor_offset_struct(const bno055_offsets_t offsets_type);

KSensorStatus bno055_check_calibration(uint8_t * count, uint8_t limit, bno055_offsets_t * calib);

#endif
#endif

/* @} */
