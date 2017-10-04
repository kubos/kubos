/*
 * Copyright (C) 2017 Kubos Corporation
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

/* Mock Linux system calls to use for KubOS Linux HAL unit tests */

#include <cmocka.h>
#include <unistd.h>
#include <errno.h>

char test_char;

//TODO: Add param checking

int __wrap_open(const char * filename, int flags)
{
    test_char = 0;
    return mock_type(int);
}

int __wrap_close(int fd)
{
    return mock_type(int);
}

int __wrap_ioctl(int fd, unsigned long request, ...)
{
    return mock_type(int);
}

ssize_t __wrap_write(int fd, const char *buf, size_t count)
{
    test_char = *buf;
    return mock_type(ssize_t);
}

ssize_t __wrap_read(int fd, char *buf, size_t count)
{
    *buf = test_char;

    /* Only relevant when we make the read call fail */
    errno = EREMOTEIO;

    return mock_type(ssize_t);
}
