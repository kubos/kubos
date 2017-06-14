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
 *
 * Kubos Sensor Example Project
 *
 * This application provides a streamlined approach to using the BME280
 * humidity/pressure sensor and the HTU21D temperature/humidity sensor via
 * the Kubos Core sensors API.
 *
 * NOTE: This project is intended for the STM32F407 target only.
 * The MSP430 does not currently have support for floating point
 * variables, so this example project will compile but not successfully run
 * on the MSP430 target.
 *
 * I2C bus: K_I2C1
 *   SDA - PB11
 *   SCL - PB10
 *
 * SPI bus: K_SPI1
 *   SDI - PA7
 *   SDO - PA6
 *   SCK - PA5
 *   CS  - PA4
 *
 * A config.json file has been included with this project which enables the
 * sensor APIs (altimeter and temperature) and the related sensors (the bme280
 * and the htu21d).
 */

#include <stdio.h>

#include "FreeRTOS.h"
#include "task.h"

#include "kubos-core/modules/sensors/altimeter.h"
#include "kubos-core/modules/sensors/temperature.h"
#include "kubos-hal/uart.h"

void task_sensors(void * p)
{
    /* store sensor values */
    float press, alt, temp, hum;
    /* setup sensor APIs */
    k_initialize_altitude_sensor();
    k_initialize_temperature_sensor();

    while (1)
    {
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

#ifdef TARGET_LIKE_MSP430
    /* Stop the watchdog. */
    WDTCTL = WDTPW + WDTHOLD;

    __enable_interrupt();

    P2OUT = BIT1;
#endif

    xTaskCreate(task_sensors, "SENSORS", configMINIMAL_STACK_SIZE * 5, NULL,
                2, NULL);
    vTaskStartScheduler();

    while (1);

    return 0;
}
