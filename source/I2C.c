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
#include "kubos-hal-msp430f5529/I2C.h"

#include <msp430.h>

int k_hal_i2c_state_machine(KI2CDevNum i2c, uint8_t *ptr, int len)
{
	/* get i2c struct */
	msp_i2c *msp_i2c = kprv_msp_i2c_get(i2c);
	/* loop variable */
	int i = 0;

	switch(UCB0IV) /* check interrupt register */
	{
    	case USCI_NONE:          break; /* No interrupts */
    	case USCI_I2C_UCALIFG:   break; /* ALIFG */

    	case USCI_I2C_UCNACKIFG: /* NACK */
    		msp_i2c->reg->control1 |= UCTXSTT; /* I2C start condition */
    		break;

    	case USCI_I2C_UCSTTIFG:  break;
    	case USCI_I2C_UCSTPIFG:  break;

    	case USCI_I2C_UCRXIFG: /* receive mode */

    		if(ptr == 0) /* if bad pointer */
    			return -1; /* error */

    		/* get data from ptr */
    		for (; i < len; i++, ptr++){
    			*ptr = msp_i2c->reg->rxBuffer;
    	    }

    		/* trigger a stop condition */
    		msp_i2c->reg->control1 |= UCTXSTP;
    		msp_i2c->k_i2c->status = I2C_DATA_RECEIVED;

    		break;

    	case USCI_I2C_UCTXIFG: /* transmit mode */
    	if(ptr == 0) /* if bad pointer */
    		return -1; /* error */

		/* get data from ptr */
		for (; i < len; i++, ptr++){
			msp_i2c->reg->txBuffer = *ptr;
	    }

    	if(msp_i2c->k_i2c->status == I2C_READ)
    	{
    		/* trigger a repeat start with receiver mode */
    		msp_i2c->reg->control1 &= ~UCTR;
    		msp_i2c->reg->control1 |= UCTXSTT;                    /* I2C start condition */
    	}
    	else /* done transmitting */
    	{
    		/* trigger a stop condition */
    		msp_i2c->reg->control1 |= UCTXSTP;
    		msp_i2c->k_i2c->status = I2C_IDLE;
    	}
    	break;

    	default: break;
	}

	/* done, set to idle */
	msp_i2c->k_i2c->status = I2C_IDLE;

	/* return success */
	return 0;
}
