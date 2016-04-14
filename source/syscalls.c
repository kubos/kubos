/*
 * KubOS RT
 * Copyright (C) 2016 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#include "stm32f4xx.h"
#include "FreeRTOS.h"

#include <reent.h>
#include <unistd.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/time.h>

extern unsigned int __heap_size;
extern unsigned int __mbed_sbrk_start;

#define SBRK_ALIGN 8U
#define SBRK_INC_MIN (SBRK_ALIGN)

void * _sbrk_r(struct _reent *r, ptrdiff_t size) {
    static void * volatile mbed_sbrk_ptr;
    static volatile ptrdiff_t mbed_sbrk_diff;
    if (!mbed_sbrk_ptr) {
        mbed_sbrk_ptr = &__mbed_sbrk_start;
        mbed_sbrk_diff = (ptrdiff_t) &__heap_size;
    }

    if (size == 0) {
        return (void *) mbed_sbrk_ptr;
    }

    ptrdiff_t size_internal = abs(size);
    if ((uintptr_t)size_internal < SBRK_INC_MIN) {
            size_internal = SBRK_INC_MIN;
    }

    size_internal = (size_internal + SBRK_ALIGN - 1) & ~(SBRK_ALIGN - 1);
    // it's min sized plus aligned, assign back the sign
    if (size < 0) {
        size_internal = -size_internal;
    }

    if (size_internal > mbed_sbrk_diff) {
        r->_errno = ENOMEM;
        return (void *) -1;
    }

    mbed_sbrk_ptr += size_internal;
    mbed_sbrk_diff -= size_internal;
    return (void *)(mbed_sbrk_ptr - size_internal);
}


ssize_t _read_r(struct _reent *r, int fd, void *ptr, size_t len)
{
    r->_errno = ENOSYS;
    return len;
    //return k_uart_read_r(r, fd, ptr, len);
}


ssize_t _write_r(struct _reent *r, int fd, const void *ptr, size_t len)
{
    r->_errno = ENOSYS;
    return len;
    //return k_uart_write_r(r, fd, ptr, len);
}

off_t _lseek_r(struct _reent *r, int fd, _off_t ptr, int dir)
{
    fd  = fd;
    ptr = ptr;
    dir = dir;
    r->_errno = ENOSYS;
    return -1;
}

int _close_r(struct _reent *r, int fd)
{
    fd = fd;
    r->_errno = ENOSYS;
    return -1;
}

int _fstat_r(struct _reent *r, int fd, struct stat *st)
{
    if ((fd >= STDIN_FILENO) && (fd <= STDERR_FILENO)) {
      st->st_mode = S_IFCHR;
      return 0;
    }

    r->_errno = ENOSYS;
    return -1;
}

int _isatty_r(struct _reent *r, int fd)
{
    r  = r;
    fd = fd;
    return 1;
}

int _kill_r(struct _reent *r, int pid, int signal)
{
    r->_errno = ENOSYS;
    return -1;
}

int _getpid_r(struct _reent *r)
{
    r->_errno = ENOSYS;
    return -1;
}

void __malloc_lock(struct _reent *r)
{
    vPortEnterCritical();
}

void __malloc_unlock(struct _reent *r)
{
    vPortExitCritical();
}
