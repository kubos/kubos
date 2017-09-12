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

/* Response Codes found in the tECP_Message_Ack message */
#define ECP_R_SUCCESS          0x00000000
#define ECP_R_ERROR            0x80000000

/* Channel IDs */
#define ECP_C_SYS              0x00001000
#define ECP_C_EPS              0x00001001

/* Message IDs used as tECP_Message id's */
#define ECP_M_SYS              "SYS"
#define ECP_M_SYS_BEGIN        0x00010000

#define ECP_M_RIO              "RIO"
#define ECP_M_RIO_INFO_REQ     0x00020000
#define ECP_M_RIO_INFO_RES     0x00020001
#define ECP_M_RIO_ON           0x00020002
#define ECP_M_RIO_ON_ACK       0x00020003
#define ECP_M_RIO_OFF          0x00020004
#define ECP_M_RIO_OFF_ACK      0x00020005
#define ECP_M_RIO_XMIT         0x00020006
#define ECP_M_RIO_XMIT_ACK     0x00020007
#define ECP_M_RIO_RECV         0x00020009
#define ECP_M_RIO_TEMP_SET     0x0002000A
#define ECP_M_RIO_TEMP_SET_ACK 0x0002000B
#define ECP_M_RIO_TEMP         0x0002000D

#define ECP_M_EPS              "EPS"
#define ECP_M_EPS_INFO_REQ     0x00030000
#define ECP_M_EPS_INFO_RES     0x00030001
#define ECP_M_EPS_STAT_REQ     0x00030002
#define ECP_M_EPS_STAT_RES     0x00030003
#define ECP_M_EPS_ALL_ON       0x00030004
#define ECP_M_EPS_ALL_ON_ACK   0x00030005
#define ECP_M_EPS_ALL_OFF      0x00030006
#define ECP_M_EPS_ALL_OFF_ACK  0x00030007
#define ECP_M_EPS_ON           0x00030008
#define ECP_M_EPS_ON_ACK       0x00030009
#define ECP_M_EPS_OFF          0x0003000A
#define ECP_M_EPS_OFF_ACK      0x0003000B
#define ECP_M_EPS_BAT_STAT_REQ 0x0003000C
#define ECP_M_EPS_BAT_STAT_RES 0x0003000D
#define ECP_M_EPS_BAT_LVL_REQ  0x0003000E
#define ECP_M_EPS_BAT_LVL_RES  0x0003000F
#define ECP_M_EPS_BAT_LVL_ALRM 0x00030011

/* File Includes */
#include <stdint.h>

/* Typedefs, Structs, etc. */
typedef int tECP_Error;

/* Empty Message */
typedef struct {
} tECP_Message_Null;

/* Used for responses to some requests. response *should* be set to one of the ECP_R_* values */
typedef struct {
  uint32_t response;
} tECP_Message_Ack;

/* Response message for INFO queries */
typedef struct {
  uint32_t major;
  uint32_t minor;
  uint32_t patch;
} tECP_Message_Info;

/* A message format holding a single temperature value */
typedef struct {
  uint32_t temp;
} tECP_Message_Temp;

/* Response message for ECP_M_EPS_STAT_RES containing voltages & current on 8 different power rails */
typedef struct {
  int32_t voltage[8];
  int32_t current[8];
} tECP_Message_EPS;

/* Response message for ECP_M_EPS_BAT_STAT_RES */
typedef struct {
  int32_t line;
} tECP_Message_EPS_Line;

/* Response message for ECP_M_RIO_RECV or request for ECP_M_RIO_XMIT. Contains the full path to a file */
typedef struct {
  unsigned char path[ 128 ];
} tECP_Message_Filespec;

/* A message is a message ID + message ID specific content */
typedef struct {
  uint32_t id;
  union {
    tECP_Message_Null null;
    tECP_Message_Ack ack;
    tECP_Message_Info info;
    tECP_Message_Temp temp;
    tECP_Message_EPS eps;
    tECP_Message_Filespec filename;
    tECP_Message_EPS_Line line;
  } content;
} tECP_Message;

struct _tECP_Context;

typedef tECP_Error (*message_parser)(struct _tECP_Context * context, DBusMessage * message, void * handler);

typedef struct _tECP_MessageHandler {
    struct _tECP_MessageHandler * next;
    char * interface;
    char * member;
    message_parser parser;
    void * cb;
} tECP_MessageHandler;


/* Channel ID type */
typedef uint16_t tECP_Channel;

typedef struct _tECP_ChannelAction {
  struct _tECP_ChannelAction *next;
  tECP_Channel channel;
  tECP_Error (*callback) ();
} tECP_ChannelAction;

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
tECP_Error ECP_Add_Message_Handler(tECP_Context * context, tECP_MessageHandler handler);
tECP_Error ECP_Call(tECP_Context * context, const char * interface, const char * path, const char * method);
tECP_Error ECP_Register_Method(tECP_Context * context, const char * method, tECP_Callback callback);