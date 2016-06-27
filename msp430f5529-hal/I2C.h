/*
 * KubOS HAL
 * Copyright (C) 2016 Kubos Corporation
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
 * @defgroup I2C
 * @addtogroup I2C
 * @{
 */

 /**
   *
   * @file      I2C.h
   * @brief      MSP430F5529 HAL - I2C module
   *
   * @author     kubos.co
   */

/**
  * @brief Type by which i2c buses are numbered.
  */
typedef enum
{
    HAL_I2C_B0 = 0,
    HAL_I2C_B1
} hal_i2c_bus;

/**
 * @brief Expected addressing mode of i2c bus.
 */
typedef enum
{
    HAL_I2C_ADDRESSINGMODE_7BIT = 0,
    HAL_I2C_ADDRESSINGMODE_10BIT
} hal_i2c_addressing_mode;

/**
 * @brief Expected role of i2c bus.
 */
typedef enum {
    HAL_I2C_MASTER = 0,
    HAL_I2C_SLAVE
} hal_i2c_role;

/**
 * @brief i2c function status.
 */
typedef enum {
    HAL_I2C_OK = 0,
    HAL_I2C_ERROR
} hal_i2c_status;

/**
  * @brief This type is a map of the USCI_Bx i2c registers.
  */
typedef struct
{    /** UCBxCTL1 */
    volatile uint8_t control1;
    /** UCBxCTL0 */
    volatile uint8_t control0;
    uint8_t padding1[4];
    /** UCBxBR0 */
    volatile uint8_t baudrate0;
    /** UCBxBR1 */
    volatile uint8_t baudrate1;
    uint8_t padding2[2];
    /** UCBxSTAT */
    volatile uint8_t status;
    uint8_t padding3;
    /** UCBxRXBUF */
    volatile uint8_t rxBuffer;
    uint8_t padding4;
    /** UCBxTXBUF */
    volatile uint8_t txBuffer;
    uint8_t padding5;
    /** UCBxI2COA */
    volatile uint8_t ownAddress;
    uint8_t padding6;
    /** UCBxI2CSA */
    volatile uint8_t slaveAddress;
    uint8_t padding7[9];
    /** UCAxIE */
    volatile uint8_t interruptEnable;
    /** UCAxIFG */
    volatile uint8_t interruptFlags;
    /** UCAxIV */
    volatile uint8_t interruptVector;
} hal_i2c_mem_reg;

/**
 * @brief i2c configuration structure.
 */
typedef struct {
	hal_i2c_addressing_mode AddressingMode;
    hal_i2c_role Role;
	uint32_t ClockSpeed;
} hal_i2c_config;

/**
 * @brief i2c bus data structure.
 */
typedef struct {
	hal_i2c_bus bus_num;
    hal_i2c_config conf;
    /* no need for semaphore here */
} hal_i2c_bus_conf;

/**
  * @brief This type contains all i2c config and register details for this layer.
  */
typedef struct
{
	hal_i2c_mem_reg* reg;
	hal_i2c_bus_conf* bus;
	volatile uint8_t * select;
	uint8_t selectVal;
} hal_i2c_handle;

/**
  * @brief Static array of avaiable i2c handles.
  */
extern hal_i2c_handle hal_i2c_buses[];

/**
  * @brief Creates a i2c handle and fills in details associated with
  *        the specified bus.
  * @param i2c Instance of hal_i2c_bus, specifying bus to use.
  * @retval hal_i2c_handle *
  */
hal_i2c_handle * hal_i2c_device_init(hal_i2c_bus i2c);

/**
  * @brief Creates a i2c handle according to details specified in config.
  * @param config Instance of hal_i2c_config with init details.
  * @param i2c Instance of hal_i2c_bus, specifying bus to use.
  * @retval hal_i2c_handle *
  */
hal_i2c_handle * hal_i2c_init(hal_i2c_config config, hal_i2c_bus i2c);

/**
  * @brief Low level hardware de-init of i2c.
  * @param handle Instance of initilized hal_i2c_handle containing hardware
  *               registers and config values.
  */
void hal_i2c_dev_terminate(hal_i2c_handle * handle);

/**
  * @brief Low level hardware setup of i2c.
  * @param handle Instance of initilaized hal_i2c_handle containing hardware
  *               registers and config values.
  */
void hal_i2c_setup(hal_i2c_handle * handle);

static void hal_i2c_set_addressing(hal_i2c_handle * handle);

static void hal_i2c_set_clock(hal_i2c_handle * handle);

/**
  * @brief Writes a buffer to a slave device over i2c.
  * @param handle i2c bus handle to transmit on
  * @param addr slave address to write to
  * @param ptr buffer pointer to write
  * @param len number of characters to write
  */
hal_i2c_status hal_i2c_master_write_state_machine(hal_i2c_handle * handle, uint16_t addr, uint8_t *ptr, int len);

/**
  * @brief Reads a buffer from a slave device over i2c.
  * @param handle i2c bus handle to read from
  * @param addr slave address to read from
  * @param ptr buffer pointer to read to
  * @param len number of characters to read
  */
hal_i2c_status hal_i2c_master_read_state_machine(hal_i2c_handle * handle, uint16_t addr, uint8_t *ptr, int len);


