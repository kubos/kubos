#include "kubos-core/arch/k_alloc_malloc.h"

void * malloc_new(size_t size)
{
    return malloc(size);
}

void malloc_free(void * ptr)
{
    if (NULL != ptr)
        free(ptr);
}

void * malloc_realloc(void * buff, size_t old_size, size_t new_size)
{
    return realloc(buff, new_size);
}

void k_alloc_malloc_init()
{
    malloc_alloc._new = malloc_new;
    malloc_alloc._free = malloc_free;
    malloc_alloc._realloc = malloc_realloc;
}
