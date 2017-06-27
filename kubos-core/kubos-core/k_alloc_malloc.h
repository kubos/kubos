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
 * @addtogroup KUBOS_CORE_ALLOCATOR
 * @{
 */

#ifndef K_ALLOC_MALLOC_H
#define K_ALLOC_MALLOC_H

#include "kubos-core/k_alloc.h"
#include <stdlib.h>

void * malloc_new(size_t size);
void malloc_free(void * ptr);
void * malloc_realloc(void * buff, size_t old_size, size_t new_size);
void k_alloc_malloc_init();

k_alloc_t malloc_alloc;

#define K_BUFFER_NEW_MALLOC(n, d, s) \
        k_buffer_alloc(n, d, s, &malloc_alloc)

#define K_BUFFER_FREE_MALLOC(b) \
        k_buffer_free_new(b, &malloc_alloc)

#define K_BUFFER_REALLOC_MALLOC(b, s) \
        k_buffer_realloc_new(b, s, &malloc_alloc)


#endif

/* @} */
