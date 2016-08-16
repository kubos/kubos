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
/**
 * @brief This file contains implementation of weak functions used by the
 * STM32Cubef4 HAL and which are not driver specific.
 * @author kubos.co
 */

#include <stm32cubef4/stm32f4xx_hal.h>
#include <FreeRTOS.h>
#include <task.h>

/**
 * Return FreeRTOS's tick count
 */
uint32_t HAL_GetTick(void)
{
    return xTaskGetTickCount();
}

/**
 * Gives FreeRTOS time to increment the tick
 */
void HAL_IncTick(void)
{
    vTaskDelay(1);
}

void HAL_Delay(uint32_t Delay)
{
    vTaskDelay(Delay);
}
