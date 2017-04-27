/*
 * KubOS Shell Example
 * Copyright (C) 2017 Kubos Corporation
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
#define MY_ADDRESS 1
#define MY_PORT    10
#define BLINK_MS   100

#ifdef TARGET_LIKE_MSP430
#define CSP_BUFFERS           YOTTA_CFG_SHELL_EXAMPLE_MSP430_CSP_BUFFERS
#define CSP_BUFFER_SIZE       YOTTA_CFG_SHELL_EXAMPLE_MSP430_CSP_BUFFER_SIZE
#define CSP_ROUTE_STACK_SIZE  YOTTA_CFG_SHELL_EXAMPLE_MSP430_CSP_ROUTE_STACK_SIZE
#define CSP_SERVER_STACK_SIZE YOTTA_CFG_SHELL_EXAMPLE_MSP430_CSP_SERVER_STACK_SIZE
#define SLASH_STACK_SIZE      YOTTA_CFG_SHELL_EXAMPLE_MSP430_SLASH_STACK_SIZE
#define SLASH_LINE_SIZE       YOTTA_CFG_SHELL_EXAMPLE_MSP430_SLASH_LINE_SIZE
#define SLASH_HISTORY_SIZE    YOTTA_CFG_SHELL_EXAMPLE_MSP430_SLASH_HISTORY_SIZE
#else
#define CSP_BUFFERS           YOTTA_CFG_SHELL_EXAMPLE_ARM_CSP_BUFFERS
#define CSP_BUFFER_SIZE       YOTTA_CFG_SHELL_EXAMPLE_ARM_CSP_BUFFER_SIZE
#define CSP_ROUTE_STACK_SIZE  YOTTA_CFG_SHELL_EXAMPLE_ARM_CSP_ROUTE_STACK_SIZE
#define CSP_SERVER_STACK_SIZE YOTTA_CFG_SHELL_EXAMPLE_ARM_CSP_SERVER_STACK_SIZE
#define SLASH_STACK_SIZE      YOTTA_CFG_SHELL_EXAMPLE_ARM_SLASH_STACK_SIZE
#define SLASH_LINE_SIZE       YOTTA_CFG_SHELL_EXAMPLE_ARM_SLASH_LINE_SIZE
#define SLASH_HISTORY_SIZE    YOTTA_CFG_SHELL_EXAMPLE_ARM_SLASH_HISTORY_SIZE
#endif
