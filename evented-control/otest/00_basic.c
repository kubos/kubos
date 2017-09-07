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

/* 00_basic.c
**
** This is a basic test of the API and a demo of how it's intended to be
** used.
*/

#include <stdlib.h>
#include <stdio.h>
#include "evented-control/ecp.h"

/* This is a callback setup with the ECP_Listen() API call. */
tECP_Error _sys_handler( tECP_Context * context, tECP_Message * message );

int main( int argc, char *argv[] ) {
  /* Two data types you'll need to know about are tECP_Error and tECP_Context.
  ** The former is (not surprisingly) an atomic integer type used to
  ** communicate success status of API calls. The latter contains all the
  ** state needed to communicate on the ECP message bus.
  */

  tECP_Error err;
  tECP_Context context;

  /* Using MIT style do { ... } while( 0 ); construct in preference to gotos
  ** or nested if's.
  */
  do {

    /* First, we call the ECP_Init() function. This initializes the connection
    ** with the message bus.
    */

    if( ECP_E_NOERR != ( err = ECP_Init( & context ) ) ) {
      printf( "00BASIC: Error calling ECP_Init(): %d\n", err );
      break;
    }

    printf( "00BASIC: Successfully called ECP_Init()\n" );

    /* Now we call the ECP_Listen() function to set up the callback for a
    ** particular channel. In this case we're registering a callback for the
    ** ECP_C_SYS channel.
    */

    if( ECP_E_NOERR != ( err = ECP_Listen( & context, ECP_C_SYS, _sys_handler ) ) ) {
      printf( "00BASIC: Error calling ECP_Listen(): %d\n", err );
      break;
    }
    
    printf( "00BASIC: Successfully called ECP_Listen()\n" );

    /* The ECP_Loop() function actually kicks off the listen and broadcast
    ** calls and blocks until there's activity on a message channel we're
    ** listening on or until a certain number of microseconds goes by. This
    ** call waits for one second (1000 * 1000 microseconds.)
    **
    ** In this case we're only calling ECP_Loop() once. In a "real" app,
    ** we would put this in a loop.
    */
    if( ECP_E_NOERR != ( err = ECP_Loop( & context, 1000 * 1000 ) ) ) {
      printf( "00BASIC: Error calling ECP_Loop(): %d\n", err );
      break;
    }

    printf( "00BASIC: Successfully called ECP_Loop()\n" );

    /* ECP_Destroy() cleans up after you're ready to stop interacting with
    ** the message bus.
    */
    if( ECP_E_NOERR != ( err = ECP_Destroy( & context ) ) ) {
      printf( "00BASIC: Error calling ECP_Destroy(): %d\n", err );
      break;
    }

    printf( "00BASIC: Successfully called ECP_Destroy()\n" );

  } while( 0 );

  return( err );
}

tECP_Error _sys_handler( tECP_Context * context, tECP_Message * message ) {
  return( ECP_E_NOERR );
}
