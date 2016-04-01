#include <stdlib.h>
#include "kubos-core/arch/k_buffer.h"


k_buffer_t *k_buffer_new(void * data, int size)
{
    k_buffer_t * newbuff = NULL;
    newbuff = malloc(sizeof(k_buffer_t));
    newbuff->data = malloc(sizeof(char) * size);
    
    return newbuff;
}
