/*
 * Copyright (C) 2018 Kubos Corporation
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
 *
 * Global Antenna Structures/Enumerators
 */
/**
 * @addtogroup Antenna
 * @{
 */

#pragma once

/**
 * Antenna function return values
 */
typedef enum {
    ANTS_OK,                     /**< Requested function completed succesfully */
    ANTS_ERROR,                  /**< Generic error */
    ANTS_ERROR_CONFIG,           /**< Configuration error */
    ANTS_ERROR_NOT_IMPLEMENTED   /**< Requested function has not been implemented for the subsystem */
} KANTSStatus;

/* @} */
