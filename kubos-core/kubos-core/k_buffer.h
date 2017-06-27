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
  * @defgroup KUBOS_CORE_BUFFER Kubos Core Messaging Buffer Wrapper
  * @addtogroup KUBOS_CORE_BUFFER
  * @{
  */

 /**
  *
  * @file       k_buffer.h
  * @brief      Messaging Buffer Wrapper
  *
  * @author     kubos.co
  */


#ifndef K_BUFFER_H
#define K_BUFFER_H

#include <stdlib.h>
#include "kubos-core/k_alloc.h"
#include "kubos-core/k_alloc_csp.h"
#include "kubos-core/k_alloc_malloc.h"

#define K_ALLOC_MALLOC

typedef struct k_buffer {
    struct k_buffer * next;
    size_t size;
    void * data;
} k_buffer_t;

void k_buffer_init();

k_buffer_t *k_buffer_alloc(struct k_buffer * next, void * data, size_t size, k_alloc_t * alloc);

int k_buffer_realloc_new(k_buffer_t * buffer, size_t new_size, k_alloc_t * alloc);

void k_buffer_free_new(k_buffer_t * buffer, k_alloc_t * alloc);


k_buffer_t *k_buffer_new(void * data, size_t size);

k_buffer_t *k_buffer_add(struct k_buffer* next, void * data, size_t size);

size_t k_buffer_size(k_buffer_t * buffer);

int k_buffer_realloc(k_buffer_t * buffer, size_t new_size);

void k_buffer_free(k_buffer_t * buffer);


#ifdef K_ALLOC_MALLOC
    #define K_BUFFER_NEW(n, d, s) K_BUFFER_NEW_MALLOC(n, d, s)
    #define K_BUFFER_FREE(b) K_BUFFER_FREE_MALLOC(b)
    #define K_BUFFER_REALLOC(b, s) K_BUFFER_REALLOC_MALLOC(b, s)
#elif defined K_ALLOC_CSP
    #define K_BUFFER_NEW(n, d, s) K_BUFFER_NEW_CSP(n, d, s)
    #define K_BUFFER_FREE(b) K_BUFFER_FREE_CSP(b)
    #define K_BUFFER_REALLOC(b, s) K_BUFFER_REALLOC_CSP(b, s)
#else
    #error "k_buffer requires K_ALLOC_XX define."
#endif

#endif

/* @} */
