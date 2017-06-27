/*
 * KubOS SD Example
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
 * Kubos HAL SD Over SPI Example Project
 *
 * This project will:
 *   - Connect to an SD card over SPI
 *   - Write data to a file
 *   - Sync the file system
 *   - Get the status of the file
 *   - Read data from the file
 *   - Verify that the read contents match was was written
 *
 * The SD over SPI feature is turned on and configured with the included
 * config.json file.
 *
 * SPI Bus: K_SPI1
 *
 * MSP430F5529 Launchpad:
 *   MOSI - P3.0
 *   MISO - P3.1
 *   SCK  - P3.2
 *   CS   - P3.7
 *
 * STM32F407 Discovery:
 *   MOSI - PA7
 *   MISO - PA6
 *   SCK  - PA5
 *   CS   - PA4
 *
 * Note: By default, this project is configured to run on the MSP430F5529 target.
 * In order to run on the STM32F407 target, please update the chip select
 * definition in the config.json file to be "PA4" instead of "P37".
 *
 */
#include <stdio.h>

#include "FreeRTOS.h"
#include "task.h"

#include "kubos-core/modules/fatfs/ff.h"
#include "kubos-hal/gpio.h"
#include "kubos-hal/uart.h"

#define BLINK_MS 1000
#define TEST_LEN 3072
#define FILE_NAME "data.txt"

static char write[TEST_LEN];
static char read[TEST_LEN];

static inline void blink(int pin)
{
    k_gpio_write(pin, 1);
    vTaskDelay(BLINK_MS / portTICK_RATE_MS);
    k_gpio_write(pin, 0);
}

void task_sd(void * p)
{
    static int x = 0;
    FATFS      FatFs;
    FIL        Fil;
    UINT   bw;
    uint16_t   ret;
    int        c;
    FILINFO    fno;
    uint8_t    status;

    /* Generate test data to write */
    for (c = 0; c < TEST_LEN; c++)
    {
        write[c] = 'Z';
    }

    /* SD Write and Sync */
    printf("Mount SD\r\n");
    if ((ret = f_mount(&FatFs, "", 1)) != FR_OK)
    {
        printf("mount failed %d\r\n", ret);
        blink(K_LED_RED);
        return;
    }

    printf("Open file\r\n");
    if ((ret = f_open(&Fil, FILE_NAME, FA_WRITE | FA_OPEN_ALWAYS)) != FR_OK)
    {
        printf("Open file failed %d\r\n", ret);
        blink(K_LED_RED);
        f_mount(0, "", 0);
        return;
    }

    printf("Writing to file\r\n");
    if ((ret = f_write(&Fil, write, TEST_LEN, &bw)) != FR_OK)
    {
        printf("File writing failed %d\r\n", ret);
        blink(K_LED_RED);
        f_close(&Fil);
        f_mount(0, "", 0);
        return;
    }

    printf("Wrote %d bytes\r\n", bw);

    printf("Syncing file\r\n");
    if ((ret = f_sync(&Fil)) != FR_OK)
    {
        printf("Sync failed %d\r\n", ret);
        blink(K_LED_RED);
        f_close(&Fil);
        f_mount(0, "", 0);
        return;
    }

    printf("Closing file\r\n");
    if ((ret = f_close(&Fil)) != FR_OK)
    {
        printf("Closing file failed\r\n");
        blink(K_LED_RED);
        f_mount(0, "", 0);
        return;
    }

    printf("Unmounting file\r\n");
    if ((ret = f_mount(0, "", 0)) != FR_OK)
    {
        printf("Unmount failed\r\n");
        blink(K_LED_RED);
        return;
    }
    /* End of SD Write and Sync Code */

    /* SD File Status and Read */
    printf("Mounting SD\r\n");
    if ((ret = f_mount(&FatFs, "", 1)) != FR_OK)
    {
        printf("Mount failed %d\r\n", ret);
        blink(K_LED_RED);
        return;
    }

    printf("Getting file status\r\n");
    if ((ret = f_stat(FILE_NAME, &fno)) != FR_OK)
    {
        printf("Get file status failed %d\r\n", ret);
        blink(K_LED_RED);
        f_mount(0, "", 0);
        return;
    }

    printf("File status: size %ld name %s\r\n", fno.fsize, fno.fname);

    printf("Opening file\r\n");
    if ((ret = f_open(&Fil, FILE_NAME, FA_READ | FA_OPEN_ALWAYS)) != FR_OK)
    {
        printf("Open fail %d\r\n", ret);
        blink(K_LED_RED);
        f_mount(0, "", 0);
        return;
    }

    printf("Reading file\r\n");
    if ((ret = f_read(&Fil, read, TEST_LEN, &bw)) != FR_OK)
    {
        printf("Reading failed %d\r\n", ret);
        blink(K_LED_RED);
        f_close(&Fil);
        f_mount(0, "", 0);
        return;
    }
    printf("Read %d bytes\r\n", bw);

    printf("Closing file\r\n");
    if ((ret = f_close(&Fil)) != FR_OK)
    {
        printf("Closing file failed %d\r\n", ret);
        blink(K_LED_RED);
        f_mount(0, "", 0);
        return;
    }

    printf("Unmounting SD\r\n");
    if ((ret = f_mount(0, "", 0)) != FR_OK)
    {
        printf("Unmount SD failed %d\r\n", ret);
        blink(K_LED_RED);
        return;
    }
    /* End of SD File Status and Read Code */

    printf("Verifying data...\r\n");
    for (c = 0; c < TEST_LEN; c++)
    {
        if (write[c] != read[c])
        {
            printf("Found different at %d:%d - %d\r\n", c, write[c], read[c]);
            blink(K_LED_RED);
            break;
        }
    }

    printf("All done!\r\n");
    blink(K_LED_GREEN);
}

int main(void)
{
    k_uart_console_init();

#ifdef TARGET_LIKE_STM32
    k_gpio_init(K_LED_GREEN, K_GPIO_OUTPUT, K_GPIO_PULL_NONE);
    k_gpio_init(K_LED_RED, K_GPIO_OUTPUT, K_GPIO_PULL_NONE);
#endif

#ifdef TARGET_LIKE_MSP430
    k_gpio_init(K_LED_GREEN, K_GPIO_OUTPUT, K_GPIO_PULL_NONE);
    k_gpio_init(K_LED_RED, K_GPIO_OUTPUT, K_GPIO_PULL_NONE);
    /* Stop the watchdog. */
    WDTCTL = WDTPW + WDTHOLD;

    __enable_interrupt();

    P2OUT = BIT1;
#endif

    xTaskCreate(task_sd, "SD", configMINIMAL_STACK_SIZE * 4, NULL, 2, NULL);
    vTaskStartScheduler();

    while (1);

    return 0;
}
