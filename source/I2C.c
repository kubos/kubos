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

int kprv_hal_i2c_state_machine(KI2CDevNum i2c, uint8_t *ptr, int len)
{
	/* get i2c struct */
	msp_i2c *msp_i2c = kprv_msp_i2c_get(i2c);
	/* loop variable */
	int i = 0;

	/* receive mode */
	if (msp_i2c->reg->interruptFlags & UCRXIFG)
	{
		if(ptr == 0) /* if bad pointer */
			return -1; /* error */

		/* put data in ptr */
		for (; i < len; i++, ptr++)
		{
			/* read from rx reg */
			*ptr = msp_i2c->reg->rxBuffer;

			/* if not receiving only one byte */
			if(len != 1)
			{
				/* wait until RXIFG to be set to receiving more data */
				while(!(msp_i2c->reg->interruptFlags & UCRXIFG));
			}
		}

		/* trigger a stop condition */
		msp_i2c->reg->control1 |= UCTXSTP;
		while (msp_i2c->reg->control1 & UCTXSTP); /* stop condition sent? */

		msp_i2c->k_i2c->status = I2C_DATA_RECEIVED;

	}
	/* transmit mode */
	else if (msp_i2c->reg->interruptFlags & UCTXIFG)
	{
		if(ptr == 0) /* if bad pointer */
			return -1; /* error */

		/* get data from ptr */
		for (; i < len; i++, ptr++)
		{
			/* write byte to buffer */
			msp_i2c->reg->txBuffer = *ptr;

			if(i == 0) /* if first byte */
			{
				/* wait for STT to clear */
				while (msp_i2c->reg->control1 & UCTXSTT){};
			}

			/* slave not responding? */
			if (msp_i2c->reg->interruptFlags & UCNACKIFG)
			{
				/* trigger a stop condition */
				msp_i2c->reg->control1 |= UCTXSTP;
				while (msp_i2c->reg->control1 & UCTXSTP); /* stop condition sent? */

				msp_i2c->k_i2c->status = I2C_IDLE;
				return -1; /* error */
			}

			/* wait until TXIFG to be set */
			while(!(msp_i2c->reg->interruptFlags & UCTXIFG));
		}

		/* trigger a stop condition */
		msp_i2c->reg->control1 |= UCTXSTP;
		while (msp_i2c->reg->control1 & UCTXSTP); /* stop condition sent? */

		msp_i2c->k_i2c->status = I2C_IDLE;

	}
	else /* something else? */
	{
		/* trigger a stop condition */
		msp_i2c->reg->control1 |= UCTXSTP;
		while (msp_i2c->reg->control1 & UCTXSTP); /* stop condition sent? */

		msp_i2c->k_i2c->status = I2C_IDLE;
	}

	/* return success */
	return 0;
}
