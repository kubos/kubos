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
	slave.dev1 = 0x28; // BNO055
	slave.dev2 = 0x40; // HTU21D

	/* set upper hal */
	msp_i2c->k_i2c = k_i2c;

	/* init i2c bus and registers */
	if (k_i2c->bus_num == K_I2C1)
	{
		msp_i2c->reg = (hal_i2c_mem_reg *)__MSP430_BASEADDRESS_USCI_B0__;
		P3SEL |= BIT1 | BIT0;
	}
	else /* HAL_I2C_B1 */
	{
		msp_i2c->reg = (hal_i2c_mem_reg *)__MSP430_BASEADDRESS_USCI_B1__;
		P4SEL |= BIT2 | BIT1;  /* Assign I2C pins BIT1,2 */
	}

	/* set device being used */
	if (i2c == K_I2CDev1)
	{
		msp_i2c->k_i2c->conf.SlaveAddress = slave.dev1;
	}
	else /* other device */
	{
		msp_i2c->k_i2c->conf.SlaveAddress = slave.dev2;
	}

	/* initialize */
	msp_i2c->reg->control1 |= UCSWRST; /* software reset */
	msp_i2c->reg->control0 |= UCMODE_3 | UCMST | UCSYNC; /* I2C, Master, sync */
	msp_i2c->reg->control1 |= UCSSEL_2 | UCSWRST; /* SMCLK + keep reset */
	msp_i2c->reg->baudrate0 = 0x80; /* prescaler divider for 8 MHz SMCLK -> 100 KHz */
	msp_i2c->reg->baudrate1 = 0;    /* 80d = 0x50h */
	msp_i2c->reg->slaveAddress = msp_i2c->k_i2c->conf.SlaveAddress; /* set slave addr */
	msp_i2c->reg->control1 &= ~UCSWRST; /* enable I2C by releasing reset */

	/* set address in HAL */
	k_i2c->conf.OwnAddress1 = msp_i2c->reg->ownAddress;

    /* set status idle */
    k_i2c->status = I2C_IDLE;
}

int kprv_i2c_transmit_i2c(KI2CDevNum i2c, uint8_t *ptr, int len)
{
	msp_i2c *msp_i2c = kprv_msp_i2c_get(i2c);

	/* clear buffer */
	msp_i2c->reg->txBuffer = 0;

	/* reset interrupt flag */
	msp_i2c->reg->interruptFlags = 0;

	/* set slave address */
	msp_i2c->reg->slaveAddress = msp_i2c->k_i2c->conf.SlaveAddress;
	/* set write mode */
	msp_i2c->k_i2c->status = I2C_WRITE;

	/* set I2C start condition and UCTR */
	msp_i2c->reg->control1 |= UCTR;
	msp_i2c->reg->control1 |= UCTXSTT;


	return kprv_hal_i2c_state_machine(i2c, ptr, len);
}

int kprv_i2c_receive_i2c(KI2CDevNum i2c, uint8_t *ptr, int len)
{
	msp_i2c *msp_i2c = kprv_msp_i2c_get(i2c);

	/* clear buffer */
	msp_i2c->reg->rxBuffer = 0;

	/* reset interrupt flag */
	msp_i2c->reg->interruptFlags = 0;

	/* set slave address */
	msp_i2c->reg->slaveAddress = msp_i2c->k_i2c->conf.SlaveAddress;
	/* set read mode */
	msp_i2c->k_i2c->status = I2C_READ;

	/* set I2C start condition and clear UCTR */
	msp_i2c->reg->control1 &= ~UCTR;
	msp_i2c->reg->control1 |= UCTXSTT;

	/* wait for STT to clear (slave recevied address) */
	while (msp_i2c->reg->control1 & UCTXSTT);

	/* if only sending 1 byte */
	if(len == 1)
	{
		/* set stop bit immediately */
		msp_i2c->reg->control1 |= UCTXSTP;
	}

	/* slave not responding? */
	if (msp_i2c->reg->interruptFlags & UCNACKIFG)
	{
		/* if not sent already (len =/= 1) */
		if (msp_i2c->reg->control1 & UCTXSTP)
		{
			/* trigger a stop condition */
			msp_i2c->reg->control1 |= UCTXSTP;
			/* stop condition sent? */
			while (msp_i2c->reg->control1 & UCTXSTP);
		}
		/* set idle status */
		msp_i2c->k_i2c->status = I2C_IDLE;
		return -1; /* error */
	}

	/* wait until RXIFG to be set to begin receiving data */
	while(!(msp_i2c->reg->interruptFlags & UCRXIFG));

	return kprv_hal_i2c_state_machine(i2c, ptr, len);
}




