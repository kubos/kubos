#ifndef KC_PKTBUF_H
#define KC_PKTBUF_H


typedef struct k_pktbuf {
    struct k_pktbuf * next;
    int size;
    void * data;
} k_pktbuf_t;

k_pktbuf_t *k_pktbuf_add(k_pktbuf_t * next, void * data, int size);

k_pktbuf_t *k_pktbuf_realloc_data(k_pktbuf_t *pkt, int size);

#endif
