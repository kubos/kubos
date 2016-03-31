#include <stdint.h>
#include "kubos-core/arch/kc_types.h"

/**
 * @brief Describes a message object which can be sent between threads.
 *
 * User can set type and one of content.ptr and content.value. (content is a union)
 * The meaning of type and the content fields is totally up to the user,
 * the corresponding fields are never read by the kernel.
 *
 */
typedef struct msg {
    kernel_pid_t sender_pid;    /**< PID of sending thread. Will be filled in
                                     by msg_send. */
    uint16_t type;              /**< Type field. */
    union {
        char *ptr;              /**< Pointer content field. */
        uint32_t value;         /**< Value content field. */
    } content;                  /**< Content of the message. */
} msg_t;


/**
 * @brief Send a message (blocking).
 *
 * This function sends a message to another thread. The ``msg_t`` structure has
 * to be allocated (e.g. on the stack) before calling the function and can be
 * freed afterwards. If called from an interrupt, this function will never
 * block.
 *
 * @param[in] m             Pointer to preallocated ``msg_t`` structure, must
 *                          not be NULL.
 * @param[in] target_pid    PID of target thread
 *
 * @return 1, if sending was successful (message delivered directly or to a
 *            queue)
 * @return 0, if called from ISR and receiver cannot receive the message now
 *            (it is not waiting or it's message queue is full)
 * @return -1, on error (invalid PID)
 */
int kc_msg_send(msg_t *m, kernel_pid_t target_pid);
