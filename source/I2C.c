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

hal_i2c_handle hal_i2c_buses[YOTTA_CFG_HARDWARE_I2CCOUNT];

hal_i2c_handle * hal_i2c_device_init(hal_i2c_bus i2c)
{
	hal_i2c_handle * handle = NULL;

	handle = &hal_i2c_buses[i2c];
    if (HAL_I2C_B0 == i2c)
    {
        handle->select = &P3SEL;
        handle->selectVal = BIT1 + BIT0;
        handle->reg = (hal_i2c_mem_reg *)__MSP430_BASEADDRESS_USCI_B0__;
    }
    else if (HAL_I2C_B1 == i2c)
    {
        handle->select = &P4SEL;
        handle->selectVal = BIT2 + BIT1;
        handle->reg = (hal_i2c_mem_reg *)__MSP430_BASEADDRESS_USCI_B1__;
    }
    return handle;
}

hal_i2c_handle * hal_i2c_init(hal_i2c_config config, hal_i2c_bus i2c)
{
    hal_i2c_handle * handle = hal_i2c_device_init(i2c);
    handle->bus->conf = config;
    return handle;
}

void hal_i2c_dev_terminate(hal_i2c_handle * handle)
{
	handle->reg->control1 |= UCSWRST; /* software reset */
    /* de-select pins */
    *(handle->select) &= ~handle->selectVal;

}

void hal_i2c_setup(hal_i2c_handle * handle)
{
	handle->reg->control1 |= UCSWRST; /* software reset */
	handle->reg->control0 |= UCMODE_3 | UCSYNC; /* I2C mode, sync */

	hal_i2c_set_addressing(handle);

	hal_i2c_set_clock(handle);

    /* configure pins */
    *(handle->select) |= handle->selectVal;

	handle->reg->control1 &= ~UCSWRST; /* enable I2C by releasing reset */
}

static void hal_i2c_set_addressing(hal_i2c_handle * handle)
{
	/* check addressing mode */
	if(handle->bus->conf.AddressingMode == HAL_I2C_ADDRESSINGMODE_10BIT)
	{
		handle->reg->control0 |= UCSLA10; /* set 10bit */
	}
}

static void hal_i2c_set_clock(hal_i2c_handle * handle)
{
	/* SMCLK FREQ constant 1 MHz for F5529 */
	const uint32_t SMCLK_FREQ = 1000000;
	uint16_t preScalar;
	preScalar = (uint16_t)(SMCLK_FREQ/handle->bus->conf.ClockSpeed);

	handle->reg->control1 |= UCSSEL_2 | UCSWRST; /* SMCLK + keep reset */
	handle->reg->baudrate0 = preScalar;
	handle->reg->baudrate1 = 0;
}



hal_i2c_status hal_i2c_master_write_state_machine(hal_i2c_handle * handle, uint16_t addr, uint8_t *ptr, int len)
{
	/* loop variable */
	int i = 0;

	/* clear buffer */
	handle->reg->txBuffer = 0;

	/* reset interrupt flag */
	handle->reg->interruptFlags = 0;

	/* set slave address */
	handle->reg->slaveAddress = addr;

	/* set I2C start condition and UCTR */
	handle->reg->control1 |= UCTR;
	handle->reg->control1 |= UCTXSTT;

	/* transmit mode */
	if (handle->reg->interruptFlags & UCTXIFG)
	{
		if(ptr == 0) /* if bad pointer */
			return I2C_ERROR; /* error */

		/* get data from ptr */
		for (; i < len; i++, ptr++)
		{
			/* write byte to buffer */
			handle->reg->txBuffer = *ptr;

			if(i == 0) /* if first byte */
			{
				/* wait for STT to clear */
				while (handle->reg->control1 & UCTXSTT){};
			}

			/* slave not responding? */
			if (handle->reg->interruptFlags & UCNACKIFG)
			{
				/* trigger a stop condition */
				handle->reg->control1 |= UCTXSTP;
				while (handle->reg->control1 & UCTXSTP); /* stop condition sent? */

				return I2C_ERROR; /* error */
			}
			/* wait until TXIFG to be set */
			while(!(handle->reg->interruptFlags & UCTXIFG));
		}
		/* trigger a stop condition */
		handle->reg->control1 |= UCTXSTP;
		while (handle->reg->control1 & UCTXSTP); /* stop condition sent? */
	}
	else /* something else? */
	{
		/* trigger a stop condition */
		handle->reg->control1 |= UCTXSTP;
		while (handle->reg->control1 & UCTXSTP); /* stop condition sent? */
	}

	/* return success */
	return I2C_OK;

}


hal_i2c_status hal_i2c_master_read_state_machine(hal_i2c_handle * handle, uint16_t addr, uint8_t *ptr, int len)
{
	/* loop variable */
	int i = 0;

	/* clear buffer */
	handle->reg->rxBuffer = 0;
	/* reset interrupt flag */
	handle->reg->interruptFlags = 0;

	/* set slave address */
	handle->reg->slaveAddress = addr;

	/* set I2C start condition and clear UCTR */
	handle->reg->control1 &= ~UCTR;
	handle->reg->control1 |= UCTXSTT;

	/* wait for STT to clear (slave recevied address) */
	while (handle->reg->control1 & UCTXSTT);

	/* if only sending 1 byte */
	// TODO: test 1 byte read
	if(len == 1)
	{
		/* set stop bit immediately */
		handle->reg->control1 |= UCTXSTP;
	}

	/* slave not responding? */
	if (handle->reg->interruptFlags & UCNACKIFG)
	{
		/* if not sent already (len =/= 1) */
		if (handle->reg->control1 & UCTXSTP)
		{
			/* trigger a stop condition */
			handle->reg->control1 |= UCTXSTP;
			/* stop condition sent? */
			while (handle->reg->control1 & UCTXSTP);
		}

		return I2C_ERROR; /* error */
	}

	/* wait until RXIFG to be set to begin receiving data */
	while(!(handle->reg->interruptFlags & UCRXIFG));

	/* receive mode */
	if (handle->reg->interruptFlags & UCRXIFG)
	{
		if(ptr == 0) /* if bad pointer */
			return I2C_ERROR; /* error */

		/* put data in ptr */
		for (; i < len; i++, ptr++)
		{
			/* read from rx reg */
			*ptr = handle->reg->rxBuffer;

			/* if not receiving only one byte */
			if(len != 1)
			{
				/* wait until RXIFG to be set to receiving more data */
				while(!(handle->reg->interruptFlags & UCRXIFG));
			}
		}
		/* trigger a stop condition */
		handle->reg->control1 |= UCTXSTP;
		while (handle->reg->control1 & UCTXSTP); /* stop condition sent? */
	}
	else /* something else? */
	{
		/* trigger a stop condition */
		handle->reg->control1 |= UCTXSTP;
		while (handle->reg->control1 & UCTXSTP); /* stop condition sent? */
	}

	/* return success */
	return I2C_OK;
}
