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

#include "kubos-hal/I2C.h"
#include "msp430f5529-hal/I2C.h"

#include <msp430.h>


/* set I2C speed */
#define I2CSPEED 400000

static msp_i2c k_msp_i2cs[K_NUM_I2CS];

msp_i2c* kprv_msp_i2c_get(KI2CDevNum i2c)
{
    return &k_msp_i2cs[i2c];
}

void kprv_i2c_dev_init(KI2CDevNum i2c)
{
	/* get config structs */
	KI2C *k_i2c = kprv_i2c_get(i2c);
	msp_i2c *msp_i2c = &k_msp_i2cs[i2c];
	dev_addr slave;

	/* SET YOUR DEVICE ADDR HERE */
	slave.dev1 = 0x28;
	slave.dev2 = 0x40;

	/* set upper hal */
	msp_i2c->k_i2c = k_i2c;

	/* init i2c bus and registers */
	if (k_i2c->bus_num == K_I2C1)
	{
		msp_i2c->reg = (hal_i2c_mem_reg *)__MSP430_BASEADDRESS_USCI_B0__;
		msp_i2c->word = (hal_i2c_mem_word *)__MSP430_BASEADDRESS_USCI_B0__;
	}
	else /* HAL_I2C_B1 */
	{
		msp_i2c->reg = (hal_i2c_mem_reg *)__MSP430_BASEADDRESS_USCI_B1__;
		msp_i2c->word = (hal_i2c_mem_word *)__MSP430_BASEADDRESS_USCI_B1__;
	}

	/* set device being used */
	if (k_i2c == K_I2CDev1)
	{
		msp_i2c->k_i2c->conf.SlaveAddress = slave.dev1;
	}
	else /* HAL_I2C_B1 */
	{
		msp_i2c->k_i2c->conf.SlaveAddress = slave.dev2;
	}

	/* initialize */
	msp_i2c->word->controlWord |= UCSWRST; /* software reset */
	msp_i2c->word->controlWord |= UCMODE_3 | UCMST | UCSYNC | UCSSEL__SMCLK; /* I2C, Master, sync, SMCLK */
	//msp_i2c->word->baudWord = (uint16_t) (CS_getSMCLK() / I2CSPEED); /* set speed */
	msp_i2c->reg->baudrate0 = 0x3f; /* prescaler */
	msp_i2c->reg->baudrate1 = 0;
	msp_i2c->word->controlWord &= ~UCSWRST; /* enable I2C by releasing reset */

	/* set address in HAL */
	k_i2c->conf.OwnAddress1 = UCB0I2COA;

    /* set status idle */
    k_i2c->status = I2C_IDLE;
}

int kprv_i2c_transmit_i2c(KI2CDevNum i2c, uint8_t *ptr, int len)
{
	msp_i2c *msp_i2c = kprv_msp_i2c_get(i2c);

	/* set slave address */
	msp_i2c->reg->slaveAddress = msp_i2c->k_i2c->conf.SlaveAddress;
	/* set write mode */
	msp_i2c->k_i2c->status = I2C_WRITE;

	/* set I2C start condition */
	msp_i2c->reg->control1 |= UCTR;
	msp_i2c->reg->control1 |= UCTXSTT;

	return k_hal_i2c_state_machine(i2c, *ptr, len);
}

int kprv_i2c_receive_i2c(KI2CDevNum i2c, uint8_t *ptr, int len)
{
	msp_i2c *msp_i2c = kprv_msp_i2c_get(i2c);

	/* set slave address */
	msp_i2c->reg->slaveAddress = msp_i2c->k_i2c->conf.SlaveAddress;
	/* set read mode */
	msp_i2c->k_i2c->status = I2C_READ;

	/* set I2C start condition */
	msp_i2c->reg->control1 |= UCTR;
	msp_i2c->reg->control1 |= UCTXSTT;

	return k_hal_i2c_state_machine(i2c, *ptr, len);
}




