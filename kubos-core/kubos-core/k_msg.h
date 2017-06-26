/*
 * KubOS Core Flight Services
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
  * @defgroup KUBOS_CORE_MESSAGE Kubos Core Messaging Interface
  * @addtogroup KUBOS_CORE_MESSAGE
  * @{
  */

#ifndef K_MSG_H
#define K_MSG_H

#include <stdint.h>
#include "csp/csp.h"

typedef struct k_msg {
    uint16_t type;
    char * content;
} k_msg_t;

int k_msg_send(k_msg_t * m, csp_conn_t * conn);

#endif

/* @} */
