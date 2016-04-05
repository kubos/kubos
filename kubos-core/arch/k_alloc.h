#ifndef K_ALLOC_H
#define K_ALLOC_H

#include <stdlib.h>

typedef struct k_alloc {
    void * (* _new)(size_t size);
    void * (* _realloc)(void * buff, size_t old_size, size_t new_size);
    void (* _free)();
} k_alloc_t;

#endif
