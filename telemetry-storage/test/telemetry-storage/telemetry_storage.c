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
#include <cmocka.h>
#include <telemetry/telemetry.h>
#include "telemetry-storage/telemetry_storage.c"
#include "telemetry-storage/config.h"

/* Test requires the following configuration:
 * #define FILE_NAME_BUFFER_SIZE 8 
 * #define DATA_BUFFER_SIZE 8
 * When running test the following argument should be called:
 * kubos test --config="path/to/test-config.json"
 * */
 
static void test_create_filename_null_pointers(void **state)
{
    assert_int_equal(create_filename(NULL,0,0,"test"),0);
    assert_int_equal(create_filename("test",0,0,NULL),0);
}


static void test_format_log_entry_csv_null_pointers(void **state)
{
    telemetry_packet packet;
    assert_int_equal(format_log_entry_csv(NULL,packet),0);
}


static void test_create_filename(void **state)
{
    static char filename_buffer[FILE_NAME_BUFFER_SIZE];
    static char *filename_buf_ptr;
    filename_buf_ptr = filename_buffer;
    
    /* Test catching partial writes from snprintf by passing 8 chars 
     * given a buffer size of 8 (no room for null terminator)
     */
    assert_int_equal(create_filename(filename_buf_ptr,255,1,"test"),0);
}


static void test_format_log_entry_csv(void **state)
{
    static char data_buffer[DATA_BUFFER_SIZE];
    static char *data_buf_ptr;
    telemetry_packet packet = { .data.i = 1234, .timestamp = 0, \
        .source.subsystem_id = 0x0, .source.data_type = TELEMETRY_TYPE_INT, \
        .source.source_id = 0x1};
    
    data_buf_ptr = data_buffer;
    
    /* Test catching partial writes from snprintf by passing a telemetry
     * packet with 7 digits given a buffer size of 8 (no room for comma,
     * carriage return, new line, and null terminator)
     */
     assert_int_equal(format_log_entry_csv(data_buf_ptr, packet),0);
     
     packet.source.data_type = 2;
     
    /* Pass an unknown telemetry type */
     assert_int_equal(format_log_entry_csv(data_buf_ptr, packet),0);
}


int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_create_filename_null_pointers),
        cmocka_unit_test(test_format_log_entry_csv_null_pointers),
        cmocka_unit_test(test_create_filename),
        cmocka_unit_test(test_format_log_entry_csv),
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}
