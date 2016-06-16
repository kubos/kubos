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

//#include "kubos-hal/I2C.h"

/* MSP I2C status */
#define I2C_IDLE		   -1
#define I2C_WRITE			0
#define I2C_READ			1
#define I2C_DATA_RECEIVED 	2

typedef struct
{    /** UCBxCTL1 */
    volatile uint8_t control1; // 0x05E0
    /** UCBxCTL0 */
    volatile uint8_t control0; // 0x05E1
    uint8_t padding1[4]; // 0x05E2-5
    /** UCBxBR0 */
    volatile uint8_t baudrate0; // 0x05E6
    /** UCBxBR1 */
    volatile uint8_t baudrate1; // 0x05E7
    uint8_t padding2[2]; // 0x05E8-9
    /** UCBxSTAT */
    volatile uint8_t status; // 0x05EA
    uint8_t padding3; // 0x05EB
    /** UCBxRXBUF */
    volatile uint8_t rxBuffer; // 0x05EC
    uint8_t padding4; // // 0x05ED
    /** UCBxTXBUF */
    volatile uint8_t txBuffer; // 0x05EE
    uint8_t padding5; // 0x05EF
    /** UCBxI2COA */
    volatile uint8_t ownAddress; // 0x05F0
    uint8_t padding6; // 0x05F1
    /** UCBxI2CSA */
    volatile uint8_t slaveAddress; // 0x05F2
    uint8_t padding7[9]; // 0x05F3-B
    /** UCAxIE */
    volatile uint8_t interruptEnable; // 0x05FC
    /** UCAxIFG */
    volatile uint8_t interruptFlags; // 0x05FD
    /** UCAxIV */
    volatile uint8_t interruptVector; // 0x05FE
} hal_i2c_mem_reg;

typedef struct
{
    /** UCBxCTLW0 */
    volatile uint16_t controlWord; // 0x05E0-2
    uint8_t padding1[4]; // 0x05E2-5
    volatile uint16_t baudWord; // 0x05E6-7
} hal_i2c_mem_word;

typedef struct
{	/* reg addr */
	hal_i2c_mem_reg* reg;
	/* ctrl word */
	hal_i2c_mem_word* word;
	/* HAL I2C struct */
	KI2C* k_i2c;
} msp_i2c;



typedef struct
{
	uint8_t dev1;
	uint8_t dev2;
} dev_addr;


// private APIs
msp_i2c* kprv_msp_i2c_get(KI2CDevNum i2c);

