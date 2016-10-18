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
#ifdef YOTTA_CFG_FS_FATFS
#ifndef FATFS_H
#define FATFS_H

#include "kubos-core/modules/fs/fs.h"

extern fs_dev_t fatfs_dev;

int fatfs_mount(fs_mount_t *mnt);
int fatfs_unmount(fs_mount_t *mnt);
int fatfs_open(struct _reent *r, const char *path, int flags, int mode, fs_fd_t *fd_out);
int fatfs_close(struct _reent *r, fs_fd_t *fd);
long fatfs_write(struct _reent *r, fs_fd_t *fd, const char *ptr, int len);
long fatfs_read(struct _reent *r, fs_fd_t *fd, char *ptr, int len);
_off_t fatfs_lseek(struct _reent *r, fs_fd_t *fd, _off_t pos, int dir);
int fatfs_sync(struct _reent *r, fs_fd_t *fd);
int fatfs_fstat(struct _reent *r, fs_fd_t *fd, struct stat *st);
int fatfs_stat(struct _reent *r, char *path, struct stat *st);
int fatfs_unlink(struct _reent *r, char *path);

int fatfs_opendir(fs_dir_t *out, const char *path);
int fatfs_readdir(fs_dir_t *dir, fs_info_t *info);
int fatfs_closedir(fs_dir_t *dir);

#endif
#endif
