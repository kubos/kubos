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

typedef struct
{
	hal_i2c_mem_reg* reg;
	KI2C* k_i2c;
} msp_i2c;

/* private APIs */
msp_i2c* kprv_msp_i2c_get(KI2CNum i2c);
KI2CStatus kprv_i2c_master_state_machine(KI2CNum i2c, uint8_t *ptr, int len);


