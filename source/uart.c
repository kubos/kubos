/*
 * KubOS HAL
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

 /**
   * @defgroup UART
   * @addtogroup UART
   * @{
   */
#include "kubos-hal/gpio.h"
#include "kubos-hal/uart.h"
#include "kubos-hal-stm32f407vg/stm32f4_gpio.h"

static inline USART_TypeDef *uart_dev(KUARTNum uart)
{
    switch (uart) {
        case K_UART1: return USART1;
        case K_UART2: return USART2;
        case K_UART3: return USART3;
        case K_UART4: return UART4;
        case K_UART5: return UART5;
        case K_UART6: return USART6;
        default: return NULL;
    }
}

static inline IRQn_Type uart_irqn(KUARTNum uart)
{
    switch (uart) {
        case K_UART1: return USART1_IRQn;
        case K_UART2: return USART2_IRQn;
        case K_UART3: return USART3_IRQn;
        case K_UART4: return UART4_IRQn;
        case K_UART5: return UART5_IRQn;
        case K_UART6: return USART6_IRQn;
        default: return 0;
    }
}

static inline void uart_clk_enable(KUARTNum uart)
{
    switch (uart) {
        case K_UART1: __HAL_RCC_USART1_CLK_ENABLE(); break;
        case K_UART2: __HAL_RCC_USART2_CLK_ENABLE(); break;
        case K_UART3: __HAL_RCC_USART3_CLK_ENABLE(); break;
        case K_UART4: __HAL_RCC_UART4_CLK_ENABLE(); break;
        case K_UART5: __HAL_RCC_UART5_CLK_ENABLE(); break;
        case K_UART6: __HAL_RCC_USART6_CLK_ENABLE(); break;
    }
}

static inline uint8_t uart_alt(KUARTNum uart)
{
    switch (uart) {
        case K_UART1: return GPIO_AF7_USART1;
        case K_UART2: return GPIO_AF7_USART2;
        case K_UART3: return GPIO_AF7_USART3;
        case K_UART4: return GPIO_AF8_UART4;
        case K_UART5: return GPIO_AF8_UART5;
        case K_UART6: return GPIO_AF8_USART6;
        default: return 0;
    }
}

void kprv_uart_dev_init(KUARTNum uart)
{
    // Enable peripheral clocks
    KUART *k_uart = kprv_uart_get(uart);
    int rx = k_uart_rx_pin(uart);
    int tx = k_uart_tx_pin(uart);

    RCC->AHB1ENR |= STM32F4_PIN_AHB1ENR_BIT(rx) | STM32F4_PIN_AHB1ENR_BIT(tx);
    uart_clk_enable(uart);

    // Initialize Serial Port
    //
    HAL_GPIO_Init(STM32F4_PIN_GPIO(tx), &(GPIO_InitTypeDef) {
        .Pin   = STM32F4_PIN_MASK(tx),
        .Speed = GPIO_SPEED_FAST,
        .Mode  = GPIO_MODE_AF_PP,
        .Alternate = uart_alt(uart)
    });

    HAL_GPIO_Init(STM32F4_PIN_GPIO(rx), &(GPIO_InitTypeDef) {
        .Pin  = STM32F4_PIN_MASK(rx),
        .Mode = GPIO_MODE_INPUT,
        .Pull = GPIO_PULLUP
    });

    kprv_gpio_alt_config(STM32F4_PIN_GPIO(rx), STM32F4_PIN_OFFSET(rx),
                         uart_alt(uart));

    HAL_NVIC_SetPriority(uart_irqn(uart), 15, 0);
    HAL_NVIC_EnableIRQ(uart_irqn(uart));

    USART_HandleTypeDef u = {
        .Instance = uart_dev(uart),
        .Init = {
            .BaudRate = k_uart->conf.baud_rate,
            .Mode = USART_MODE_TX_RX
        }
    };

    switch (k_uart->conf.word_len) {
        case K_WORD_LEN_9BIT:
            u.Init.WordLength = USART_WORDLENGTH_9B; break;
        case K_WORD_LEN_8BIT:
        default:
            u.Init.WordLength = USART_WORDLENGTH_8B; break;
    }

    switch (k_uart->conf.stop_bits) {
        case K_STOP_BITS_1_5:
            u.Init.StopBits = USART_STOPBITS_1_5; break;
        case K_STOP_BITS_2:
            u.Init.StopBits = USART_STOPBITS_2; break;
        case K_STOP_BITS_1:
        default:
            u.Init.StopBits = USART_STOPBITS_1; break;
    }

    switch (k_uart->conf.parity) {
        case K_PARITY_EVEN:
            u.Init.Parity = USART_PARITY_EVEN; break;
        case K_PARITY_ODD:
            u.Init.Parity = USART_PARITY_ODD; break;
        case K_PARITY_NONE:
        default:
            u.Init.Parity = USART_PARITY_NONE; break;
    }

    HAL_USART_Init(&u);
    __HAL_USART_ENABLE_IT(&u, USART_IT_RXNE);
}

void kprv_uart_enable_tx_int(KUARTNum uart)
{
    uart_dev(uart)->CR1 |= USART_CR1_TXEIE;
}

void k_uart_write_immediate(KUARTNum uart, char c)
{
    USART_TypeDef *usart = uart_dev(uart);

    usart->DR |= c;
    while (!(usart->SR & USART_FLAG_TXE));
}

void USART6_IRQHandler(void)
{
    static portBASE_TYPE task_woken;
    KUART *k_uart = kprv_uart_get(K_UART6);

    task_woken = pdFALSE;
    if ((USART6->SR & USART_SR_ORE) == USART_SR_ORE) {
        uint32_t tmpreg = USART6->SR;
        tmpreg = USART6->DR;
        ((void)tmpreg);
    }

    if ((USART6->SR & USART_SR_RXNE) == USART_SR_RXNE) {
        char c = USART6->DR;
        xQueueSendToBackFromISR(k_uart->rx_queue, (void *) &c, &task_woken);
        if (task_woken != pdFALSE) {
            portYIELD();
        }
    }

    if ((USART6->CR1 & USART_CR1_TXEIE) == USART_CR1_TXEIE &&
        (USART6->SR & USART_SR_TXE) == USART_SR_TXE) {
        char c;
        task_woken = pdFALSE;
        BaseType_t result = xQueueReceiveFromISR(k_uart->tx_queue,
                                                 (void *) &c,
                                                 &task_woken);
        if (result == pdTRUE) {
            // send a queued byte
            USART6->DR = c;
        } else {
            // nothing to send, disable interrupt
            USART6->CR1 &= ~USART_CR1_TXEIE;
        }

        if (task_woken != pdFALSE) {
            portYIELD();
        }
    }

}


/* @} */
