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
#include <stdio.h>

#include "shell.h"

#define TAG "kubos"
#include "klog.h"

#ifdef MODULE_GPS
    extern int location_demo(int argc, char **argv);
#endif

#ifdef MODULE_HAM
#include "ham_shell.h"
#endif

int hello_world(int argc, char **argv) {
    /* Suppress compiler errors */
    (void)argc;
    (void)argv;
    printf("hello world!\n");
    return 0;
}

const shell_command_t shell_commands[] = {
    {"hello", "prints hello world", hello_world},
#ifdef MODULE_GPS
    {"gps", "Gets GPS Data", location_demo},
#endif
#ifdef MODULE_HAM
    HAM_SHELL_COMMANDS
#endif
    { NULL, NULL, NULL }
};

int main(void)
{
    KLOG_INFO(TAG, "Welcome to KubOS! Initializing...");

#ifdef MODULE_HAM
    ham_cmd_init();
#endif

    /* start shell */
    KLOG_INFO(TAG, "All up, running the shell now");

    char line_buf[SHELL_DEFAULT_BUFSIZE];
    shell_run(shell_commands, line_buf, SHELL_DEFAULT_BUFSIZE);

    /* should be never reached */
    return 0;
}
