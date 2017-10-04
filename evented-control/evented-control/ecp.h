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

/**
 * ECP error codes
 */
typedef enum {
    ECP_OK = 0,
    ECP_GENERIC,
} ECPStatus;

/**
 * ECPContext forward declaration
 */
struct _ECPContext;

/**
 * ECPMessageHandler forward declaration
 */
struct _ECPMessageHandler;

/**
 * Function pointer typedef for message parser functions
 */
typedef ECPStatus (*ECPMessageParser)(struct _ECPContext *        context,
                                     DBusMessage *                 message,
                                     struct _ECPMessageHandler * handler);

/**
 * Structure for MessageHandlers. These structures are
 * message specific and are used to parse/callback when
 * messages are received.
 */
typedef struct _ECPMessageHandler
{
    /** Next MessageHandler in list */
    struct _ECPMessageHandler * next;
    /** Interface of DBus object that owns the method/signal */
    char * interface;
    /** Name of DBus signal/method producing messages */
    char * member;
    /** Function pointer to parser for handling messages */
    ECPMessageParser parser;
} ECPMessageHandler;

/**
 * Context structure - currently used to hold DBus connection
 * and MessageHandler list.
 */
typedef struct _ECPContext
{
    /** List of message handlers */
    ECPMessageHandler * callbacks;
    /** DBus connection object */
    DBusConnection * connection;
} ECPContext;

/**
 * Callback type for ECP_Listen callbacks
 */
typedef DBusHandlerResult (*tECP_Callback)(DBusConnection * connection,
                                           DBusMessage * message, void * data);

/**
 * Initializes data structures for ECP and connection.
 * @param[in] context ECP Context
 * @param[in] name Current process interface name for ECP
 * @return ECPStatus ECP_OK if successful, otherwise an error
 */
ECPStatus ECP_Init(ECPContext * context, const char * name);

/**
 * Creates a subscription for the specified channel.
 * @param[in] context ECP Context
 * @param[in] channel Broadcast channel to listen to
 * @return ECPStatus ECP_OK if successful, otherwise an error
 */
ECPStatus ECP_Listen(ECPContext * context, const char * channel);

/**
 * Sends message over ECP, meant to be used for publishing data.
 * This function wait for or expect a reply.
 * @param[in] context ECP Context
 * @param[in] message DBusMessage to be sent
 * @return ECPStatus ECP_OK if successful, otherwise an error
 */
ECPStatus ECP_Broadcast(ECPContext * context, DBusMessage * message);

/**
 * ECP loop/process function. Meant to be used in place of a message
 * processing super loop. Needs to be running in order for the ECP lib
 * to process incoming messages.
 * @param[in] context ECP Context
 * @param[in] timeout timeout for internal loop/work function
 * @return ECPStatus ECP_OK if successful, otherwise an error
 */
ECPStatus ECP_Loop(ECPContext * context, unsigned int timeout);

/**
 * Cleans up ECP connections and data structures
 * @param[in] context ECP Context
 * @return ECPStatus ECP_OK if successful, otherwise an error
 */
ECPStatus ECP_Destroy(ECPContext * context);

/**
 * Takes a message, iterates through the MessageHandlers in a context
 * and attempts to handle the message.
 * @param[in] context ECP Context
 * @param[in] message Newly received message which needs handling
 * @return ECPStatus ECP_OK if successful, otherwise an error
 */
ECPStatus ECP_Handle_Message(ECPContext * context, DBusMessage * message);

/**
 * Adds a MessageHandler into the context's list of handlers.
 * @param[in] context ECP context with list of message handlers
 * @param[in] handler message handler to add to context
 * @return ECPStatus ECP_OK if successful, otherwise an error
 */
ECPStatus ECP_Add_Message_Handler(ECPContext *        context,
                                   ECPMessageHandler * handler);

/**
 * Sends a method call message over ECP. Expects a reply from the message
 * and will block for up to 1000 ms until reply received.
 * @param[in] message method call message to be sent
 * @param[in] context ECP context with connection information
 * @return ECPStatus ECP_OK if successful, otherwise an error
 */
ECPStatus ECP_Call(ECPContext * context, DBusMessage * message);


/* @} */
