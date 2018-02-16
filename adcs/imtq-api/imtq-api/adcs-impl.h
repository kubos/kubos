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
/**
 * @defgroup IMTQ_API ISIS iMTQ API
 * @addtogroup IMTQ_API
 * @{
 */

#pragma once

#include "imtq-core.h"
#include "imtq-config.h"
#include "imtq-data.h"
#include "imtq-ops.h"

/**
 *  @name iMTQ config.json configuration options and default values
 */
/**@{*/
/**
 * I2C bus the iMTQ is connected to
 */
#ifdef YOTTA_CFG_ADCS_IMTQ_I2C_BUS
#define IMTQ_I2C_BUS YOTTA_CFG_ADCS_IMTQ_I2C_BUS
#else
#define IMTQ_I2C_BUS K_I2C1
#endif

/**
 * iMTQ I2C address
 */
#ifdef YOTTA_CFG_MTQ_IMTQ_ADDR
#define MTQ_ADDR YOTTA_CFG_MTQ_IMTQ_ADDR
#else
#define IMTQ_ADDR 0x10
#endif

/**
 * Watchdog timeout (in seconds)
 */
#ifdef YOTTA_CFG_ADCS_IMTQ_WATCHDOG_TIMEOUT
#define IMTQ_WD_TIMEOUT YOTTA_CFG_ADCS_IMTQ_WATCHDOG_TIMEOUT
#else
#define IMTQ_WD_TIMEOUT 60
#endif
/**@}*/

typedef struct {
    /* Not an implemented structure/function. Need for compliance with generic API */
} adcs_orient;

typedef struct {
    /* Not an implemented structure/function. Need for compliance with generic API */
} adcs_spin;

/* @} */
