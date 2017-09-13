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

#pragma once

#include <dbus/dbus.h>

/* Macro Definitions */

/* Error Codes for ECP_*() calls */
#define ECP_E_NOERR 0
#define ECP_E_GENERIC 1

/* File Includes */
#include <stdint.h>

/* Typedefs, Structs, etc. */
typedef int tECP_Error;

struct _tECP_Context;
struct _tECP_MessageHandler;

typedef tECP_Error (*message_parser)(struct _tECP_Context * context, DBusMessage * message, struct _tECP_MessageHandler * handler);

typedef struct _tECP_MessageHandler {
    struct _tECP_MessageHandler * next;
    char * interface;
    char * member;
    message_parser parser;
} tECP_MessageHandler;

typedef struct _tECP_Context {
  int talk;
  int listen;
  int talk_id;
  int listen_id;
  tECP_MessageHandler * callbacks;
  DBusConnection * connection;
  DBusObjectPathVTable vtable;
} tECP_Context;

/* Callback type for ECP_Listen callbacks */
typedef DBusHandlerResult (*tECP_Callback)(DBusConnection * connection, DBusMessage * message, void * data );

/* Function Prototypes */
tECP_Error ECP_Init( tECP_Context * context, const char * name, tECP_Callback callback);
tECP_Error ECP_Listen(tECP_Context * context, const char * channel);
tECP_Error ECP_Broadcast( tECP_Context * context, DBusMessage * message );
tECP_Error ECP_Loop( tECP_Context * context, unsigned int timeout );
tECP_Error ECP_Destroy( tECP_Context * context );
tECP_Error ECP_Handle_Message(tECP_Context * context, DBusMessage * message);
tECP_Error ECP_Add_Message_Handler(tECP_Context * context, tECP_MessageHandler * handler);
tECP_Error ECP_Call(tECP_Context * context, DBusMessage * message);
tECP_Error ECP_Register_Method(tECP_Context * context, const char * method, tECP_Callback callback);