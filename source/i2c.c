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

#if (defined YOTTA_CFG_HARDWARE_I2C) && (YOTTA_CFG_HARDWARE_I2C_COUNT > 0)
#include "kubos-hal/i2c.h"
#include "msp430f5529-hal/i2c.h"
#include "FreeRTOS.h"
#include "task.h"
#include <msp430.h>

hal_i2c_handle hal_i2c_buses[YOTTA_CFG_HARDWARE_I2C_COUNT];

/* defines for register timeout mode */
#define SET 0
#define RELEASE 1

/* private functions */
static void hal_i2c_set_addressing(hal_i2c_handle * handle);
static void hal_i2c_set_clock(hal_i2c_handle * handle);
static hal_i2c_status hal_i2c_register_timeout(hal_i2c_handle * handle, uint8_t flag, uint8_t mode);


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
	handle->conf = config;
	return handle;
}

void hal_i2c_dev_terminate(hal_i2c_handle * handle)
{
	handle->reg->control1 |= UCSWRST; /* software reset */
	handle->reg->control0 &= ~(UCMODE_3 | UCSYNC | UCMST); /* clear CTL0 */
	handle->reg->control1 &= ~UCSWRST; /* releasing reset */

	/* de-select pins */
	*(handle->select) &= ~handle->selectVal;

}

void hal_i2c_setup(hal_i2c_handle * handle)
{
	/* configure pins */
	*(handle->select) |= handle->selectVal;

	handle->reg->control1 |= UCSWRST; /* software reset */
	handle->reg->control0 |= UCMODE_3 | UCSYNC | UCMST; /* I2C mode, sync, master */

	hal_i2c_set_addressing(handle);

	hal_i2c_set_clock(handle);

	handle->reg->control1 &= ~UCSWRST; /* enable I2C by releasing reset */
}

static void hal_i2c_set_addressing(hal_i2c_handle * handle)
{
	/* check addressing mode */
	if(handle->conf.addressing_mode == HAL_I2C_ADDRESSINGMODE_10BIT)
	{
		handle->reg->control0 |= UCSLA10; /* set 10bit */
	}
	else /* 7BIT */
	{
		handle->reg->control0 &= ~UCSLA10; /* clear 10bit */
	}
}

static void hal_i2c_set_clock(hal_i2c_handle * handle)
{
	/* SMCLK FREQ constant 1 MHz for F5529 */
	const uint32_t SMCLK_FREQ = 1000000;
	uint8_t preScalar;
	preScalar = (uint8_t)(SMCLK_FREQ/handle->conf.clock_speed);

	handle->reg->control1 |= UCSSEL_2 | UCSWRST; /* SMCLK + keep reset */
	handle->reg->baudrate0 = preScalar;
	handle->reg->baudrate1 = 0;
}

static hal_i2c_status hal_i2c_register_timeout(hal_i2c_handle * handle, uint8_t flag, uint8_t mode)
{
	/* timeout counter */
	int timeout = 10;

	/* set register based on mode */
	if(mode == RELEASE)
	{
		/* while waiting for control register to clear */
		while((handle->reg->control1 & flag) && timeout > 0)
		{
			vTaskDelay(50); /* wait */
			timeout--; /* decrease counter */
		}
	}
	else /* SET */
	{
		/* while waiting for interrupt register to set */
		while(!(handle->reg->interruptFlags & flag) && timeout > 0)
		{
			vTaskDelay(50); /* wait */
			timeout--; /* decrease counter */
		}
	}

	/* if we timed out */
	if(timeout <= 0)
	{
		/* check release flags */
		if(mode == RELEASE)
		{
			switch(flag)
			{
				case(UCTXSTT): return HAL_I2C_ERROR_ADDR_TIMEOUT;
				case(UCTXSTP): return HAL_I2C_ERROR_TIMEOUT;
				default: return HAL_I2C_ERROR_TIMEOUT;
			}
		}
		else /* check set flags */
		{
			switch(flag)
			{
				case(UCTXIFG): return HAL_I2C_ERROR_TXE_TIMEOUT;
				case(UCRXIFG): return HAL_I2C_ERROR_BTF_TIMEOUT;
				default: return HAL_I2C_ERROR_TIMEOUT;
			}
		}
	}

	/* success */
	return I2C_OK;
}

hal_i2c_status hal_i2c_master_write(hal_i2c_handle * handle, uint16_t addr, uint8_t *ptr, int len)
{
	/* if bad pointer */
	if(ptr == NULL || handle == NULL)
		return HAL_I2C_ERROR_NULL_HANDLE; /* error */

	/* loop variable */
	int i = 0;
	/* return variable */
	hal_i2c_status ret = HAL_I2C_ERROR;

	/* clear buffer */
	handle->reg->txBuffer = 0;

	/* reset interrupt flag */
	handle->reg->interruptFlags = 0;

	/* set slave address */
	handle->reg->slaveAddress = addr;

	/* set I2C start condition and UCTR */
	handle->reg->control1 |= UCTR;
	handle->reg->control1 |= UCTXSTT;

	/* transmit mode set */
	if (handle->reg->interruptFlags & UCTXIFG)
	{
		/* get data from ptr */
		for (; i < len; i++, ptr++)
		{
			/* write byte to buffer */
			handle->reg->txBuffer = *ptr;

			if(i == 0) /* if first byte */
			{
				/* wait for STT to clear */
				if((ret = hal_i2c_register_timeout(handle, UCTXSTT, RELEASE)) != HAL_I2C_OK)
				{
					/* trigger a stop condition */
					handle->reg->control1 |= UCTXSTP;
					hal_i2c_register_timeout(handle, UCTXSTP, RELEASE);

					return ret; /* error */
				}
			}

			/* slave not responding to start? */
			if (handle->reg->interruptFlags & UCNACKIFG)
			{
				/* trigger a stop condition */
				handle->reg->control1 |= UCTXSTP;
				hal_i2c_register_timeout(handle, UCTXSTP, RELEASE);

				return HAL_I2C_ERROR_NACK; /* error NACK here */
			}

			/* don't wait on single byte or last byte */
			if( i != (len-1))
			{
				/* wait until TXIFG to be set to transmit more data */
				if((ret = hal_i2c_register_timeout(handle, UCTXIFG, SET)) != HAL_I2C_OK)
				{
					/* trigger a stop condition */
					handle->reg->control1 |= UCTXSTP;
					hal_i2c_register_timeout(handle, UCTXSTP, RELEASE);

					return ret; /* error */
				}
			}
		}
		/* trigger a stop condition */
		handle->reg->control1 |= UCTXSTP;
		/* if something goes wrong here we'll catch it on function exit */
		ret = hal_i2c_register_timeout(handle, UCTXSTP, RELEASE);

	}
	else /* something else? */
	{
		/* trigger a stop condition */
		handle->reg->control1 |= UCTXSTP;
		hal_i2c_register_timeout(handle, UCTXSTP, RELEASE);

		return ret; /* error */
	}

	/* return timeout status */
	return ret;

}


hal_i2c_status hal_i2c_master_read(hal_i2c_handle * handle, uint16_t addr, uint8_t *ptr, int len)
{
	/* if bad pointer */
	if(ptr == NULL || handle == NULL)
		return HAL_I2C_ERROR_NULL_HANDLE; /* error */

	/* loop variable */
	int i = 0;
	/* return variable */
	hal_i2c_status ret = HAL_I2C_ERROR;

	/* clear buffer */
	handle->reg->rxBuffer = 0;
	/* reset interrupt flag */
	handle->reg->interruptFlags = 0;

	/* set slave address */
	handle->reg->slaveAddress = addr;

	/* set I2C start condition and clear UCTR */
	handle->reg->control1 &= ~UCTR;
	handle->reg->control1 |= UCTXSTT;

	/* wait for STT to clear (slave received address) */
	if((ret = hal_i2c_register_timeout(handle, UCTXSTT, RELEASE)) != HAL_I2C_OK)
	{
		/* return error */
		return ret;
	}

	/* slave not responding to start? */
	if (handle->reg->interruptFlags & UCNACKIFG)
	{
		/* trigger a stop condition */
		handle->reg->control1 |= UCTXSTP;
		/* stop condition sent? */
		hal_i2c_register_timeout(handle, UCTXSTP, RELEASE);

		return HAL_I2C_ERROR_NACK; /* error NACK here */
	}

	/* if start OK and only receiving 1 byte */
	// TODO: test 1 byte read
	if(len == 1)
	{
		/* set stop bit immediately after STT is clear; during reception */
		/* msp-slau208 1.3.4.2.2 specification */
		handle->reg->control1 |= UCTXSTP;
	}

	/* check receive mode set */
	if ((ret = hal_i2c_register_timeout(handle, UCRXIFG, SET)) == HAL_I2C_OK)
	{
		/* put data in ptr */
		for (; i < len; i++, ptr++)
		{
			/* read from rx reg */
			*ptr = handle->reg->rxBuffer;

			/* don't wait on single byte or last byte */
			if( i != (len-1))
			{
				/* wait until RXIFG to be set to receiving more data */
				if((ret = hal_i2c_register_timeout(handle, UCRXIFG, SET)) != HAL_I2C_OK)
				{
					/* something went wrong, send stop */
					handle->reg->control1 |= UCTXSTP;
					hal_i2c_register_timeout(handle, UCTXSTP, RELEASE);

					return ret; /* return error */
				}
			}
		}

		/* only set stop bit here if receiving more than 1 byte */
		if(len != 1)
		{
			/* trigger a stop condition */
			handle->reg->control1 |= UCTXSTP;
		}
		/* if something goes wrong here we'll catch it on function exit */
		ret = hal_i2c_register_timeout(handle, UCTXSTP, RELEASE);
	}
	else /* something else? */
	{
		/* only set start bit here if receiving more than 1 byte */
		if(len != 1)
		{
			/* trigger a stop condition */
			handle->reg->control1 |= UCTXSTP;
		}
		/* don't check stop timeout ret, not useful info here */
		hal_i2c_register_timeout(handle, UCTXSTP, RELEASE);

		return ret; /* timeout status */
	}

	/* return timeout status */
	return ret;
}
