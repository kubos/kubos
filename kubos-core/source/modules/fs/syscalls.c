/*
 * KubOS Core Flight Services
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

/**
 * This code is not currently compiled because
 * 1. It is not compatible with our current msp430 libc
 * 2. It is not used by the basic FatFs layer currently implemented 
 */ 
#if 0

#include <errno.h>
#include <string.h>

#include "kubos-core/modules/fs/fs.h"

extern char _sheap;                 /* start of the heap */
extern char _eheap;                 /* end of the heap */
char *heap_top = &_sheap + 4;

#define IS_STDIO(fd) (fd == STDIN_FILENO || fd == STDOUT_FILENO || fd == STDERR_FILENO)

int _open_r(struct _reent *r, const char *name, int flags, int mode)
{
    return fs_open(r, name, flags, mode);
}

int _read_r(struct _reent *r, int fd, void *buffer, unsigned int count)
{
    return fs_read(r, fd, buffer, count);
}

int _write_r(struct _reent *r, int fd, const void *data, unsigned int count)
{
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

#endif
