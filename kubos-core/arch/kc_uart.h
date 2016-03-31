#include "kubos-core/arch/kc_types.h"

/**
 * @brief   Signature for receive interrupt callback
 *
 * @param[in] arg           context to the callback (optional)
 * @param[in] data          the byte that was received
 */
typedef void(*uart_rx_cb_t)(void *arg, uint8_t data);

/**
 * @brief   Initialize a given UART device
 *
 * The UART device will be initialized with the following configuration:
 * - 8 data bits
 * - no parity
 * - 1 stop bit
 * - baudrate as given
 *
 * @param[in] uart          UART device to initialize
 * @param[in] baudrate      desired baudrate in baud/s
 * @param[in] rx_cb         receive callback, executed in interrupt context once
 *                          for every byte that is received (RX buffer filled)
 * @param[in] arg           optional context passed to the callback functions
 *
 * @return                  0 on success
 * @return                  -1 on invalid UART device
 * @return                  -2 on inapplicable baudrate
 * @return                  -3 on other errors
 */
int uart_init(uart_t uart, uint32_t baudrate, uart_rx_cb_t rx_cb, void *arg);
