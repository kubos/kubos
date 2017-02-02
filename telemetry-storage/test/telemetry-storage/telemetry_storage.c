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
#include "source/telemetry_storage.c"
#include "telemetry-storage/config.h"
 
static void test_create_filename_null_pointers(void **state)
{
    assert_int_equal(create_filename(NULL, 0, 0, "test"), 0);
    assert_int_equal(create_filename("test", 0, 0, NULL), 0);
}


static void test_format_log_entry_csv_null_pointers(void **state)
{
    telemetry_packet packet;
    assert_int_equal(format_log_entry_csv(NULL, packet), 0);
}


static void test_create_filename(void **state)
{
    static char filename_buffer[FILE_NAME_BUFFER_SIZE];
    static char *filename_buf_ptr;
    filename_buf_ptr = filename_buffer;
    char test_string_file_ext[FILE_NAME_BUFFER_SIZE];
    char test_compare_string[] = "11.tst";
    memset(test_string_file_ext, 't', sizeof(test_string_file_ext));

    /* Test catching partial writes from snprintf by passing 128 chars 
     * given a file extension size of 126 and a address and source id of 
     * one char each (no room for null terminator)
     */
    assert_int_equal(create_filename(filename_buf_ptr, 1, 1, test_string_file_ext), 0);
    
    create_filename(filename_buf_ptr, 1, 1, ".tst");
    
    /* Test string comparison with expected output */
    assert_string_equal(filename_buffer, test_compare_string);
}


static void test_format_log_entry_csv(void **state)
{
    static char data_buffer[DATA_BUFFER_SIZE];
    static char *data_buf_ptr;
    char test_compare_string[] = "1,1";
    
    telemetry_packet packet = { .data.f = FLT_MAX, .timestamp = 65535, \
         .source.data_type = TELEMETRY_TYPE_FLOAT };
         
    telemetry_packet test_packet = { .data.i = 1, .timestamp = 1, \
         .source.data_type = TELEMETRY_TYPE_INT};
    
    data_buf_ptr = data_buffer;
    
    /* Test the maximum length of a log entry. Currently a max float 
     * plus a max timestamp. 
     */
     assert_int_equal(format_log_entry_csv(data_buf_ptr, packet), 52);
     
     packet.source.data_type = 3;
     
    /* Pass an unknown telemetry type */
     assert_int_equal(format_log_entry_csv(data_buf_ptr, packet), 0);
     
     format_log_entry_csv(data_buf_ptr, test_packet);
     
    /* Test string comparison with expected output */
    assert_string_equal(data_buffer, test_compare_string);
}


static void test_telemetry_store(void **state)
{
    telemetry_packet packet = { .data.i = 1, .timestamp = 0, \
        .source.subsystem_id = 0, .source.data_type = TELEMETRY_TYPE_INT, \
        .source.source_id = 1};
        
    expect_not_value_count(__wrap_klog_init_file, handle->config.file_path, NULL, 2);
    expect_not_value_count(__wrap_klog_init_file, handle->config.file_path_len, 0, 2);
    expect_not_value_count(__wrap_klog_init_file, handle->config.part_size, 0, 2);
    expect_not_value_count(__wrap_klog_init_file, handle->config.max_parts, 0, 2);
    expect_not_value_count(__wrap_klog_init_file, handle->config.klog_file_logging, 0, 2);
    
    expect_in_range_count(__wrap_klog_init_file, handle->config.klog_console_level, 0, LOG_ALL+1, 2);
    expect_in_range_count(__wrap_klog_init_file, handle->config.klog_file_level, 0, LOG_ALL+1, 2);
    
    will_return(__wrap_klog_init_file, 0);
    assert_true(telemetry_store(packet));
    
    will_return(__wrap_klog_init_file,-1);
    assert_false(telemetry_store(packet));
}


int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_create_filename_null_pointers),
        cmocka_unit_test(test_format_log_entry_csv_null_pointers),
        cmocka_unit_test(test_create_filename),
        cmocka_unit_test(test_format_log_entry_csv),
        cmocka_unit_test(test_telemetry_store),
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}
