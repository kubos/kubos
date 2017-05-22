
/*
 * KubOS RT
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

#include <errno.h>
#include <stdlib.h>
#include <stdio.h>

#include "FreeRTOS.h"
#include "task.h"
#include "timers.h"
#include "queue.h"

#include "kubos-hal/gpio.h"
#include "kubos-hal/uart.h"
#include "kubos-core/modules/klog.h"
#include "kubos-core/modules/sensors/altimeter.h"
#include "kubos-core/modules/sensors/temperature.h"

/**
* Enabling this example code requires certain configuration values to be present
* in the configuration json of this application. An example is given below:
*
*  {
*      "sensors": {
*        "bme280": {
*           "spi bus":"K_SPI1",
*           "CS":"PA4"
*           },
*          "htu21d": {
*              "i2c_bus": "K_I2C1"
*          }
*      }
*  }
*
* This would enable the sensor APIs altimeter and temperature and their related
* sensors the bme280 and the htu21d.
*/

void task_sensors(void *p) {
    /* store sensor values */
    float press, alt, temp, hum;
    /* setup sensor APIs */
    k_initialize_altitude_sensor();
    k_initialize_temperature_sensor();

    while (1) {
        /* get sensor data */
        k_get_altitude(&alt);
        k_get_pressure(&press);
        k_get_temperature(&temp);
        k_get_humidity(&hum);

        /* print out over console */
        printf("pressure - %f\r\n", press);
        printf("altitude - %f\r\n", alt);
        printf("temperature - %f\r\n", temp);
        printf("humidity - %f\r\n", hum);

        /* wait */
        vTaskDelay(100 / portTICK_RATE_MS);
    }
}

int main(void)
{
    k_uart_console_init();

    #ifdef TARGET_LIKE_STM32
    k_gpio_init(K_LED_GREEN, K_GPIO_OUTPUT, K_GPIO_PULL_NONE);
    k_gpio_init(K_LED_ORANGE, K_GPIO_OUTPUT, K_GPIO_PULL_NONE);
    k_gpio_init(K_LED_RED, K_GPIO_OUTPUT, K_GPIO_PULL_NONE);
    k_gpio_init(K_LED_BLUE, K_GPIO_OUTPUT, K_GPIO_PULL_NONE);
    k_gpio_init(K_BUTTON_0, K_GPIO_INPUT, K_GPIO_PULL_NONE);
    #endif

    #ifdef TARGET_LIKE_MSP430
    k_gpio_init(K_LED_GREEN, K_GPIO_OUTPUT, K_GPIO_PULL_NONE);
    k_gpio_init(K_LED_RED, K_GPIO_OUTPUT, K_GPIO_PULL_NONE);
    k_gpio_init(K_BUTTON_0, K_GPIO_INPUT, K_GPIO_PULL_UP);
    /* Stop the watchdog. */
    WDTCTL = WDTPW + WDTHOLD;

    __enable_interrupt();

    P2OUT = BIT1;
    #endif

    xTaskCreate(task_sensors, "SENSORS", configMINIMAL_STACK_SIZE * 5, NULL, 2, NULL);
    vTaskStartScheduler();

    while (1);

    return 0;
}
