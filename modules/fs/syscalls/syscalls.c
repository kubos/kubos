/*
 * KubOS Core Flight Services
 * Copyright (C) 2015 Kubos Corporation
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

#include <errno.h>
#include <string.h>

#ifdef MODULE_UART_STDIO
#include "uart_stdio.h"
#endif

#include "board.h"
#include "thread.h"
#include "fs.h"
#include "xtimer.h"

#define IS_STDIO(fd) (fd == STDIN_FILENO || fd == STDOUT_FILENO || fd == STDERR_FILENO)

extern char _sheap;                 /* start of the heap */
extern char _eheap;                 /* end of the heap */
char *heap_top = &_sheap + 4;

void _init(void)
{
#ifdef MODULE_UART_STDIO
    uart_stdio_init();
#endif
}

void _fini(void)
{
    /* nothing to do here */
}

void _exit(int n)
{
    printf("#! exit %i: resetting\n", n);
    reboot(n);
    while(1);
}

void *_sbrk_r(struct _reent *r, ptrdiff_t incr)
{
    unsigned int state = disableIRQ();
    void *res = heap_top;

    if ((heap_top + incr > &_eheap) || (heap_top + incr < &_sheap)) {
        r->_errno = ENOMEM;
        res = (void *)-1;
    }
    else {
        heap_top += incr;
    }

    restoreIRQ(state);
    return res;
}

pid_t _getpid(void)
{
    return sched_active_pid;
}

pid_t _getpid_r(struct _reent *ptr)
{
    (void) ptr;
    return sched_active_pid;
}

__attribute__ ((weak))
int _kill_r(struct _reent *r, pid_t pid, int sig)
{
    (void) pid;
    (void) sig;
    r->_errno = ESRCH;                      /* not implemented yet */
    return -1;
}

__attribute__ ((weak))
int _kill(pid_t pid, int sig)
{
    (void) pid;
    (void) sig;
    errno = ESRCH;                         /* not implemented yet */
    return -1;
}

int _open_r(struct _reent *r, const char *name, int flags, int mode)
{
    return fs_open(r, name, flags, mode);
}

int _read_r(struct _reent *r, int fd, void *buffer, unsigned int count)
{
#ifdef MODULE_UART_STDIO
    if (IS_STDIO(fd)) {
        return uart_stdio_read(buffer, count);
    }
#endif

    return fs_read(r, fd, buffer, count);
}

int _write_r(struct _reent *r, int fd, const void *data, unsigned int count)
{
#ifdef MODULE_UART_STDIO
    if (IS_STDIO(fd)) {
        return uart_stdio_write(data, count);
    }
#endif

    return fs_write(r, fd, data, count);
}

int _close_r(struct _reent *r, int fd)
{
    if (IS_STDIO(fd)) {
        r->_errno = ENODEV;                     /* not implemented yet */
        return -1;
    }

    return fs_close(r, fd);
}

_off_t _lseek_r(struct _reent *r, int fd, _off_t pos, int dir)
{
    if (IS_STDIO(fd)) {
        r->_errno = ENODEV;                     /* not implemented yet */
        return -1;
    }

    return fs_lseek(r, fd, pos, dir);
}

int _fsync_r(struct _reent *r, int fd)
{
    if (IS_STDIO(fd)) {
        return -1;
    }

    return fs_sync(r, fd);
}

int fsync(int fd)
{
    struct _reent r;
    if (_fsync_r(&r, fd) == -1) {
        errno = r._errno;
        return r._errno;
    }
    return 0;
}

int _fstat_r(struct _reent *r, int fd, struct stat *st)
{
    if (IS_STDIO(fd)) {
        r->_errno = ENODEV;                     /* not implemented yet */
        return -1;
    }

    return fs_fstat(r, fd, st);
}

int _stat_r(struct _reent *r, char *name, struct stat *st)
{
    return fs_stat(r, name, st);
}

int _isatty_r(struct _reent *r, int fd)
{
    r->_errno = 0;
    return IS_STDIO(fd) ? 1 : 0;
}

int _unlink_r(struct _reent *r, char *path)
{
    return fs_unlink(r, path);
}
