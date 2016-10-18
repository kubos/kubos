/*
 * KubOS HAL
 * Copyright (C) 2016 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
`* you may not use this file except in compliance with the License.
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

#if (defined YOTTA_CFG_HARDWARE_UART) && (YOTTA_CFG_HARDWARE_UART_COUNT > 0)

#include "kubos-hal/uart.h"

int kprv_uart_dev_init(KUARTNum uart)
{
    // init stuff
    return 0;
}

void kprv_uart_dev_terminate(KUARTNum uart)
{
    // do more stufff
}

void kprv_uart_enable_tx_int(KUARTNum uart)
{
    char data;
    int result;
    KUART * k_uart = kprv_uart_get(uart);

    if (k_uart != NULL)
    {
        // Simulate UART loopback
        while(csp_queue_dequeue(k_uart->tx_queue, &data, 0) == CSP_QUEUE_OK)
        {
            csp_queue_enqueue(k_uart->rx_queue, &data, CSP_MAX_DELAY);
        }
    }
}

#endif
