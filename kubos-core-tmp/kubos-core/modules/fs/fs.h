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

#ifdef YOTTA_CFG_FS

#ifndef FS_H
#define FS_H

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <sys/stat.h>
#include <sys/unistd.h>

#ifdef HAVE_FS_CONF_H
#include "fs_conf.h"
#endif

#ifndef FS_MAX_MOUNTS
#define FS_MAX_MOUNTS 2
#endif

#ifndef FS_MAX_FDS
#define FS_MAX_FDS 10
#endif

#ifndef FS_MAX_FILENAME
#define FS_MAX_FILENAME 32
#endif

#define FS_RETURN_ERR(r__, e__) do { \
    r__->_errno = e__; \
    return -1; \
} while (0)

struct fs_mount_s;
struct fs_fd_s;
struct fs_dir_s;
struct fs_info_s;

typedef struct {
    const char *name;
    void *priv_data;

    int (*mount)(struct fs_mount_s *mnt);
    int (*unmount)(struct fs_mount_s *mnt);
    int (*open)(struct _reent *r, const char *path, int flags, int mode, struct fs_fd_s *fd_out);
    int (*close)(struct _reent *r, struct fs_fd_s *fd);
    long (*write)(struct _reent *r, struct fs_fd_s *fd, const char *ptr, int len);
    long (*read)(struct _reent *r, struct fs_fd_s *fd, char *ptr, int len);
    _off_t (*lseek)(struct _reent *r, struct fs_fd_s *fd, _off_t pos, int dir);
    int (*sync)(struct _reent *r, struct fs_fd_s *fd);
    int (*fstat)(struct _reent *r, struct fs_fd_s *fd, struct stat *st);
    int (*stat)(struct _reent *r, char *path, struct stat *st);
    int (*unlink)(struct _reent *r, char *path);
    int (*opendir)(struct fs_dir_s *out, const char *path);
    int (*readdir)(struct fs_dir_s *dir, struct fs_info_s *info);
    int (*closedir)(struct fs_dir_s *dir);
} fs_dev_t;

typedef struct fs_mount_s {
    const char *mount;
    bool in_use;
    fs_dev_t *dev;
} fs_mount_t;

typedef struct fs_fd_s {
    bool active;
    int8_t fd;
    fs_dev_t *dev;
    void *priv_data;
} fs_fd_t;

typedef struct fs_dir_s {
    fs_dev_t *dev;
    void *priv_data;
} fs_dir_t;

#define FS_ATTR_DIR       0x01
#define FS_ATTR_READONLY  0x02
#define FS_ATTR_HIDDEN    0x04

typedef struct fs_info_s {
    fs_dev_t *dev;
    char name[FS_MAX_FILENAME];
    uint16_t name_len;
    struct stat st;
} fs_info_t;

void fs_init(void);
int fs_mount(const char *mount, fs_dev_t *dev);
int fs_open(struct _reent *r, const char *path, int flags, int mode);
long fs_read(struct _reent *r, int fd, void *buffer, unsigned int count);
long fs_write(struct _reent *r, int fd, const void *data, unsigned int count);
int fs_close(struct _reent *r, int fd);
_off_t fs_lseek(struct _reent *r, int fd, _off_t pos, int dir);
int fs_sync(struct _reent *r, int fd);
int fs_fstat(struct _reent *r, int fd, struct stat *st);
int fs_stat(struct _reent *r, char *path, struct stat *st);
int fs_unlink(struct _reent *r, char *path);
int fs_unmount(const char *mount);
int fs_next_fd(void);

int fs_opendir(fs_dir_t *out, const char *path);
int fs_readdir(fs_dir_t *dir, fs_info_t *info);
int fs_closedir(fs_dir_t *dir);
#endif

#endif
