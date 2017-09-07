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

/* 01_sendreceive.c - This test forks a child which broadcasts a message on 
** the sys channel. The parent listens for a message on the sys channel. We
** declare victory if the message is received.
*/

#define _BSD_SOURCE

#include <string.h>
#include <unistd.h>
#include <stdio.h>
#include <signal.h>
#include <error.h>
#include "evented-control/ecp.h"

void _alarm_handler( int e );
tECP_Error _sys_handler( tECP_Context * context, tECP_Message * message );

int success = 0;
int complete = 0;
int parent = 0;

int main( int argc, char * argv [] ) {
  pid_t pid;
  tECP_Error err = ECP_E_NOERR;
  tECP_Context context;
  tECP_Message message;

  memset( & message, 0, sizeof( tECP_Message ) );

  printf( "01SR: Begin\n" );
  
  pid = fork();

  if( 0 == pid ) { // Child
    printf( "01SR: Child\n" );
    sleep( 5 );
    do {
      if( ECP_E_NOERR != ( err = ECP_Init( & context ) ) ) {
        printf( "01SR: Child Error calling ECP_Init() %d\n", err );
	break;
      }

      sleep( 1 );
      
      message.id = ECP_M_SYS_BEGIN;
      
      if( ECP_E_NOERR != ( err = ECP_Broadcast( & context, ECP_C_SYS, & message ) ) ) {
	printf( "01SR: Child Error calling ECP_Broadcast() %d\n", err );
	break;
      }      
    } while( 0 );
    signal( SIGALRM, _alarm_handler );
    alarm( 4 );
  } else if( pid > 0 ) { // Parent
    printf( "01SR: Parent\n" );
    do {
      if( ECP_E_NOERR != ( err = ECP_Init( & context ) ) ) {
	printf( "01SR: Error calling ECP_Init() %d\n", err );
	break;
      }

      if( ECP_E_NOERR != ( err = ECP_Listen( & context, ECP_C_SYS, _sys_handler ) ) ) {
	printf( "01SR: Parent Error calling ECP_Broadcast() %d\n", err );
	break;
      }
    } while( 0 );

    signal( SIGALRM, _alarm_handler );
    alarm( 4 );
  } else { // Error
    printf( "Error forking" );
  }
  
  while( 0 == complete ) {
  
    if( ECP_E_NOERR != ( err = ECP_Loop( & context, 1000 * 1000 ) ) ) {
      printf( "01SR: Error calling ECP_Loop() %d\n", err );
      break;
    }

  };

  if( ECP_E_NOERR != ( err = ECP_Destroy( & context ) ) ) {
    printf( "01SR: Error calling ECP_Destroy() %d\n", err );
  }

  if( pid > 0 ) {
    if( 0 == success ) {
      printf( "01SR: Failed to receive message\n" );
    } else {
      printf( "01SR: Received message\n" );
    }
  }
  
  return( err );
}

tECP_Error _sys_handler( tECP_Context * context, tECP_Message * message ) {
  success = 1;
  complete = 1;
  return( ECP_E_NOERR );
}

void _alarm_handler( int e ) {
  complete = 1;
}
