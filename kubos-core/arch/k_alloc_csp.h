#ifndef K_ALLOC_CSP_H
#define K_ALLOC_CSP_H

#include "kubos-core/arch/k_alloc.h"
#include <stdlib.h>

#define CSP_BUFFER_SIZE 60
#define CSP_BUFFER_COUNT 10

void * _csp_new(size_t size);
void _csp_free(void * ptr);
void * _csp_realloc(void * buff, size_t old_size, size_t new_size);
void k_alloc_csp_init();

k_alloc_t csp_alloc;

#define K_BUFFER_NEW_CSP(n, d, s) \
        k_buffer_alloc(n, d, s, &csp_alloc)

#define K_BUFFER_FREE_CSP(b) \
        k_buffer_free_new(b, &csp_alloc)

#define K_BUFFER_REALLOC_CSP(b, s) \
        k_buffer_realloc_new(b, s, &csp_alloc)

#endif
