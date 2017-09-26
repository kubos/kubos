/*
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

#pragma once

#include <stdint.h>

/**
 * Radio function status
 */
typedef enum {
    RADIO_OK = 0,
    RADIO_ERROR
} KRadioStatus;

/**
 * Structure used to store radio configuration options
 */
typedef struct {
    /**
     * The size of the slave address.
     * Should be either 7-bits long or 10-bits long, as specified by the @ref I2CAddressingMode enumerator
     */
    int addressing_mode;
    /**
     * The role of the I2C bus.
     * Should be either master or slave, as specified by the @ref I2CRole enumerator
     * @warning Only the Master role is available as of v0.1.0
     */
    int role;
    /**
     * The clock speed of the I2C bus
     */
    uint32_t clock_speed;
} KRadioConf;
