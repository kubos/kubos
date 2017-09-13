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
#include <stdint.h>

/**
 * ECP error codes
 */
typedef enum {
    ECP_E_NOERR = 0,
    ECP_E_GENERIC,
} tECP_Error;

/**
 * Forward declarations - needed for following function pointer typedef
 */
struct _tECP_Context;
struct _tECP_MessageHandler;

/**
 * Function pointer typedef for message parser functions
 */
typedef tECP_Error (*message_parser)(struct _tECP_Context * context, DBusMessage * message, struct _tECP_MessageHandler * handler);

/**
 * Structure for MessageHandlers. These structures are
 * message specific and are used to parse/callback when
 * messages are received.
 */
typedef struct _tECP_MessageHandler {
    struct _tECP_MessageHandler * next;
    char * interface;
    char * member;
    message_parser parser;
} tECP_MessageHandler;

/**
 * Context structure - currently used to hold DBus connection
 * and MessageHandler list.
 */
typedef struct _tECP_Context {
    tECP_MessageHandler * callbacks;
    DBusConnection * connection;
} tECP_Context;

/**
 * Callback type for ECP_Listen callbacks
 */
typedef DBusHandlerResult (*tECP_Callback)(DBusConnection * connection, DBusMessage * message, void * data );

/**
 * Initializes data structures for ECP and connection.
 */
tECP_Error ECP_Init( tECP_Context * context, const char * name, tECP_Callback callback);

/**
 * Creates a subscription for the specified channel.
 */ 
tECP_Error ECP_Listen(tECP_Context * context, const char * channel);

/**
 * Sends message over ECP, meant to be used for publishing data.
 */ 
tECP_Error ECP_Broadcast( tECP_Context * context, DBusMessage * message );

/**
 * ECP loop/process function. Meant to be used in place of a message 
 * processing super loop. Needs to be running in order for the ECP lib
 * to process incoming messages.
 */
tECP_Error ECP_Loop( tECP_Context * context, unsigned int timeout );

/**
 * Cleans up ECP connections and data structures
 */ 
tECP_Error ECP_Destroy( tECP_Context * context );

/**
 * Takes a message, iterates through the MessageHandlers in a context
 * and attempts to handle the message.
 */
tECP_Error ECP_Handle_Message(tECP_Context * context, DBusMessage * message);

/**
 * Adds a MessageHandler into the context's list of handlers.
 */
tECP_Error ECP_Add_Message_Handler(tECP_Context * context, tECP_MessageHandler * handler);

/**
 * Sends a method call message over ECP. Expects a reply from the message
 * and will block for up to 1000 ms until reply received.
 */
tECP_Error ECP_Call(tECP_Context * context, DBusMessage * message);

