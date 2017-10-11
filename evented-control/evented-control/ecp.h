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
/**
 * @defgroup ECP_API Evented Control Plane API
 * @addtogroup ECP_API
 * @{
 */

#pragma once

#include <dbus/dbus.h>
#include <stdint.h>

#define DEFAULT_SEND_TIMEOUT 1000

/**
 * ECP error codes
 */
typedef enum {
    ECP_OK = 0,
    ECP_ERROR,
} KECPStatus;

/**
 * ecp_context forward declaration
 */
struct _ecp_context;

/**
 * ecp_message_handler forward declaration
 */
struct _ecp_message_handler;

/**
 * Function pointer typedef for message parser functions
 */
typedef KECPStatus (*ecp_message_parser)(const struct _ecp_context * context,
                                         DBusMessage *               message,
                                         struct _ecp_message_handler * handler);

/**
 * Structure for MessageHandlers. These structures are
 * message specific and are used to parse/callback when
 * messages are received.
 */
typedef struct _ecp_message_handler
{
    /** Next MessageHandler in list */
    struct _ecp_message_handler * next;
    /** Interface of DBus object that owns the method/signal */
    char * interface;
    /** Name of DBus signal/method producing messages */
    char * member;
    /** Function pointer to parser for handling messages */
    ecp_message_parser parser;
} ecp_message_handler;

/**
 * Context structure - currently used to hold DBus connection
 * and MessageHandler list.
 */
typedef struct _ecp_context
{
    /** List of message handlers */
    ecp_message_handler * callbacks;
    /** DBus connection object */
    DBusConnection * connection;
} ecp_context;

/**
 * Callback type for ecp_listen callbacks
 */
typedef DBusHandlerResult (*ECPCallback)(DBusConnection * connection,
                                         DBusMessage * message, void * data);

/**
 * Initializes data structures for ECP and connection.
 * @param[out] context ECP Context
 * @param[in] name Current process interface name for ECP
 * @return KECPStatus ECP_OK if successful, otherwise an error
 */
KECPStatus ecp_init(ecp_context * context, const char * name);

/**
 * Creates a subscription for the specified channel.
 * @param[in,out] context ECP Context
 * @param[in] channel Broadcast channel to listen to
 * @return KECPStatus ECP_OK if successful, otherwise an error
 */
KECPStatus ecp_listen(ecp_context * context, const char * channel);

/**
 * ECP loop/process function. Meant to be used in place of a message
 * processing super loop. Needs to be running in order for the ECP lib
 * to process incoming messages.
 * @param[in] context ECP Context
 * @param[in] timeout timeout for internal loop/work function
 * @return KECPStatus ECP_OK if successful, otherwise an error
 */
KECPStatus ecp_loop(const ecp_context * context, unsigned int timeout);

/**
 * Cleans up ECP connections and data structures
 * @param[in,out] context ECP Context
 * @return KECPStatus ECP_OK if successful, otherwise an error
 */
KECPStatus ecp_destroy(ecp_context * context);

/**
 * Takes a message, iterates through the MessageHandlers in a context
 * and attempts to handle the message.
 * @param[in] context ECP Context
 * @param[in] message Newly received message which needs handling
 * @return KECPStatus ECP_OK if successful, otherwise an error
 */
KECPStatus ecp_handle_message(const ecp_context * context,
                              DBusMessage *       message);

/**
 * Adds a MessageHandler into the context's list of handlers.
 * @param[in,out] context ECP context with list of message handlers
 * @param[in] handler message handler to add to context
 * @return KECPStatus ECP_OK if successful, otherwise an error
 */
KECPStatus ecp_add_message_handler(ecp_context *         context,
                                   ecp_message_handler * handler);

/**
 * Sends a method call message over ECP. Expects a reply from the message
 * and will block for up to specified timeout until reply received.
 * @param[in] message method call message to be sent
 * @param[in] context ECP context with connection information
 * @param[in] timeout timeout used to wait for reply
 * @return KECPStatus ECP_OK if successful, otherwise an error
 */
KECPStatus ecp_send_with_reply(const ecp_context * context,
                               DBusMessage * message, uint32_t timeout);

/**
 * Sends a message over ECP. This is meant to be used for publishing data.
 * This function does not wait for a reply and wil return immediately.
 * @param[in] context ECP Context
 * @param[in] message DBusMessage to be sent
 * @return KECPStatus ECP_OK if successful, otherwise an error
 */
KECPStatus ecp_send(const ecp_context * context, DBusMessage * message);

/* @} */
