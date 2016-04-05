#include <stdlib.h>
#include <string.h>
#include "kubos-core/arch/k_buffer.h"
#include "kubos-core/arch/k_alloc_malloc.h"
#include "kubos-core/arch/k_alloc_csp.h"
#include "csp/csp_buffer.h"


void k_buffer_init()
{
    k_alloc_malloc_init();
    k_alloc_csp_init();
}

k_buffer_t *k_buffer_alloc(struct k_buffer * next, void * data, size_t size, k_alloc_t * alloc)
{
    k_buffer_t * _buff = alloc->_new(sizeof(k_buffer_t));
    if (NULL == _buff) {
        return NULL;
    }
    void * _data = alloc->_new(size);
    if (NULL == _data) {
        alloc->_free(_buff);
        return NULL;
    }
    _buff->data = _data;
    if (NULL != data) {
        memcpy(_data, data, size);
    }
    _buff->size = size;
    _buff->next = next;
    return _buff;
}

int k_buffer_realloc_new(k_buffer_t * buffer, size_t new_size, k_alloc_t * alloc)
{
    if (new_size == 0)
        return -1;
    if (new_size == buffer->size)
        return 0;
    if (new_size > buffer->size)
    {
        alloc->_realloc(buffer->data, buffer->size, new_size);
        if (NULL == buffer->data) {
            return -1;
        }
    }
    buffer->size = new_size;
    return 0;
}

void k_buffer_free_new(k_buffer_t * buffer, k_alloc_t * alloc)
{
    if (NULL != buffer)
    {
        if (NULL != buffer->data)
        {
            alloc->_free(buffer->data);
        }
        alloc->_free(buffer);
    }
}

k_buffer_t *k_buffer_new(void * data, size_t size)
{
    k_buffer_t * newbuff = malloc(sizeof(k_buffer_t));
    if (NULL == newbuff) {
        return NULL;
    }
    void * _data = csp_buffer_get(size);
    if (NULL == _data) {
        free(newbuff);
        return NULL;
    }
    newbuff->data = _data;
    if (NULL != data) {
        memcpy(_data, data, size);
    }
    newbuff->size = size;
    newbuff->next = NULL;
    return newbuff;
}

k_buffer_t *k_buffer_add(k_buffer_t * next, void * data, size_t size)
{
    k_buffer_t *newbuff = k_buffer_new(data, size);
    if (NULL == newbuff) {
        return NULL;
    }
    newbuff->next = next;
    return newbuff;
}

size_t k_buffer_size(k_buffer_t * buffer)
{
    size_t len = 0;

    while (buffer) {
        len += buffer->size;
        buffer = buffer->next;
    }

    return len;
}

int k_buffer_realloc(k_buffer_t * buffer, size_t new_size)
{
    if (new_size == 0)
        return -1;
    if (new_size == buffer->size)
        return 0;
    if (new_size > buffer->size)
    {
        void * _data = csp_buffer_get(new_size);
        if (NULL == _data)
            return -1;
        memcpy(_data, buffer->data, buffer->size);
        csp_buffer_free(buffer->data);
        buffer->data = _data;
    }
    buffer->size = new_size;
    return 0;
}

void k_buffer_free(k_buffer_t * buffer)
{
    if (NULL != buffer)
    {
        if (NULL != buffer->data)
        {
            csp_buffer_free(buffer->data);
        }
        free(buffer);
    }
}
