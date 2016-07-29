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
#ifdef YOTTA_CFG_HARDWARE_SDIO

#include "kubos-hal/sdio.h"
#include "stm32cubef4/stm32f4xx_hal.h"
#include "stm32cubef4/stm32f4xx_hal_sd.h"
#include "kubos-hal/gpio.h"

static SD_HandleTypeDef sd_handle;

static void sdio_msp_init(void);

KSDIOStatus kprv_sdio_init()
{
    uint8_t ret = SDIO_OK;

    if (sd_handle.Instance != NULL)
    {
        return SDIO_OK;
    }
    HAL_SD_CardInfoTypedef card_info;

    sdio_msp_init();

    sd_handle.Instance = SDIO;
    sd_handle.Init.ClockEdge           = SDIO_CLOCK_EDGE_RISING;
    sd_handle.Init.ClockBypass         = SDIO_CLOCK_BYPASS_DISABLE;
    sd_handle.Init.ClockPowerSave      = SDIO_CLOCK_POWER_SAVE_DISABLE;
    sd_handle.Init.BusWide             = SDIO_BUS_WIDE_1B;
    sd_handle.Init.HardwareFlowControl = SDIO_HARDWARE_FLOW_CONTROL_DISABLE;
    sd_handle.Init.ClockDiv            = SDIO_TRANSFER_CLK_DIV;

    for (int tries = 10; tries > 0; tries--)
    {
        if ((ret = HAL_SD_Init(&sd_handle, &card_info)) == SD_OK)
        {
            break;
        }
        vTaskDelay(5);
    }
    if (ret != SD_OK)
    {
        return SDIO_INIT_ERROR;
    }

    // configure the SD bus width for wide operation
    if (HAL_SD_WideBusOperation_Config(&sd_handle, SDIO_BUS_WIDE_4B) != SD_OK) {
        HAL_SD_DeInit(&sd_handle);
        return SDIO_INIT_ERROR;
    }

    return SDIO_OK;
}

void kprv_sdio_terminate()
{
    HAL_SD_DeInit(&sd_handle);
}

KSDIOStatus kprv_sdio_write_blocks(uint32_t * buffer, uint64_t addr, uint32_t block_size, uint32_t count)
{
    if ((sd_handle.Instance != NULL) &&
        (HAL_SD_WriteBlocks(&sd_handle, buffer, addr, block_size, count) == SD_OK))
    {
        return SDIO_OK;
    }
    return SDIO_WRITE_ERROR;
}

KSDIOStatus kprv_sdio_read_blocks(uint32_t * buffer, uint64_t addr, uint32_t block_size, uint32_t count)
{
    if ((sd_handle.Instance != NULL) &&
        (HAL_SD_ReadBlocks(&sd_handle, buffer, addr, block_size, count) == SD_OK))
    {
        return SDIO_OK;
    }
    return SDIO_READ_ERROR;
}

KSDIOStatus kprv_sdio_card_status()
{
    HAL_SD_CardStatusTypedef card_status;
    if ((sd_handle.Instance != NULL) && (HAL_SD_GetCardStatus(&sd_handle, &card_status) == SD_OK))
    {
        return SDIO_OK;
    }
    return SDIO_ERROR;
}

k_sdio_card_info_t kprv_sdio_card_info()
{
    k_sdio_card_info_t card_info;
    HAL_SD_CardInfoTypedef hal_card_info;

    HAL_SD_Get_CardInfo(&sd_handle, &card_info);
    // We can add more into this struct later, as needed
    card_info.capacity = hal_card_info.CardCapacity;

    return card_info;
}

static void sdio_msp_init(void)
{
    GPIO_InitTypeDef GPIO_Init_Structure;
    
	SET_BIT(RCC->AHB1ENR,
		STM32F4_PIN_AHB1ENR_BIT(PC6) | STM32F4_PIN_AHB1ENR_BIT(PC7));

    /* Enable SDIO clock */
    __HAL_RCC_SDIO_CLK_ENABLE();
    __SDIO_CLK_ENABLE();


    __GPIOC_CLK_ENABLE();
    __GPIOD_CLK_ENABLE();
    __GPIOA_CLK_ENABLE();
    __GPIOB_CLK_ENABLE();

    GPIO_InitTypeDef GPIO_InitStruct;

    // Detect pin
    // PC7 -> DETECT
    GPIO_InitStruct.Pin = GPIO_PIN_7;
    GPIO_InitStruct.Mode = GPIO_MODE_INPUT;
    GPIO_InitStruct.Pull = GPIO_PULLUP;
    GPIO_InitStruct.Speed = GPIO_SPEED_FAST;
    GPIO_InitStruct.Alternate = GPIO_AF12_SDIO;
    HAL_GPIO_Init(GPIOC, &GPIO_InitStruct);


    // 4 pins for 4-bit
    // PC8  -> D0
    // PC9  -> D1
    // PC10 -> D2
    // PC11 -> D3
	// PC12 -> CLK
    GPIO_Init_Structure.Mode = GPIO_MODE_AF_PP;
    GPIO_Init_Structure.Pull = GPIO_PULLUP;
    GPIO_Init_Structure.Speed = GPIO_SPEED_HIGH;
    GPIO_Init_Structure.Alternate = GPIO_AF12_SDIO;
    GPIO_Init_Structure.Pin = GPIO_PIN_6 | GPIO_PIN_7 | GPIO_PIN_8 | GPIO_PIN_9 | GPIO_PIN_10 | GPIO_PIN_12;
    HAL_GPIO_Init(GPIOC, &GPIO_Init_Structure);

    GPIO_Init_Structure.Pin = GPIO_PIN_11;
    GPIO_Init_Structure.Pull = GPIO_NOPULL;
    HAL_GPIO_Init(GPIOC, &GPIO_Init_Structure);


    // PD2 -> CMD
    GPIO_Init_Structure.Pin = GPIO_PIN_2;
    GPIO_Init_Structure.Pull = GPIO_PULLUP;
    HAL_GPIO_Init(GPIOD, &GPIO_Init_Structure);

    // PB8 -> D4
    // PB9 -> D5
    GPIO_Init_Structure.Pin = GPIO_PIN_8 | GPIO_PIN_9;
    HAL_GPIO_Init(GPIOB, &GPIO_Init_Structure);


	// Detect pin
    // PA8 -> DETECT
    GPIO_Init_Structure.Pin = GPIO_PIN_8;
    GPIO_Init_Structure.Mode = GPIO_MODE_INPUT;
    HAL_GPIO_Init(GPIOA, &GPIO_Init_Structure);



	/* NVIC configuration for SDIO interrupts */
	HAL_NVIC_SetPriority(SDIO_IRQn, 5, 0);
	HAL_NVIC_EnableIRQ(SDIO_IRQn);
}


#endif
