#include <stdlib.h>
#include <string.h>
#include "kubos-core/arch/k_buffer.h"


k_buffer_t *k_buffer_new(void * data, size_t size)
{
    k_buffer_t * newbuff = malloc(sizeof(k_buffer_t));
    if (NULL == newbuff) {
        return NULL;
    }
    void * _data = malloc(sizeof(int) * size);
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

size_t k_buffer_len(k_buffer_t * buffer)
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
        void * _data = malloc(sizeof(int) * new_size);
        memcpy(_data, buffer->data, (buffer->size < new_size) ? buffer->size : new_size);
        free(buffer->data);
        buffer->data = _data;
    }
    buffer->size = new_size;
    return 0;
}
