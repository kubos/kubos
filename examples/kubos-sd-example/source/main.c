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
 */
#include <errno.h>
#include <stdlib.h>
#include <stdio.h>
#include <csp/csp.h>
#include <csp/arch/csp_thread.h>
#include "FreeRTOS.h"
#include "task.h"
#include "timers.h"
#include "queue.h"
#include "kubos-hal/gpio.h"
#include "kubos-hal/uart.h"
#include "kubos-core/modules/fatfs/ff.h"

#define BLINK_MS 1000
#define TEST_LEN 3072
#define FILE_NAME "data.txt"
static char write[TEST_LEN];
static char read[TEST_LEN];

static inline void blink(int pin) {
    k_gpio_write(pin, 1);
    vTaskDelay(BLINK_MS / portTICK_RATE_MS);
    k_gpio_write(pin, 0);
}

void task_sd(void *p) {
    static int x = 0;
    FATFS FatFs;
    FIL Fil;
    uint16_t bw;
    uint16_t ret;
    int c;
    FILINFO fno;

    for (c = 0; c < TEST_LEN; c++)
    {
        write[c] = 'Z';
    }

    printf("mount sd\r\n");
    if ((ret = f_mount(&FatFs, "", 1)) != FR_OK)
    {
        printf("mount failed %d\r\n", ret);
        return;
    }

    printf("open file\r\n");
    if ((ret = f_open(&Fil, FILE_NAME, FA_WRITE | FA_OPEN_ALWAYS)) != FR_OK)
    {
        printf("open file failed %d\r\n", ret);
        return;
    }

    printf("writing to file\r\n");
    if ((ret = f_write(&Fil, write, TEST_LEN, &bw)) != FR_OK)
    {
        printf("file writing failed %d\r\n", ret);
        return;
    }
    printf("wrote %d bytes\r\n", bw);

    printf("syncing file\r\n");
    if ((ret = f_sync(&Fil)) != FR_OK)
    {
        printf("sync failed %d\r\n", ret);
        return;
    }

    printf("closing file\r\n");
    if ((ret = f_close(&Fil)) != FR_OK)
    {
        printf("closing file failed\r\n");
        return;
    }

    printf("unmounting file\r\n");
    if ((ret = f_mount(0, "", 0)) != FR_OK)
    {
        printf("unmount failed\r\n");
        return;
    }

    printf("mounting sd\r\n");
    if ((ret = f_mount(&FatFs, "", 1)) != FR_OK)
    {
        printf("mount failed %d\r\n", ret);
        return;
    }

    printf("stating file\r\n");
    if ((ret = f_stat(FILE_NAME, &fno)) != FR_OK)
    {
        printf("stat failed %d\r\n", ret);
        return;
    }
    printf("file stat size %ld name %s\r\n", fno.fsize, fno.fname);

    printf("opening file\r\n");
    if ((ret = f_open(&Fil, FILE_NAME, FA_READ | FA_OPEN_ALWAYS)) != FR_OK)
    {
        printf("open fail %d\r\n", ret);
        return;
    }

    printf("reading file\r\n");
    if ((ret = f_read(&Fil, read, TEST_LEN, &bw)) != FR_OK)
    {
        printf("reading failed %d\r\n", ret);
        return;
    }
    printf("read %d bytes\r\n", bw);

    printf("closing file\r\n");
    if ((ret = f_close(&Fil)) != FR_OK)
    {
        printf("closing file failed %d\r\n", ret);
    }

    printf("unmounting sd\r\n");
    if ((ret = f_mount(0, "", 0)) != FR_OK)
    {
        printf("unmounting sd failed %d\r\n", ret);
    }

    printf("Verifying data...\r\n");
    for (c = 0; c < TEST_LEN; c++)
    {
        if (write[c] != read[c])
        {
            printf("Found different at %d:%d - %d\r\n", c, write[c], read[c]);
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

    xTaskCreate(task_sd, "SD", configMINIMAL_STACK_SIZE * 4, NULL, 2, NULL);
    vTaskStartScheduler();

    while (1);

    return 0;
}
