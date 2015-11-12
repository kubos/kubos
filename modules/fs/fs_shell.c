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
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef MODULE_FATFS
#include "fatfs.h"
#endif

#include "fs.h"
#include "fs_shell.h"

#define CAT_BUF_SIZE 256

int cat_cmd(int argc, char **argv)
{
    if (argc < 2) {
        printf("Usage: cat <path>\n");
        return -1;
    }

    char *path = argv[1];

    FILE *f = fopen(path, "r");
    if (!f) {
        printf("Error opening %s: %s\n", path, strerror(errno));
        return -1;
    }


    char buffer[CAT_BUF_SIZE];
    size_t i, bytes_read;
    do {
        bytes_read = fread(buffer, 1, CAT_BUF_SIZE, f);
        if (ferror(f)) {
            printf("Error reading: %s, bytes read: %d\n", strerror(errno), bytes_read);
            fclose(f);
            return -1;
        }

        for (i = 0; i < bytes_read; i++) {
            putchar(buffer[i]);
        }
    } while (bytes_read == CAT_BUF_SIZE);

    putchar('\n');
    fclose(f);
    return 0;
}


int cp_cmd(int argc, char **argv)
{
    return 0;
}

static void print_info(char *path, int path_len, struct stat *st)
{
    char timestamp[32];
    strftime(timestamp, 32, "%b %d %H:%M", localtime(&st->st_mtime)),

    printf("%c % 10d %s %*s\n",
           S_ISDIR(st->st_mode) ? 'd' : '-',
           (int) st->st_size,
           timestamp,
           path_len,
           path);
}

int ls_cmd(int argc, char **argv)
{
    if (argc < 2) {
        printf("Usage: ls <path>\n");
        return -1;
    }

    struct stat st;
    if (stat(argv[1], &st) != 0) {
        printf("Unknown path: %s\n", argv[1]);
        return -1;
    }

    if (st.st_mode & S_IFDIR) {
        fs_dir_t dir;
        if (fs_opendir(&dir, argv[1]) != 0) {
            printf("Error opening %s: %s\n", argv[1], strerror(errno));
            return -1;
        }

        fs_info_t info;
        while (fs_readdir(&dir, &info) == 0) {
            print_info(info.name, info.name_len, &(info.st));
        }

        fs_closedir(&dir);
    } else {
        print_info(argv[1], strlen(argv[1]), &st);
    }
    return 0;
}

int mount_cmd(int argc, char **argv)
{
    if (argc < 3) {
        printf("Usage: mount <path> <driver>\n");
        return -1;
    }

    char *path = argv[1];
    char *driver = argv[2];

#ifdef MODULE_FATFS
    int result = 0;
    if (strcasecmp("FATFS", driver) == 0) {
        result = fs_mount(path, &fatfs_dev);
    }
#endif
    else {
        printf("Error: Unknown driver: %s\n", driver);
        return -1;
    }

    if (result != 0) {
        printf("Error mounting %s: %s\n", path, strerror(result));
        return -1;
    }

    printf("Successfully mounted %s at %s\n", driver, path);
    return 0;
}

int mv_cmd(int argc, char **argv)
{
    return 0;
}

int rm_cmd(int argc, char **argv)
{
    return 0;
}

int unmount_cmd(int argc, char **argv)
{
    return 0;
}
