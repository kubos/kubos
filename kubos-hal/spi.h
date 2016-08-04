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

#ifndef K_SPI_H
#define K_SPI_H

#include "FreeRTOS.h"
#include "semphr.h"

#ifndef K_NUM_SPI
#define K_NUM_SPI 2
#endif

#define DEFAULT_SPI K_SPI1

typedef enum {
    K_SPI1 = 0,
    K_SPI2
} KSPINum;

typedef enum {
    K_SPI_MASTER = 0,
    K_SPI_SLAVE
} SPIRole;

typedef enum {
    K_SPI_DIRECTION_2LINES = 0,
    K_SPI_DIRECTION_2LINES_RXONLY,
    K_SPI_DIRECTION_1LINE
} SPIDirection;

typedef enum {
    K_SPI_DATASIZE_8BIT = 0,
    K_SPI_DATASIZE_16BIT
} SPIDataSize;

typedef enum {
    SPI_OK,
    SPI_ERROR,
    SPI_ERROR_TIMEOUT
} KSPIStatus;

typedef struct {
    SPIRole role;
    SPIDirection direction;
    SPIDataSize data_size;
    uint32_t speed;
} KSPIConf;

typedef struct {
    KSPIConf config;
    KSPINum bus_num;
    SemaphoreHandle_t spi_lock;
} KSPI;

void k_spi_init(KSPINum spi, KSPIConf * conf);

void k_spi_terminate(KSPINum spi);

KSPIStatus k_spi_write(KSPINum spi, uint8_t * buffer, uint32_t len);

KSPIStatus k_spi_read(KSPINum spi, uint8_t * buffer, uint32_t len);

KSPIStatus k_spi_write_read(KSPINum spi, uint8_t * txBuffer, uint8_t * rxBuffer, uint32_t len);

KSPI * kprv_spi_get(KSPINum spi);

KSPIStatus kprv_spi_write(KSPINum spi, uint8_t * buffer, uint32_t len);

KSPIStatus kprv_spi_read(KSPINum spi, uint8_t * buffer, uint32_t len);

KSPIStatus kprv_spi_write_read(KSPINum spi, uint8_t * txBuffer, uint8_t * rxBuffer, uint32_t len);

#endif
