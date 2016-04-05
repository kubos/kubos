#ifndef K_ALLOC_MALLOC_H
#define K_ALLOC_MALLOC_H

#include "kubos-core/arch/k_alloc.h"
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
