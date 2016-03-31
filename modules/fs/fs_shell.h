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
#ifndef FS_SHELL_H
#define FS_SHELL_H

int cat_cmd(int argc, char **argv);
int cp_cmd(int argc, char **argv);
int ls_cmd(int argc, char **argv);
int mount_cmd(int argc, char **argv);
int mv_cmd(int argc, char **argv);
int rm_cmd(int argc, char **argv);
int unmount_cmd(int argc, char **argv);

#define FS_SHELL_COMMANDS \
    { "cat", "Print the contents of a file", cat_cmd }, \
    { "cp", "Copy a file", cp_cmd }, \
    { "ls", "List the contents of a directory", ls_cmd }, \
    { "mount", "Mount a filesystem", mount_cmd }, \
    { "mv", "Move/rename a file", mv_cmd }, \
    { "rm", "Remove a file", rm_cmd }, \
    { "unmount", "Unmount a filesystem", unmount_cmd },

#endif
