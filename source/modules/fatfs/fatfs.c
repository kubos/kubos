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
{
  "fs": {
    "fatfs"
  }
}
**/
/**
 * This code is not currently compiled because
 * 1. It is not compatible with our current msp430 libc
 * 2. It is not used by the basic FatFs layer currently implemented 
 */ 
#if 0

#include <errno.h>
#include <fcntl.h>
#include <string.h>
#include <time.h>

#include "kubos-core/modules/fatfs/ffconf.h"
#include "kubos-core/modules/fatfs/ff.h"

#include "kubos-core/modules/fs/fs.h"
#include "kubos-core/modules/fatfs/fatfs.h"

#include "kubos-core/k_debug.h"
#define DEBUG_FRESULT(op__, res__) DEBUG(op__ ": %s\n", FRESULT_str[res__])
#define DEBUG_FRESULT(...)

#define CHECK_FRESULT_NOREENT(op) do { \
    FRESULT res__ = op; \
    if (res__ != FR_OK) { \
        DEBUG_FRESULT(#op, res__); \
        return FRESULT_to_errno(res__); \
    } \
} while (0)

#define CHECK_FRESULT(op) do { \
    FRESULT res__ = op; \
    if (res__ != FR_OK) { \
        DEBUG_FRESULT(#op, res__); \
        FS_RETURN_ERR(r, FRESULT_to_errno(res__)); \
    } \
} while (0)

typedef struct {
    bool in_use;
    const char *name;
    FIL file;
} fatfs_handle_t;

static fatfs_handle_t _fat_handles[FS_MAX_FDS];
static FATFS _fat_fs;
static FAT_DIR _fat_dir;

static fatfs_handle_t* first_free_handle(void)
{
    for (uint8_t i = 0; i < FS_MAX_FDS; i++) {
        if (!_fat_handles[i].in_use) {
            return &_fat_handles[i];
        }
    }
    return NULL;
}

fs_dev_t fatfs_dev = {
    .name = "FATFS",
    .priv_data = NULL,
    .mount = fatfs_mount,
    .unmount = fatfs_unmount,
    .open = fatfs_open,
    .close = fatfs_close,
    .write = fatfs_write,
    .read = fatfs_read,
    .lseek = fatfs_lseek,
    .sync = fatfs_sync,
    .fstat = fatfs_fstat,
    .stat = fatfs_stat,
    .unlink = fatfs_unlink
};

static inline int FRESULT_to_errno(FRESULT res)
{
    switch (res) {
        case FR_OK: return 0;
        case FR_DISK_ERR: return EIO;
        case FR_INT_ERR: return EINTR;
        case FR_NOT_READY: return ENODEV;
        case FR_NO_FILE: return ENOENT;
        case FR_NO_PATH: return ENOENT;
        case FR_INVALID_NAME: return EINVAL;
        case FR_DENIED: return EACCES;
        case FR_EXIST: return EEXIST;
        case FR_INVALID_OBJECT: return EINVAL;
        case FR_WRITE_PROTECTED: return EROFS;
        case FR_INVALID_DRIVE: return EINVAL;
        case FR_NOT_ENABLED: return ENODEV;
        case FR_NO_FILESYSTEM: return EILSEQ;
        case FR_MKFS_ABORTED: return ECANCELED;
        case FR_TIMEOUT: return ETIMEDOUT;
        case FR_NOT_ENOUGH_CORE: return ENOMEM;
        case FR_TOO_MANY_OPEN_FILES: return ENFILE;
        case FR_INVALID_PARAMETER: return EINVAL;
        case FR_LOCKED: return ENOLCK;
    }

    return 0;
}

#if ENABLE_DEBUG
static char* FRESULT_str[] = {
    "FR_OK", "FR_DISK_ERR", "FR_INT_ERR", "FR_NOT_READY", "FR_NO_FILE",
    "FR_NO_PATH", "FR_INVALID_NAME", "FR_DENIED", "FR_EXIST",
    "FR_INVALID_OBJECT", "FR_WRITE_PROTECTED", "FR_INVALID_DRIVE",
    "FR_NOT_ENABLED", "FR_NO_FILESYSTEM", "FR_MKFS_ABORTED", "FR_TIMEOUT",
    "FR_LOCKED", "FR_NOT_ENOUGH_CORE", "FR_TOO_MANY_OPEN_FILES",
    "FR_INVALID_PARAMETER"
};
#endif

int fatfs_mount(fs_mount_t *mnt)
{
    CHECK_FRESULT_NOREENT(f_mount(&_fat_fs, "", 1));
    return 0;
}

int fatfs_unmount(fs_mount_t *mnt)
{
    CHECK_FRESULT_NOREENT(f_mount(NULL, "", 1));
    return 0;
}

int fatfs_open(struct _reent *r, const char *path, int flags, int mode, fs_fd_t *fd_out)
{
    if (!path || !fd_out) {
        FS_RETURN_ERR(r, EINVAL);
    }

    fatfs_handle_t *handle = first_free_handle();
    if (!handle) {
        FS_RETURN_ERR(r, ENOMEM);
    }

    DEBUG("fatfs_open %s mode=%08X, flags=%08X\n", path, mode, flags);

    BYTE fat_mode = 0;
    if ((flags & O_RDONLY) == O_RDONLY) {
        fat_mode |= FA_READ;
    }

    if ((flags & O_WRONLY) == O_WRONLY) {
        fat_mode |= FA_WRITE;
    }

    if ((flags & O_RDWR) == O_RDWR) {
        fat_mode |= FA_READ | FA_WRITE;
    }

    if ((flags & O_TRUNC) == O_TRUNC) {
        fat_mode |= FA_CREATE_ALWAYS;
    } else if ((flags & O_CREAT) == O_CREAT) {
        fat_mode |= FA_OPEN_ALWAYS;
    } else {
        fat_mode |= FA_OPEN_EXISTING;
    }

    CHECK_FRESULT(f_open(&handle->file, path, fat_mode));
    handle->in_use = true;
    handle->name = path;
    fd_out->priv_data = handle;
    return 0;
}

int fatfs_close(struct _reent *r, fs_fd_t *fd)
{
    if (!fd) {
        FS_RETURN_ERR(r, EINVAL);
    }

    fatfs_handle_t* handle = (fatfs_handle_t *) fd->priv_data;
    if (!handle) {
        FS_RETURN_ERR(r, EBADF);
    }

    CHECK_FRESULT(f_close(&handle->file));
    handle->in_use = 0;
    return 0;
}

long fatfs_write(struct _reent *r, fs_fd_t *fd, const char *ptr, int len)
{
    if (!fd || !ptr || len < 0) {
        FS_RETURN_ERR(r, EINVAL);
    }

    fatfs_handle_t* handle = (fatfs_handle_t *) fd->priv_data;
    if (!handle) {
        FS_RETURN_ERR(r, EBADF);
    }

    uint16_t written;
    CHECK_FRESULT(f_write(&handle->file, ptr, (UINT) len, (UINT *) &written));

    return written;
}

long fatfs_read(struct _reent *r, fs_fd_t *fd, char *ptr, int len)
{
    if (!fd || !ptr || len < 0) {
        FS_RETURN_ERR(r, EINVAL);
    }

    fatfs_handle_t* handle = (fatfs_handle_t*) fd->priv_data;
    if (!handle) {
        FS_RETURN_ERR(r, EBADF);
    }

    UINT bytes_read;
    CHECK_FRESULT(f_read(&handle->file, ptr, (UINT) len, &bytes_read));

    return bytes_read;
}

_off_t fatfs_lseek(struct _reent *r, fs_fd_t *fd, _off_t pos, int dir)
{
    if (!fd) {
        FS_RETURN_ERR(r, EINVAL);
    }

    fatfs_handle_t* handle = (fatfs_handle_t*) fd->priv_data;
    if (!handle) {
        FS_RETURN_ERR(r, EBADF);
    }

    DWORD new_pos = (DWORD) pos;
    if (dir == SEEK_CUR) {
        new_pos += f_tell(&handle->file);
    } else if (dir == SEEK_END) {
        new_pos += f_size(&handle->file);
    }

    CHECK_FRESULT(f_lseek(&handle->file, new_pos));

    return (_off_t) f_tell(&handle->file);
}

int fatfs_sync(struct _reent *r, fs_fd_t *fd)
{
    if (!fd) {
        FS_RETURN_ERR(r, EINVAL);
    }

    fatfs_handle_t* handle = (fatfs_handle_t*) fd->priv_data;
    if (!handle) {
        FS_RETURN_ERR(r, EBADF);
    }

    CHECK_FRESULT(f_sync(&handle->file));
    return 0;
}

int fatfs_fstat(struct _reent *r, fs_fd_t *fd, struct stat *st)
{
    if (!fd || !st) {
        FS_RETURN_ERR(r, EINVAL);
    }

    fatfs_handle_t* handle = (fatfs_handle_t*) fd->priv_data;
    if (!handle) {
        FS_RETURN_ERR(r, EBADF);
    }

    return fatfs_stat(r, (char *) handle->name, st);
}

static inline int info_to_time(FILINFO *info)
{
    struct tm time;
    time.tm_year = ((info->fdate >> 9) + 1980) - 1900;
    time.tm_mon = (info->fdate >> 5 & 15) - 1;
    time.tm_mday = info->fdate & 31;
    time.tm_hour = info->ftime >> 11;
    time.tm_min = info->ftime >> 5 & 63;
    return mktime(&time);
}

static inline void info_to_stat(FILINFO *info, struct stat *st)
{
    st->st_mode = 0;
    if (info->fattrib & AM_DIR) {
        st->st_mode |= S_IFDIR;
    } else {
        st->st_mode |= S_IFREG;
    }

    st->st_size = info->fsize;
    st->st_mtime = info_to_time(info);
}

int fatfs_stat(struct _reent *r, char *path, struct stat *st)
{
    FILINFO info;
    if (!path || !st) {
        FS_RETURN_ERR(r, EINVAL);
    }

    CHECK_FRESULT(f_stat(path, &info));

    info_to_stat(&info, st);
    return 0;
}

int fatfs_unlink(struct _reent *r, char *path)
{
    if (!path) {
        FS_RETURN_ERR(r, EINVAL);
    }

    CHECK_FRESULT(f_unlink(path));
    return 0;
}

int fatfs_opendir(fs_dir_t *out, const char *path)
{
    if (!out || !path) {
        return EINVAL;
    }

    CHECK_FRESULT_NOREENT(f_opendir(&_fat_dir, path));

    out->priv_data = &_fat_dir;
    return 0;
}

int fatfs_readdir(fs_dir_t *dir, fs_info_t *info)
{
    if (!dir || !info) {
        return EINVAL;
    }

    FILINFO fat_info;
    CHECK_FRESULT_NOREENT(f_readdir((FAT_DIR *) dir->priv_data, &fat_info));

#if _USE_LFN
    char *fn = *fat_info.lfname ? fat_info.lfname : fat_info.fname;
    size_t len = *fat_info.lfname ? fat_info.lfsize : 13;
#else
    char *fn = fat_info.fname;
    size_t len = 13;
#endif

    info->name_len = (uint16_t) len > FS_MAX_FILENAME ? FS_MAX_FILENAME : len;
    strncpy(info->name, fn, (size_t) info->name_len);
    info_to_stat(&fat_info, &info->st);

    return 0;
}

int fatfs_closedir(fs_dir_t *dir)
{
    if (!dir || !dir->priv_data) {
        return EINVAL;
    }

    CHECK_FRESULT_NOREENT(f_closedir((FAT_DIR *) dir->priv_data));
    return 0;
}

#endif
