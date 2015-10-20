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
#include <stdint.h>

#include <kernel.h>
#include <msg.h>
#include <ringbuffer.h>
#include <thread.h>

#include "kiss.h"

#if !KISS_DRIVER_UART
#include <unistd.h>
#endif

#define ENABLE_DEBUG (0)
#include "debug.h"

#define _KISS_MSG_TYPE       (0xd1cc) // randomly chosen
#define _KISS_NAME           "KISS"
#define _KISS_MSG_QUEUE_SIZE (4U)

#define _KISS_DEV(d)         ((kiss_dev_t *) d)

#if ENABLE_DEBUG
static size_t _kiss_sent = 0;
#endif

#if KISS_DRIVER_UART
static void _kiss_rx_cb(void *arg, char data)
{
}

static int _kiss_tx_cb(void *arg)
{
    if (_KISS_DEV(arg)->out_buf.avail > 0) {
        char c = (char)ringbuffer_get_one(&_KISS_DEV(arg)->out_buf);
        uart_write((uart_t)(_KISS_DEV(arg)->uart), c);
#if ENABLE_DEBUG
        _kiss_sent++;
#endif
        return 1;
    }
    return 0;
}
#endif


static inline void _kiss_send_char(kiss_dev_t *dev, char c)
{
#if KISS_DRIVER_UART
    DEBUG("KISS TX[%d]: %02x\n", dev->uart, (uint8_t) c);
    ringbuffer_add_one(&dev->out_buf, c);
    uart_tx_begin(dev->uart);
#else
    DEBUG("KISS TX[%d]: %02x\n", dev->fd, (uint8_t) c);
    write(dev->fd, &c, 1);
#if ENABLE_DEBUG
    _kiss_sent++;
#endif

#endif
}

static inline void _kiss_send_start(kiss_dev_t *dev, uint8_t function,
                                    uint8_t port)
{
    _kiss_send_char(dev, KISS_FEND);
    _kiss_send_char(dev, ((port & 0x0F) << 4) | (function & 0x0F));
}

static void _kiss_send(kiss_dev_t *dev, gnrc_pktsnip_t *pkt)
{
    gnrc_pktsnip_t *ptr = pkt;
    if (pkt->type == GNRC_NETTYPE_NETIF) {
        ptr = pkt->next; // ignore gnrc_netif_hdr_t
    }

#if ENABLE_DEBUG
    size_t total = gnrc_pkt_len(pkt);
    _kiss_sent = 0;
#endif

    _kiss_send_start(dev, KISS_FUNC_DATA, 0);

    while (ptr != NULL) {
        char *data = ptr->data;
        for (size_t i = 0; i < ptr->size; i++) {
            switch (data[i]) {
                case KISS_FEND:
                    _kiss_send_char(dev, KISS_FESC);
                    _kiss_send_char(dev, KISS_TFEND);
                    break;
                case KISS_FESC:
                    _kiss_send_char(dev, KISS_FESC);
                    _kiss_send_char(dev, KISS_TFESC);
                    break;
                default:
                    _kiss_send_char(dev, data[i]);
                    break;
            }
        }
        ptr = ptr->next;
    }

    _kiss_send_char(dev, KISS_FEND);
    gnrc_pktbuf_release(pkt);

    DEBUG("KISS: Sent %d bytes, %d with framing\n", total, _kiss_sent);
}

static void *_kiss_thread(void *args)
{
    kiss_dev_t *dev = _KISS_DEV(args);
    msg_t msg, reply, msg_q[_KISS_MSG_QUEUE_SIZE];

    msg_init_queue(msg_q, _KISS_MSG_QUEUE_SIZE);
    dev->kiss_pid = thread_getpid();
    gnrc_netif_add(dev->kiss_pid);

    while (1) {
        msg_receive(&msg);

        switch (msg.type) {
            case GNRC_NETAPI_MSG_TYPE_SND:
                DEBUG("kiss: GNRC_NETAPI_MSG_TYPE_SND received\n");
                _kiss_send(dev, (gnrc_pktsnip_t *)msg.content.ptr);
                break;

            case GNRC_NETAPI_MSG_TYPE_GET:
            case GNRC_NETAPI_MSG_TYPE_SET:
                reply.type = GNRC_NETAPI_MSG_TYPE_ACK;
                reply.content.value = (uint32_t) (-ENOTSUP);
                msg_reply(&msg, &reply);
                break;
        }
    }

    return NULL;
}

#if KISS_DRIVER_UART

kernel_pid_t kiss_init_uart(kiss_dev_t *dev, uart_t uart, uint32_t baudrate,
                            char *stack, size_t stack_size, char priority)
#else // Native driver

kernel_pid_t kiss_init_native(kiss_dev_t *dev, int fd, char *stack,
                              size_t stack_size, char priority)
#endif
{
    kernel_pid_t pid;

#if KISS_DRIVER_UART
    dev->uart = uart;
#else
    dev->fd = fd;
#endif

    dev->kiss_pid = KERNEL_PID_UNDEF;

    ringbuffer_init(&dev->out_buf, dev->tx_mem, sizeof(dev->tx_mem));

#if KISS_DRIVER_UART
    if (uart_init(uart, baudrate, _kiss_rx_cb, _kiss_tx_cb, dev) < 0) {
        DEBUG("kiss: error initializing UART_%i with baudrate %" PRIu32 "\n",
              uart, baudrate);
        return -ENODEV;
    }
#endif

    DEBUG("starting kiss thread\n");
    pid = thread_create(stack, stack_size, priority, CREATE_STACKTEST,
                        _kiss_thread, dev, "KISS");

    if (pid < 0) {
        DEBUG("slip: unable to create SLIP thread\n");
        return -EFAULT;
    }
    return pid;
}
