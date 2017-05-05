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
/**
 * @addtogroup STM32F4_HAL_UART
 * @{
 */

#if (defined YOTTA_CFG_HARDWARE_UART) && (YOTTA_CFG_HARDWARE_UART_COUNT > 0)
#include "kubos-hal/gpio.h"
#include "kubos-hal/uart.h"
#include "kubos-hal-stm32f4/stm32f4_gpio.h"
#include "stm32cubef4/stm32f4xx_hal_uart.h"

/** Checks if flag is set on UART registers */
#define __GET_FLAG(__HANDLE__, __FLAG__) (((__HANDLE__)->SR & (__FLAG__)) == (__FLAG__))

/**
 * Internal function to get appropriate USART_TypeDef based on UART num
 * @param[in] uart UART bus num
 * @return USART_TypeDef
 */
USART_TypeDef *uart_dev(KUARTNum uart)
{
    switch (uart) {
#ifdef YOTTA_CFG_HARDWARE_UART_UART1
        case K_UART1: return USART1;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART2
        case K_UART2: return USART2;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART3
        case K_UART3: return USART3;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART4
        case K_UART4: return UART4;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART5
        case K_UART5: return UART5;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART6
        case K_UART6: return USART6;
#endif
        default: return NULL;
    }
}

/**
 * Internal function to get appropriate interrupt number based on UART num
 * @param[in] uart UART bus num
 * @return IRQn_Type interrupt number
 */
IRQn_Type uart_irqn(KUARTNum uart)
{
    switch (uart) {
#ifdef YOTTA_CFG_HARDWARE_UART_UART1
        case K_UART1: return USART1_IRQn;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART2
        case K_UART2: return USART2_IRQn;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART3
        case K_UART3: return USART3_IRQn;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART4
        case K_UART4: return UART4_IRQn;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART5
        case K_UART5: return UART5_IRQn;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART6
        case K_UART6: return USART6_IRQn;
#endif
        default: return 0;
    }
}

/**
 * Internal function to enable the correct UART clock based on UART num
 * @param[in] uart UART bus num
 */
static inline void uart_clk_enable(KUARTNum uart)
{
    switch (uart) {
#ifdef YOTTA_CFG_HARDWARE_UART_UART1
        case K_UART1: __HAL_RCC_USART1_CLK_ENABLE(); break;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART2
        case K_UART2: __HAL_RCC_USART2_CLK_ENABLE(); break;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART3
        case K_UART3: __HAL_RCC_USART3_CLK_ENABLE(); break;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART4
        case K_UART4: __HAL_RCC_UART4_CLK_ENABLE(); break;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART5
        case K_UART5: __HAL_RCC_UART5_CLK_ENABLE(); break;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART6
        case K_UART6: __HAL_RCC_USART6_CLK_ENABLE(); break;
#endif
    }
}

/**
 * Internal function to disable the correct UART clock based on UART num
 * @param[in] uart UART bus num
 */
static inline void uart_clk_disable(KUARTNum uart)
{
    switch (uart) {
#ifdef YOTTA_CFG_HARDWARE_UART_UART1
        case K_UART1: __HAL_RCC_USART1_CLK_DISABLE(); break;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART2
        case K_UART2: __HAL_RCC_USART2_CLK_DISABLE(); break;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART3
        case K_UART3: __HAL_RCC_USART3_CLK_DISABLE(); break;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART4
        case K_UART4: __HAL_RCC_UART4_CLK_DISABLE(); break;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART5
        case K_UART5: __HAL_RCC_UART5_CLK_DISABLE(); break;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART6
        case K_UART6: __HAL_RCC_USART6_CLK_DISABLE(); break;
#endif
    }
}

/**
 * Internal function to fetch the alternate UART pin based on UART num
 * @param[in] uart UART bus num
 * @return GPIO pin
 */
static inline uint8_t uart_alt(KUARTNum uart)
{
    switch (uart) {
#ifdef YOTTA_CFG_HARDWARE_UART_UART1
        case K_UART1: return GPIO_AF7_USART1;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART2
        case K_UART2: return GPIO_AF7_USART2;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART3
        case K_UART3: return GPIO_AF7_USART3;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART4
        case K_UART4: return GPIO_AF8_UART4;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART5
        case K_UART5: return GPIO_AF8_UART5;
#endif
#ifdef YOTTA_CFG_HARDWARE_UART_UART6
        case K_UART6: return GPIO_AF8_USART6;
#endif
        default: return 0;
    }
}

/**
 * Setup and enable UART bus
 * @param[in] uart UART bus to initialize
 * @return KUARTStatus UART_OK if success, otherwise failure
 */
KUARTStatus kprv_uart_dev_init(KUARTNum uart)
{
	HAL_StatusTypeDef ret = 0;

    // Enable peripheral clocks
    KUART *k_uart = kprv_uart_get(uart);
    if (k_uart == NULL) {
        return UART_ERROR_NULL_HANDLE;
    }

    int rx = k_uart_rx_pin(uart);
    int tx = k_uart_tx_pin(uart);

    SET_BIT(RCC->AHB1ENR,
            STM32F4_PIN_AHB1ENR_BIT(rx) | STM32F4_PIN_AHB1ENR_BIT(tx));

    uart_clk_enable(uart);

    // Initialize Serial Port
    //
    HAL_GPIO_Init(STM32F4_PIN_GPIO(tx), &(GPIO_InitTypeDef) {
        .Pin   = STM32F4_PIN_MASK(tx),
        .Speed = GPIO_SPEED_HIGH,
        .Pull = GPIO_PULLUP,
        .Mode  = GPIO_MODE_AF_PP,
        .Alternate = uart_alt(uart)
    });

    HAL_GPIO_Init(STM32F4_PIN_GPIO(rx), &(GPIO_InitTypeDef) {
        .Pin  = STM32F4_PIN_MASK(rx),
        .Speed = GPIO_SPEED_HIGH,
        .Pull = GPIO_PULLUP,
        .Mode  = GPIO_MODE_AF_PP,
        .Alternate = uart_alt(uart)
    });

    kprv_gpio_alt_config(STM32F4_PIN_GPIO(rx), STM32F4_PIN_OFFSET(rx),
                         uart_alt(uart));

    HAL_NVIC_SetPriority(uart_irqn(uart), 5, 0);
    HAL_NVIC_EnableIRQ(uart_irqn(uart));

    UART_HandleTypeDef u = {
        .Instance = uart_dev(uart),
        .Init = {
            .BaudRate = k_uart->conf.baud_rate,
            .Mode = UART_MODE_TX | UART_MODE_RX
        }
    };

    switch (k_uart->conf.word_len) {
        case K_WORD_LEN_9BIT:
            u.Init.WordLength = UART_WORDLENGTH_9B;
            break;
        case K_WORD_LEN_8BIT:
        default:
            u.Init.WordLength = UART_WORDLENGTH_8B;
            break;
    }

    switch (k_uart->conf.stop_bits) {
        case K_STOP_BITS_2:
            u.Init.StopBits = UART_STOPBITS_2; break;
        case K_STOP_BITS_1:
        default:
            u.Init.StopBits = UART_STOPBITS_1; break;
    }

    switch (k_uart->conf.parity) {
        case K_PARITY_EVEN:
            u.Init.Parity = UART_PARITY_EVEN; break;
        case K_PARITY_ODD:
            u.Init.Parity = UART_PARITY_ODD; break;
        case K_PARITY_NONE:
        default:
            u.Init.Parity = UART_PARITY_NONE; break;
    }

    u.Init.HwFlowCtl = UART_HWCONTROL_NONE;
    u.Init.OverSampling = UART_OVERSAMPLING_16;

    if((ret = HAL_UART_Init(&u)) != HAL_OK)
    {
    	return ret;
    }

    __HAL_UART_ENABLE_IT(&u, UART_IT_RXNE);

    return ret;
}

/**
 * UART hardware cleanup and disabling
 * @param[in] uart bus num to terminate
 */
void kprv_uart_dev_terminate(KUARTNum uart)
{
    KUART *k_uart = kprv_uart_get(uart);
    if (!k_uart) {
        return;
    }

    int rx = k_uart_rx_pin(uart);
    int tx = k_uart_tx_pin(uart);

    UART_HandleTypeDef u = {
        .Instance = uart_dev(uart),
        .Init = {
            .BaudRate = k_uart->conf.baud_rate,
            .Mode = UART_MODE_TX | UART_MODE_RX
        }
    };


    __HAL_UART_DISABLE_IT(&u, UART_IT_RXNE);
    HAL_UART_DeInit(&u);

    HAL_NVIC_DisableIRQ(uart_irqn(uart));

    HAL_GPIO_DeInit(STM32F4_PIN_GPIO(tx), STM32F4_PIN_MASK(tx));
    HAL_GPIO_DeInit(STM32F4_PIN_GPIO(rx), STM32F4_PIN_MASK(rx));

    uart_clk_disable(uart);

    CLEAR_BIT(RCC->AHB1ENR,
            STM32F4_PIN_AHB1ENR_BIT(rx) | STM32F4_PIN_AHB1ENR_BIT(tx));
}

/**
 * Enable UART tx interrupt
 * @param[in] uart UART bus to initialize
 */
void kprv_uart_enable_tx_int(KUARTNum uart)
{
    USART_TypeDef *dev = uart_dev(uart);
    if (!dev) {
        return;
    }

    SET_BIT(dev->CR1, USART_CR1_TXEIE);
}

/**
 * Write a character directly to the UART interface
 *
 * The function k_uart_write queues up characters in an internal write buffer
 * which is read by the UART interrupt. This function skips the write buffer
 * and writes the character directly to the UART hardware.
 *
 * @param[in] uart UART bus
 * @param[in] c character to write
 * @return KUARTStatus UART_OK if success, otherwise failure
 */
KUARTStatus k_uart_write_immediate(KUARTNum uart, char c)
{
    USART_TypeDef *dev = uart_dev(uart);
    if (dev ==  NULL) {
        return UART_ERROR_NULL_HANDLE;
    }

    dev->DR = c;
    while (!CHECK_BIT(dev->SR, UART_FLAG_TXE));

    return UART_OK;
}

/**
 * Internal function to process triggered interrupt
 * @param[in] uart UART bus num
 */
static inline void uart_irq_handler(KUARTNum uart)
{
    portBASE_TYPE task_woken = pdFALSE;
    USART_TypeDef *dev = uart_dev(uart);
    KUART *k_uart = kprv_uart_get(uart);
    if (!dev || !k_uart) {
        return;
    }

    UART_HandleTypeDef u = { .Instance = uart_dev(uart) };

    HAL_NVIC_DisableIRQ(uart_irqn(uart));

    if (__GET_FLAG(dev, USART_SR_PE))
    {
        // clear out the data register on parity error
        __HAL_UART_CLEAR_PEFLAG(&u);
    }

    if (__GET_FLAG(dev, USART_SR_FE))
    {
        // clear out the data register on framing error
        __HAL_UART_CLEAR_PEFLAG(&u);
    }

    if (__GET_FLAG(dev, USART_SR_NE))
    {
        // clear out the data register on noise error
        __HAL_UART_CLEAR_PEFLAG(&u);
    }

    if (__GET_FLAG(dev, USART_SR_ORE))
    {
        // clear out the data register on overrun
        __HAL_UART_CLEAR_PEFLAG(&u);
    }

    if (__GET_FLAG(dev, USART_SR_RXNE) )
    {
        char c = dev->DR;
        csp_queue_enqueue_isr(k_uart->rx_queue, (void *) &c, &task_woken);
        if (task_woken != pdFALSE) {
            portYIELD();
        }
    }

    if (__GET_FLAG(dev, USART_SR_TXE) )
    {
        char c;
        task_woken = pdFALSE;
        BaseType_t result = csp_queue_dequeue_isr(k_uart->tx_queue,
                                                 (void *) &c,
                                                 &task_woken);
        if (result == CSP_QUEUE_OK) {
            // send a queued byte
            dev->DR = c;
        } else {
            // nothing to send, disable interrupt
            CLEAR_BIT(dev->CR1, USART_CR1_TXEIE);
            CLEAR_BIT(dev->CR1, USART_CR1_TCIE);
        }

        if (task_woken != pdFALSE) {
            portYIELD();
        }
    }

    if (__GET_FLAG(dev, USART_SR_TC))
    {
        CLEAR_BIT(dev->CR1, USART_CR1_TCIE);
    }

    HAL_NVIC_EnableIRQ(uart_irqn(uart));
}

#ifdef YOTTA_CFG_HARDWARE_UART_UART1
/**
 * Specify interrupt routine for uart1
 */
void USART1_IRQHandler(void)
{
    uart_irq_handler(K_UART1);
}
#endif

#ifdef YOTTA_CFG_HARDWARE_UART_UART2
/**
 * Specify interrupt routine for uart2
 */
void USART2_IRQHandler(void)
{
    uart_irq_handler(K_UART2);
}
#endif

#ifdef YOTTA_CFG_HARDWARE_UART_UART3
/**
 * Specify interrupt routine for uart3
 */
void USART3_IRQHandler(void)
{
    uart_irq_handler(K_UART3);
}
#endif

#ifdef YOTTA_CFG_HARDWARE_UART_UART4
/**
 * Specify interrupt routine for uart4
 */
void UART4_IRQHandler(void)
{
    uart_irq_handler(K_UART4);
}
#endif

#ifdef YOTTA_CFG_HARDWARE_UART_UART5
/**
 * Specify interrupt routine for uart5
 */
void UART5_IRQHandler(void)
{
    uart_irq_handler(K_UART5);
}
#endif

#ifdef YOTTA_CFG_HARDWARE_UART_UART6
/**
 * Specify interrupt routine for uart6
 */
void USART6_IRQHandler(void)
{
    uart_irq_handler(K_UART6);
}
#endif
#endif

/* @} */
