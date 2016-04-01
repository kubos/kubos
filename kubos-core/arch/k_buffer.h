#ifndef KC_PKTBUF_H
#define KC_PKTBUF_H


typedef struct k_buffer {
    int size;
    void * data;
} k_buffer_t;

k_buffer_t *k_buffer_new(void * data, int size);

#endif
