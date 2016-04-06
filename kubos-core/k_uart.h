#ifndef K_UART_H
#define K_UART_H

#include "csp/drivers/usart.h"

/**
 * @brief   Signature for receive interrupt callback
 *
 * @param[in] arg           context to the callback (optional)
 * @param[in] data          the byte that was received
 */

typedef unsigned int uart_t;

int k_uart_init(struct usart_conf * conf, usart_callback_t rx_cb);

#endif
