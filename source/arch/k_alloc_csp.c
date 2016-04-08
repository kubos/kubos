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
