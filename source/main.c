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

#include <csp/csp.h>
#include <csp/arch/csp_thread.h>

#define MY_ADDRESS 1
#define MY_PORT    10
#define BLINK_MS 100

static xQueueHandle button_queue;

#undef putchar

int putchar(int c) {
    int tmp = c;
    tmp++;
    tmp++;
    return 1;
}


size_t strnlen(const char * str, size_t size) {
    return size;
}
//
// static inline void blink(int pin) {
//     k_gpio_write(pin, 1);
//     vTaskDelay(BLINK_MS / portTICK_RATE_MS);
//     k_gpio_write(pin, 0);
// }
//
// static inline void blink_green() {
//     P4OUT ^= (BIT7);
//     vTaskDelay(BLINK_MS / portTICK_RATE_MS);
//     P4OUT ^= (BIT7);
// }
//
// static inline void blink_red() {
//     P1OUT ^= (BIT0);
//     vTaskDelay(BLINK_MS / portTICK_RATE_MS);
//     P1OUT ^= (BIT0);
// }

void red(void *p)
{
    /* Create socket without any socket options */
    csp_socket_t *sock = csp_socket(CSP_SO_NONE);

    /* Bind all ports to socket */
    csp_bind(sock, CSP_ANY);

    /* Create 10 connections backlog queue */
    csp_listen(sock, 3);

    /* Pointer to current connection and packet */
    csp_conn_t *conn;
    csp_packet_t *packet;
    //
    // while (1)
    // {
    //     P1OUT ^= (BIT0); // Invert P1.0
    //     vTaskDelay(100);
    //     // P4OUT ^= (BIT7);
    //     // vTaskDelay(100);
    // }



    // while (1)
    // {
    //     P1OUT ^= (BIT0); // Invert P1.0
    //     vTaskDelay(100); //__delay_cycles(250000); // a ~250000uS pause
    // }


    while (1)
    {

        //blink_red();
        /* Wait for connection, 10000 ms timeout */
        if ((conn = csp_accept(sock, 100)) == NULL)
        {
            continue;
        }

        /* Read packets. Timout is 100 ms */
        while ((packet = csp_read(conn, 100)) != NULL) {
            switch (csp_conn_dport(conn)) {
                case MY_PORT:
                    /* Process packet here */
                    P1OUT ^= (BIT0); // Invert P1.0
                    vTaskDelay(100);
                    csp_buffer_free(packet);
                    break;

                default:
                    /* Let the service handler reply pings, buffer use, etc. */
                    P1OUT ^= (BIT0); // Invert P1.0
                    vTaskDelay(100);
                    csp_service_handler(conn, packet);
                    break;
            }
        }

        /* Close current connection, and handle next */
        csp_close(conn);
    }
}

void green(void *p)
{

    csp_packet_t * packet;
    csp_conn_t * conn;
    portBASE_TYPE status;
    int signal;

    // while (1)
    // {
    //     // P1OUT ^= (BIT0); // Invert P1.0
    //     // vTaskDelay(100);
    //     P4OUT ^= (BIT7);
    //     vTaskDelay(170);
    // }

    /**
     * Try ping
     */

    // csp_sleep_ms(200);
    //
    //
    // int result = csp_ping(MY_ADDRESS, 100, 100, CSP_O_NONE);
    // if (result) {
    //     P4OUT ^= (BIT7); // Invert P4.7
    //     vTaskDelay(100);
    // }


    // while (1)
    // {
    //     P4OUT ^= (BIT7); // Invert P4.7
    //     vTaskDelay(100);
    // }



    while(1)
    {

        P4OUT ^= (BIT7); // Invert P4.7
        //k_gpio_write(K_LED_GREEN, 1);
        vTaskDelay(500);
        P4OUT ^= (BIT7); // Invert P4.7
        //k_gpio_write(K_LED_GREEN, 0);
        vTaskDelay(500);

        /* Get packet buffer for data */
        packet = csp_buffer_get(10);
        if (packet == NULL) {
            /* Could not get buffer element */
            continue;
        }

        /* Connect to host HOST, port PORT with regular UDP-like protocol and 1000 ms timeout */
        conn = csp_connect(CSP_PRIO_NORM, MY_ADDRESS, MY_PORT, 1000, CSP_O_NONE);
        if (conn == NULL) {
            /* Connect failed */
            //blink_red();
            /* Remember to free packet buffer */
            csp_buffer_free(packet);
            continue;
        }

        /* Copy dummy data to packet */
        char *msg = "Hello World";
        strcpy((char *) packet->data, msg);

        /* Set packet length */
        packet->length = strlen(msg);

        /* Send packet */
        if (!csp_send(conn, packet, 1000)) {
            //blink_red();
            /* Send failed */
            csp_buffer_free(packet);
        }

        /* Close connection */
        csp_close(conn);
    }
}

int main(void)
{
    // k_gpio_init(K_LED_RED, K_GPIO_OUTPUT, K_GPIO_PULL_NONE);
    // k_gpio_init(K_LED_GREEN, K_GPIO_OUTPUT, K_GPIO_PULL_NONE);
    // k_gpio_init(K_BUTTON_0, K_GPIO_INPUT, K_GPIO_PULL_NONE);


    //taskDISABLE_INTERRUPTS();

    /* Stop the watchdog. */
    WDTCTL = WDTPW + WDTHOLD;

    unsigned int i = 0;

    //k_gpio_init(K_LED_GREEN, K_GPIO_OUTPUT, K_GPIO_PULL_NONE);

    // Configure LEDs
    P1DIR |= (BIT0); // P1.0 as output (RED)
    P4DIR |= (BIT7); // P4.7 as output (GREEN)

    // Configure button P2.1run
    P2REN |= BIT1;
    P2OUT |= BIT1;

    // Start LEDs off
    P1OUT = 0;
    P4OUT = 0;




    button_queue = xQueueCreate(10, sizeof(int));

    csp_buffer_init(5, 100);
    csp_init(MY_ADDRESS);
    csp_route_start_task(500, 1);


    xTaskCreate(red, "red", configMINIMAL_STACK_SIZE, NULL, 1, NULL);
    xTaskCreate(green, "green", configMINIMAL_STACK_SIZE, NULL, 1, NULL);


    vTaskStartScheduler();

    while (1);

    return 0;
}

int __wrap_main() {
    return main();
}
