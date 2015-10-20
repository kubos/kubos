/*
 * KubOS Core Flight Services
 * Copyright (C) 2015 Kubos Corporation
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
#ifndef KISS_H
#define KISS_H

#include <inttypes.h>

#include <net/gnrc.h>
#include <ringbuffer.h>

#if KISS_DRIVER_UART
#include <periph/uart.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif

#ifndef KISS_BUFSIZE
#define KISS_BUFSIZE    (256U)
#endif

#define KISS_FEND           (char)0xC0
#define KISS_FESC           (char)0xDB
#define KISS_TFEND          (char)0xDC
#define KISS_TFESC          (char)0xDD

#define KISS_FUNC_DATA      0x00

typedef struct {
#if KISS_DRIVER_UART
    uart_t uart;
#else // native uses a file descriptor
    int fd;
#endif

    ringbuffer_t out_buf;
    char tx_mem[KISS_BUFSIZE];

    kernel_pid_t kiss_pid;
} kiss_dev_t;


#if KISS_DRIVER_UART
kernel_pid_t kiss_init_uart(kiss_dev_t *dev,
                            uart_t uart,
                            uint32_t baudrate,
                            char *stack,
                            size_t stack_size,
                            char priority);
#else // Native
kernel_pid_t kiss_init_native(kiss_dev_t *dev,
                              int fd,
                              char *stack,
                              size_t stack_size,
                              char priority);
#endif

#ifdef __cplusplus
}
#endif

#endif // KISS_H
