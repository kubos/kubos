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
 *
 * Global ADCS Structures/Enumerators
 */
/**
 * @addtogroup ADCS
 * @{
 */

#pragma once

/**
 * ADCS function return values
 */
typedef enum {
    ADCS_OK,
    ADCS_ERROR,                  /**< Generic error */
    ADCS_ERROR_CONFIG,           /**< Configuration error */
    ADCS_ERROR_NO_RESPONSE,      /**< No response received from subsystem */
    ADCS_ERROR_INTERNAL,         /**< An error was thrown by the subsystem */
    ADCS_ERROR_MUTEX,            /**< Mutex-related error */
    ADCS_ERROR_NOT_IMPLEMENTED   /**< Requested function has not been implemented for the subsystem */
} KADCSStatus;

/* @} */
