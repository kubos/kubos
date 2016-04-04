#ifndef KC_PKTBUF_H
#define KC_PKTBUF_H


typedef struct k_buffer {
    struct k_buffer * next;
    size_t size;
    void * data;
} k_buffer_t;

k_buffer_t *k_buffer_new(void * data, size_t size);

k_buffer_t *k_buffer_add(struct k_buffer* next, void * data, size_t size);

size_t k_buffer_len(k_buffer_t * buffer);

int k_buffer_realloc(k_buffer_t * buffer, size_t new_size);

#endif
