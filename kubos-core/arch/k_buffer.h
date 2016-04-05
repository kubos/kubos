#ifndef K_BUFFER_H
#define K_BUFFER_H

#include <stdlib.h>
#include "kubos-core/arch/k_alloc.h"
#include "kubos-core/arch/k_alloc_csp.h"
#include "kubos-core/arch/k_alloc_malloc.h"

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
