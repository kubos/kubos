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

#include <stdlib.h>
#include "kubos-core/arch/k_alloc_csp.h"
#include "csp/csp_buffer.h"
#include <string.h>

void * _csp_new(size_t size)
{
    return csp_buffer_get(size);
}

void * _csp_realloc(void * buff, size_t old_size, size_t new_size)
{
    void * _data = csp_buffer_get(new_size);
    if (NULL == _data)
        return NULL;
    memcpy(_data, buff, old_size);
    return _data;
}

void _csp_free(void * buff)
{
    csp_buffer_free(buff);
}

void k_alloc_csp_init()
{
    csp_buffer_init(CSP_BUFFER_COUNT, CSP_BUFFER_SIZE);

    csp_alloc._new = _csp_new;
    csp_alloc._realloc = _csp_realloc;
    csp_alloc._free = _csp_free;
}
