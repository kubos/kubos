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

msp_i2c* kprv_msp_i2c_get(KI2CNum i2c)
{
    return &k_msp_i2cs[i2c];
}

void kprv_i2c_dev_init(KI2CNum i2c)
{
	/* SMCLK FREQ constant 1 MHz */
	const uint32_t SMCLK_FREQ = 1000000;
	uint16_t preScalar;

	/* get config structs */
	KI2C *k_i2c = kprv_i2c_get(i2c);
	msp_i2c *msp_i2c = &k_msp_i2cs[i2c];

	/* set upper hal bus struct */
	msp_i2c->k_i2c = k_i2c;

	/* init i2c bus and registers */
	if(k_i2c->bus_num == K_I2C1)
	{
		msp_i2c->reg = (hal_i2c_mem_reg *)__MSP430_BASEADDRESS_USCI_B0__;
		P3SEL |= BIT1 | BIT0; /* assign I2C pins 0 and 1 */
	}
	else /* HAL_I2C_B1 */
	{
		msp_i2c->reg = (hal_i2c_mem_reg *)__MSP430_BASEADDRESS_USCI_B1__;
		P4SEL |= BIT2 | BIT1;  /* assign I2C pins 1 and 2 */
	}

	msp_i2c->reg->control1 |= UCSWRST; /* software reset */

	/* check addressing mode */
	if(k_i2c->conf.AddressingMode == K_ADDRESSINGMODE_10BIT)
	{
		msp_i2c->reg->control0 |= UCSLA10; /* set 10bit */
	}
	/* set i2c frequency */
	preScalar = (uint16_t)(SMCLK_FREQ/k_i2c->conf.ClockSpeed);

	/* check i2c mode */
	if(k_i2c->conf.Role == K_MASTER)
	{
		msp_i2c->reg->control0 |= UCMST; /* set master bit */
	}

	msp_i2c->reg->control0 |= UCMODE_3 | UCSYNC; /* I2C mode, sync */
	msp_i2c->reg->control1 |= UCSSEL_2 | UCSWRST; /* SMCLK + keep reset */
	msp_i2c->reg->baudrate0 = preScalar;
	msp_i2c->reg->baudrate1 = 0;
	msp_i2c->reg->control1 &= ~UCSWRST; /* enable I2C by releasing reset */
}

KI2CStatus kprv_i2c_master_write(KI2CNum i2c, uint16_t addr, uint8_t *ptr, int len)
{
	msp_i2c *msp_i2c = kprv_msp_i2c_get(i2c);

	/* clear buffer */
	msp_i2c->reg->txBuffer = 0;

	/* reset interrupt flag */
	msp_i2c->reg->interruptFlags = 0;

	/* set slave address */
	msp_i2c->reg->slaveAddress = addr;

	/* set I2C start condition and UCTR */
	msp_i2c->reg->control1 |= UCTR;
	msp_i2c->reg->control1 |= UCTXSTT;

	return kprv_i2c_master_state_machine(i2c, ptr, len);
}

KI2CStatus kprv_i2c_master_read(KI2CNum i2c, uint16_t addr, uint8_t *ptr, int len)
{
	msp_i2c *msp_i2c = kprv_msp_i2c_get(i2c);

	/* clear buffer */
	msp_i2c->reg->rxBuffer = 0;
	/* reset interrupt flag */
	msp_i2c->reg->interruptFlags = 0;

	/* set slave address */
	msp_i2c->reg->slaveAddress = addr;

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

		return I2C_ERROR; /* error */
	}

	/* wait until RXIFG to be set to begin receiving data */
	while(!(msp_i2c->reg->interruptFlags & UCRXIFG));

	return kprv_i2c_master_state_machine(i2c, ptr, len);
}




