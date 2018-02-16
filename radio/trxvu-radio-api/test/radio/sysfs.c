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

/* Mock Linux system calls to use for Kubos Linux HAL unit tests */

#include <cmocka.h>
#include <errno.h>
#include <unistd.h>

/* Returns a file descriptor or -1 on failure */
int __wrap_open(const char * filename, int flags)
{
    return mock_type(int);
}

/* Returns 0 on success and -1 on failure */
int __wrap_close(int fd)
{
    return mock_type(int);
}

/* 
 * Returns 0 on success (or occasionally a positive value) and -1 on failure
 */
int __wrap_ioctl(int fd, unsigned long request, ...)
{
    /* Pretty sure this shouldn't ever fail */
    return 0;
}

/* Returns number of bytes "written" or -1 on failure */
ssize_t __wrap_write(int fd, const char * buf, size_t count)
{
    /* Verify that we're sending the correct command */
    uint8_t cmd = buf[0];
    check_expected(cmd);

    return (ssize_t) count;
}

/* Returns number of bytes "read" or -1 on failure */
ssize_t __wrap_read(int fd, char * buf, size_t count)
{
    ssize_t len = (ssize_t) mock();

    if (len < 0)
    {
        /* Only relevant when we make the read call fail */
        errno = EREMOTEIO;
        return -1;
    }
    char * data = (char *) mock();
    memcpy(buf, data, (int) len);

    return len;
}
