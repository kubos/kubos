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
 * Kubos SPI Example Project
 *
 * This application gives two examples of how to interact with SPI devices in
 * a Kubos project:
 *
 * 1. If no sensor has been defined in the project’s config.json file, then
 *    this application will initialize a generic SPI connection over SPI
 *    bus 1.
 *    The application will then enter a loop and attempt to send and receive a
 *    dummy byte.
 *
 *    Note: This case is not a complete example, because it omits the manual
 *    manipulation of a chip select pin that is required for SPI
 *    communication.
 *
 * 2. If the BME280 sensor has been defined in the project’s config.json file,
 *    the appropriate initialization calls will be made and then the
 *    application will enter a loop.
 *
 *    In each iteration of the loop, the application will fetch and print the
 *    current temperature and humidity data from the sensor to the default
 *    UART port.
 *
 * Support and configuration for the sensor is established with the included
 * config.json file.
 *
 * NOTE: This project is intended for the STM32F407 target only.
 * The MSP430 does not currently have support for floating point variables, so
 * this example project will compile but not successfully run on the MSP430
 * target.
 *
 * SPI bus: K_SPI1
 *   SDI - PA7
 *   SDO - PA6
 *   SCK - PA5
 *   CS  - PA4
 */

#include <stdio.h>

#include "FreeRTOS.h"
#include "task.h"

#include "kubos-hal/spi.h"
#include "kubos-hal/uart.h"

#ifdef YOTTA_CFG_SENSORS_BME280
#include "kubos-core/modules/sensors/bme280.h"
#endif

void task_spi(void * p)
{

#ifdef YOTTA_CFG_SENSORS_BME280
    /* store sensor values */
    float temp, hum;
    /* setup sensor spi interface */
    bme280_setup();
#else
/* define own bus */
#define SPI_BUS K_SPI1
    /* data to send */
    uint8_t tx = 1;
    /* data to receive */
    volatile uint8_t rx = 0;
    /* create own config */
    KSPIConf conf = {.role      = K_SPI_MASTER,
                     .direction = K_SPI_DIRECTION_2LINES,
                     .data_size = K_SPI_DATASIZE_8BIT,
                     .speed     = 10000 };
    /* Initialize spi bus with configuration */
    k_spi_init(SPI_BUS, &conf);
#endif

    while (1)
    {
#ifdef YOTTA_CFG_SENSORS_BME280
        /* get sensor data */
        bme280_read_temperature(&temp);
        bme280_read_humidity(&hum);
        /* print out over console */
        printf("temp - %f\r\n", temp);
        printf("humidity - %f\r\n", hum);
#else
        /* Send single byte over spi and then receive it */
        k_spi_write_read(SPI_BUS, &tx, &rx, 1);
        /* print out over console */
        printf("rx - %f\r\n", rx);
        /* increment tx */
        tx++;
#endif
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

    xTaskCreate(task_spi, "SPI", configMINIMAL_STACK_SIZE * 5, NULL, 2, NULL);

    vTaskStartScheduler();

    while (1);

    return 0;
}
