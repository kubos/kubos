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
 * @defgroup KUBOS_CORE_BNO055 Kubos Core BNO055 Sensor Interface
 * @addtogroup KUBOS_CORE_BNO055
 * @{
 */

#ifdef YOTTA_CFG_SENSORS_BNO055
#ifndef BNO055_H
#define BNO055_H

#include <stdint.h>
#include "kubos-hal/i2c.h"
#include "kubos-core/modules/sensors/sensors.h"

/**
 * Register Map for BNO055 Sensor
 */
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

/**
 * Power mode values for BNO055 sensor
 */
typedef enum {
    POWER_MODE_NORMAL = 0x00, /**< Sensor should operate in normal mode */
    POWER_MODE_LOWPOWER = 0x01, /**< Sensor should operate in low power mode */
    POWER_MODE_SUSPEND = 0x02 /**< Sensor should operate in suspended mode */
} bno055_powermode_t;

/**
 * Operation mode values for BNO055 sensor
 */
typedef enum {
    /* Operation mode settings*/
    /**
     *  @brief Configuration mode
     *
     *  Default mode after power-on or RESET. Used to to configure power mode,
     *  output units, and axis mapping.
     *
     */
    OPERATION_MODE_CONFIG = 0x00,
    OPERATION_MODE_ACCONLY = 0x01, /**< Only accelerometer data will be gathered */
    OPERATION_MODE_MAGONLY = 0x02, /**< Only magnetometer data will be gathered */
    OPERATION_MODE_GYRONLY = 0x03, /**< Only gyroscope data will be gathered */
    OPERATION_MODE_ACCMAG = 0x04, /**< Accelerometer and magnetometer data will be gathered */
    OPERATION_MODE_ACCGYRO = 0x05, /**< Accelerometer and gyroscope data will be gathered */
    OPERATION_MODE_MAGGYRO = 0x06, /**< Gyroscope and magnetometer data will be gathered */
    OPERATION_MODE_AMG = 0x07, /**< Accelerometer,gyroscope, and magnetometer data will be gathered */
    OPERATION_MODE_IMUPLUS = 0x08, /**< Relative orientation in space will be calculated using accelerometer and gyroscope (fast) */
    OPERATION_MODE_COMPASS = 0x09, /**< The geographic direction of the sensor will be calculated */
    OPERATION_MODE_M4G = 0x0A, /**< Relative orientation in space will be calculated using accelerometer and magnetometer */
    OPERATION_MODE_NDOF_FMC_OFF = 0x0B, /**< NDOF without fast magnetometer calibration */
    OPERATION_MODE_NDOF = 0x0C, /**< Absolute orientation will be calculated from Accelerometer,gyroscope, and magnetometer data */
    OPERATION_MODE_INVALID = 0x99 /**< Extra value to indicate an error */
} bno055_opmode_t;

/**
 * Configuration values to configure sensor to new reference axis
 *
 * Default: REMAP_CONFIG_P1 (0x24)
 */
typedef enum {
    REMAP_CONFIG_P0 = 0x21,
    REMAP_CONFIG_P1 = 0x24, /**< Default configuration */
    REMAP_CONFIG_P2 = 0x24,
    REMAP_CONFIG_P3 = 0x21,
    REMAP_CONFIG_P4 = 0x24,
    REMAP_CONFIG_P5 = 0x21,
    REMAP_CONFIG_P6 = 0x21,
    REMAP_CONFIG_P7 = 0x24
} bno055_axis_remap_config_t;

/**
 * Axis signs to be used with new reference axis. P* values should
 * be the same when using REMAP_CONFIG_P* and REMAP_SIGN_P*.
 *
 * Default: REMAP_SIGN_P1 (0x00)
 */
typedef enum {
    REMAP_SIGN_P0 = 0x04,
    REMAP_SIGN_P1 = 0x00, /**< Default signs */
    REMAP_SIGN_P2 = 0x06,
    REMAP_SIGN_P3 = 0x02,
    REMAP_SIGN_P4 = 0x03,
    REMAP_SIGN_P5 = 0x01,
    REMAP_SIGN_P6 = 0x07,
    REMAP_SIGN_P7 = 0x05
} bno055_axis_remap_sign_t;

/**
 * Storage structure for system ID values
 */
typedef struct {
    uint8_t accel_rev; /**< Accelerometer chip ID */
    uint8_t mag_rev; /**< Magnetometer chip ID */
    uint8_t gyro_rev; /**< Gyroscope chip ID */
    uint16_t sw_rev; /**< Software revision ID */
    uint8_t bl_rev;  /**< Bootloader version */
} bno055_rev_info_t;

/**
 * Storage structure for calibration values
 */
typedef struct {
    uint16_t accel_offset_x; /**< Accelerometer x-axis offset */
    uint16_t accel_offset_y; /**< Accelerometer y-axis offset */
    uint16_t accel_offset_z; /**< Accelerometer z-axis offset */
    uint16_t gyro_offset_x; /**< Gyroscope x-axis offset */
    uint16_t gyro_offset_y; /**< Gyroscope y-axis offset */
    uint16_t gyro_offset_z; /**< Gyroscope z-axis offset */
    uint16_t mag_offset_x; /**< Magnetometer x-axis offset */
    uint16_t mag_offset_y; /**< Magnetometer y-axis offset */
    uint16_t mag_offset_z; /**< Magnetometer z-axis offset */

    uint16_t accel_radius; /**< Accelerometer radius */
    uint16_t mag_radius; /**< Magnetometer radius */
} bno055_offsets_t;

/**
 * Convenience enum to make the appropriate data addresses more readable in code
 */
typedef enum
{
    VECTOR_ACCELEROMETER = BNO055_ACCEL_DATA_X_LSB_ADDR,
    VECTOR_MAGNETOMETER = BNO055_MAG_DATA_X_LSB_ADDR,
    VECTOR_GYROSCOPE = BNO055_GYRO_DATA_X_LSB_ADDR,
    VECTOR_EULER = BNO055_EULER_H_LSB_ADDR,
    VECTOR_LINEARACCEL = BNO055_LINEAR_ACCEL_DATA_X_LSB_ADDR,
    VECTOR_GRAVITY = BNO055_GRAVITY_DATA_X_LSB_ADDR
} vector_type_t;

/**
 * Storage structure for calculated quaternion values
 */
typedef struct
{
    double w; /**< */
    double x; /**< */
    double y; /**< */
    double z; /**< */
} bno055_quat_data_t;

/**
 * Storage structure for calculated vector values
 */
typedef struct
{
    double x; /**< */
    double y; /**< */
    double z; /**< */
} bno055_vector_data_t;

/**
 * Storage structure for system status values
 */
typedef struct
{
    /**
     * System Status (see [section 4.3.58](https://cdn-shop.adafruit.com/datasheets/BST_BNO055_DS000_12.pdf))
     *
     * @code
     * 0 = Idle
     * 1 = System Error
     * 2 = Initializing Peripherals
     * 3 = System Iniitalization
     * 4 = Executing Self-Test
     * 5 = Sensor fusio algorithm running
     * 6 = System running without fusion algorithms
     * @endcode
     */
    uint8_t status;
    /**
     *  Self Test Results
     *
     * @code
     * 1 = test passed, 0 = test failed
     *
     * Bit 0 = Accelerometer self test
     * Bit 1 = Magnetometer self test
     * Bit 2 = Gyroscope self test
     * Bit 3 = MCU self test
     *
     * 0x0F = all good!
     * @endcode
     *
     */
    uint8_t self_test;
    /**
     * System Error Status
     *
     * @code
     * 0 = No error
     * 1 = Peripheral initialization error
     * 2 = System initialization error
     * 3 = Self test result failed
     * 4 = Register map value out of range
     * 5 = Register map address out of range
     * 6 = Register map write error
     * 7 = BNO low power mode not available for selected operat ion mode
     * 8 = Accelerometer power mode not available
     * 9 = Fusion algorithm configuration error
     * A = Sensor configuration error
     * @endcode
     */
    uint8_t error;
} bno055_system_status_t;

/**
 * Storage structure for sensor calibration values
 *
 * For each component:
 *   - 3 - Fully calibrated
 *   - 1,2 - Partially calibrated
 *   - 0 - Not calibrated
 */
typedef struct
{
    uint8_t sys; /**< Overall system calibration status */
    uint8_t gyro; /**< Gyroscope calibration status */
    uint8_t accel; /**< Accelerometer calibration status */
    uint8_t mag; /**< Accelerometer calibration status */
} bno055_calibration_data_t;

/* config functions */

/**
 * Set up default BNO055 connection
 *
 * Starts I2C bus connection with default configuration and then
 * calls ::<bno055_init>
 *
 * @param[in] mode Operation mode to set sensor to after initialization completes
 * @returns KSensorStatus SENSOR_OK if successful, otherwise indicates appropriate error
 */
KSensorStatus bno055_setup(bno055_opmode_t mode);
/**
 * Initialize BNO055 connection
 *
 * Makes necessary writes and reads to initialize sensor and verify that it
 * is running correctly, then sets requested operation mode.
 *
 * @param[in] mode Operation mode to set sensor to
 * @returns KSensorStatus SENSOR_OK if successful, otherwise indicates appropriate error
 */
KSensorStatus bno055_init(bno055_opmode_t mode);
/**
 * Set BNO055 operating mode
 *
 * @param[in] mode Operation mode to set sensor to
 * @returns KSensorStatus SENSOR_OK if successful, otherwise indicates appropriate error
 */
KSensorStatus bno055_set_mode(bno055_opmode_t mode);
/**
 * Get BNO055 operating mode
 *
 * @param[out] mode Sensor's current operating mode
 * @returns KSensorStatus SENSOR_OK if successful, otherwise indicates appropriate error
 */
KSensorStatus bno055_get_mode(uint8_t * value);
/**
 * Get BNO055 system info
 *
 * @param[out] info Sensor's revision and chip ID information
 * @returns KSensorStatus SENSOR_OK if successful, otherwise indicates appropriate error
 */
KSensorStatus bno055_get_rev_info(bno055_rev_info_t * info);
/**
 * Set whether BNO055 should use external crystal oscillator as its clock source
 *
 * Note: If turned on, an external oscillator must be connected to pins XIN32 and XOUT32
 *
 * @param[in] use 1 if external oscillator should be used, 0 if not
 * @returns KSensorStatus SENSOR_OK if successful, otherwise indicates appropriate error
 */
KSensorStatus bno055_set_ext_crystal_use(int use);
/**
 * Get BNO055 system status
 *
 * @param[out] status Sensor's current status, self-test results, and error state
 * @returns KSensorStatus SENSOR_OK if successful, otherwise indicates appropriate error
 */
KSensorStatus bno055_get_system_status(bno055_system_status_t * status);
/**
 * Get BNO055 calibration status
 *
 * @param[out] data Sensor's current calibration status
 * @returns KSensorStatus SENSOR_OK if successful, otherwise indicates appropriate error
 */
KSensorStatus bno055_get_calibration(bno055_calibration_data_t * data);

/* data functions */
/**
 * Get value from specific BNO055 register
 *
 * @param[in] reg Register to read from
 * @param[out] value Pointer to data byte to read to
 * @returns KSensorStatus SENSOR_OK if successful, otherwise indicates appropriate error
 */
KSensorStatus bno055_get_single_data(bno055_reg_t reg, uint8_t * value);
/**
 * Get BNO055 vector data
 *
 * @param[in] type Specifies which vector should be returned
 * @param[out] vector Pointer to vector structure to read to
 * @returns KSensorStatus SENSOR_OK if successful, otherwise indicates appropriate error
 */
KSensorStatus bno055_get_data_vector(vector_type_t type, bno055_vector_data_t * vector);
/**
 * Get BNO055 quaternion data
 *
 * @param[out] quat Pointer to quaternion structure to read to
 * @returns KSensorStatus SENSOR_OK if successful, otherwise indicates appropriate error
 */
KSensorStatus bno055_get_position(bno055_quat_data_t * quat);
/**
 * Get BNO055 temperature data
 *
 * @param[out] temp Pointer to data byte to read to
 * @returns KSensorStatus SENSOR_OK if successful, otherwise indicates appropriate error
 */
KSensorStatus bno055_get_temperature(int8_t * temp);

/* Functions to deal with raw calibration data */
/**
 * Get current BNO055 calibration offsets
 *
 * @param[out] offsets_type Pointer to offsets structure to read to
 * @returns KSensorStatus SENSOR_OK if successful, otherwise indicates appropriate error
 */
KSensorStatus bno055_get_sensor_offset_struct(bno055_offsets_t * offsets_type);
/**
 * Set BNO055 calibration offsets
 *
 * The BNO055's calibration offsets can be set, if desired, removing the need to physically
 * calibrate it.
 *
 * Note: This does not ensure full calibration. Some minimal physical calibration might
 * still be required. However, the amount of time required to calibrate the sensor will be
 * greatly reduced.
 *
 * @param[in] offsets_type Offset values to set
 * @returns KSensorStatus SENSOR_OK if successful, otherwise indicates appropriate error
 */
KSensorStatus bno055_set_sensor_offset_struct(const bno055_offsets_t offsets_type);

/**
 * Check BNO055 calibration status
 *
 * Gets the sensor's current calibration status.
 *
 * If fully calibrated, the current calibration offsets are read.
 *
 * If not, the passed counter is increased. If the counter is greater than the passed limit, then
 * the counter is reset to zero. This allows the caller to retry system calibration completely
 * if too long has passed without the sensor becoming fully calibrated.
 *
 * @param[in] count Pointer to counter
 * @param[in] limit Maximum counter value
 * @param[out] calib Pointer to offset structure to read to
 * @returns KSensorStatus SENSOR_OK if successful, otherwise indicates appropriate error
 */
KSensorStatus bno055_check_calibration(uint8_t * count, uint8_t limit, bno055_offsets_t * calib);

#endif
#endif

/* @} */
