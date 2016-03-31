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

#include "board.h"
#include "fs.h"
#include "xtimer.h"

#define FS_FD_START (STDERR_FILENO + 1)

static fs_mount_t _fs_mounts[FS_MAX_MOUNTS];
static fs_fd_t _fs_fds[FS_MAX_FDS];

static int find_free_mount(void)
{
    for (int i = 0; i < FS_MAX_MOUNTS; i++) {
        if (!_fs_mounts[i].mount) {
            return i;
        }
    }
    return -1;
}

static int find_free_fd(void)
{
    for (int i = 0; i < FS_MAX_FDS; i++) {
        if (!_fs_fds[i].active) {
            return i;
        }
    }
    return -1;
}

static fs_fd_t *find_fd(int fd)
{
    for (int i = 0; i < FS_MAX_FDS; i++) {
        if (_fs_fds[i].active && _fs_fds[i].fd == fd) {
            return &_fs_fds[i];
        }
    }
    return NULL;
}

static fs_mount_t *find_mount(char *path)
{
    fs_mount_t *mnt = NULL;
    for (int i = 0; i < FS_MAX_MOUNTS; i++) {
        if (!_fs_mounts[i].mount) {
            continue;
        }

        if (strncmp(path, _fs_mounts[i].mount,
                    strlen(_fs_mounts[i].mount)) == 0) {

            mnt = &_fs_mounts[i];
            break;
        }
    }

    return mnt;
}

static void reset_mount(fs_mount_t *mount)
{
    mount->mount = NULL;
    mount->in_use = false;
    mount->dev = NULL;
}

void fs_init(void)
{
    int i;
    for (i = 0; i < FS_MAX_MOUNTS; i++) {
        reset_mount(&_fs_mounts[i]);
    }

    for (i = 0; i < FS_MAX_FDS; i++) {
        _fs_fds[i].active = false;
        _fs_fds[i].fd = i + FS_FD_START;
    }
}

int fs_mount(const char *mount, fs_dev_t *dev)
{
    if (!mount || !dev) {
        return EINVAL;
    }

    int mnt_index = find_free_mount();
    if (mnt_index == -1) {
        return ENOMEM;
    }

    _fs_mounts[mnt_index].mount = mount;
    _fs_mounts[mnt_index].in_use = true;
    _fs_mounts[mnt_index].dev = dev;

    if (!dev->mount) {
        return ENOTSUP;
    }

    return dev->mount(&_fs_mounts[mnt_index]);
}

int fs_open(struct _reent *r, const char *path, int flags, int mode)
{
    if (!path) {
        FS_RETURN_ERR(r, EINVAL);
    }

    int fd_ = find_free_fd();
    if (fd_ == -1) {
        FS_RETURN_ERR(r, ENOMEM);
    }

    fs_fd_t *fd = &_fs_fds[fd_];
    fs_mount_t *mnt = find_mount((char *)path);
    if (!mnt || !mnt->dev) {
        FS_RETURN_ERR(r, ENODEV);
    }

    if (!mnt->dev->open) {
        FS_RETURN_ERR(r, ENOTSUP);
    }

    path += strlen(mnt->mount) + 1;

    int result = mnt->dev->open(r, path, flags, mode, fd);
    if (result == 0) {
        fd->dev = mnt->dev;
        fd->active = true;
        return fd->fd;
    }

    return -1;
}

#define FS_DEV_CHECK_IMPL(fd_name, __fn, __fd) \
    fs_fd_t *fd_name = find_fd(__fd); \
    if (!fd_name) FS_RETURN_ERR(r, ENOENT); \
    if (!fd_name->dev) FS_RETURN_ERR(r, ENODEV); \
    if (!fd_name->dev->__fn) FS_RETURN_ERR(r, ENOTSUP)

#define FS_DEV_WRAP(__fn, __fd, ...) \
    FS_DEV_CHECK_IMPL(fd__, __fn, __fd); \
    return fd__->dev->__fn(r, fd__, ## __VA_ARGS__)

long fs_read(struct _reent *r, int fd, void *buffer, unsigned int count)
{
    FS_DEV_CHECK_IMPL(fd_impl, read, fd);

    long result = fd_impl->dev->read(r, fd_impl, buffer, count);
    return result;
}

long fs_write(struct _reent *r, int fd, const void *data, unsigned int count)
{
    FS_DEV_WRAP(write, fd, data, count);
}

int fs_close(struct _reent *r, int fd_)
{
    fs_fd_t *fd = find_fd(fd_);
    if (!fd) {
        FS_RETURN_ERR(r, ENOENT);
    }

    if (!fd->dev) {
        FS_RETURN_ERR(r, ENODEV);
    }

    if (!fd->dev->close) {
        FS_RETURN_ERR(r, ENOTSUP);
    }

    int result = fd->dev->close(r, fd);
    return result;
}

_off_t fs_lseek(struct _reent *r, int fd, _off_t pos, int dir)
{
    FS_DEV_WRAP(lseek, fd, pos, dir);
}

int fs_sync(struct _reent *r, int fd)
{
    FS_DEV_WRAP(sync, fd);
}

int fs_fstat(struct _reent *r, int fd, struct stat *st)
{
    FS_DEV_WRAP(fstat, fd, st);
}

int fs_stat(struct _reent *r, char *path, struct stat *st)
{
    if (!path || !st) {
        FS_RETURN_ERR(r, EINVAL);
    }

    fs_mount_t *mnt = find_mount(path);
    if (!mnt || !mnt->dev) {
        FS_RETURN_ERR(r, ENODEV);
    }

    if (!mnt->dev->stat) {
        FS_RETURN_ERR(r, ENOTSUP);
    }

    path += strlen(mnt->mount) + 1;
    return mnt->dev->stat(r, path, st);
}

int fs_unlink(struct _reent *r, char *path)
{
    if (!path) {
        FS_RETURN_ERR(r, EINVAL);
    }

    fs_mount_t *mnt = find_mount(path);
    if (!mnt || !mnt->dev) {
        FS_RETURN_ERR(r, ENODEV);
    }

    if (!mnt->dev->unlink) {
        FS_RETURN_ERR(r, ENOTSUP);
    }

    path += strlen(mnt->mount) + 1;
    return mnt->dev->unlink(r, path);
}

int fs_unmount(const char *mount)
{
    fs_mount_t *mnt = find_mount((char *)mount);
    if (!mnt || !mnt->dev) {
        return ENODEV;
    }

    int result = 0;
    if (mnt->dev->unmount) {
        result = mnt->dev->unmount(mnt);
    }

    if (result == 0) {
        reset_mount(mnt);
    }
    return result;
}

int fs_next_fd(void)
{
    int i = find_free_fd();
    return _fs_fds[i].fd;
}

int fs_opendir(fs_dir_t *out, const char *path)
{
    if (!out) {
        return EINVAL;
    }

    fs_mount_t *mnt = find_mount((char *) path);
    if (!mnt || !mnt->dev) {
        return ENODEV;
    }

    if (!mnt->dev->opendir) {
        return ENOTSUP;
    }

    path += strlen(mnt->mount) + 1;
    out->dev = mnt->dev;

    return mnt->dev->opendir(out, path);
}

int fs_readdir(fs_dir_t *dir, fs_info_t *info)
{
    if (!dir || !dir->dev) {
        return ENODEV;
    }

    if (!dir->dev->readdir) {
        return ENOTSUP;
    }

    return dir->dev->readdir(dir, info);
}

int fs_closedir(fs_dir_t *dir)
{
    if (!dir || !dir->dev) {
        return ENODEV;
    }

    if (!dir->dev->closedir) {
        return ENOTSUP;
    }

    return dir->dev->closedir(dir);
}
