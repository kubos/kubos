#include <cmocka.h>
#include <csp/arch/csp_thread.h>
#include <csp/csp.h>
#include "evented-control/ecp.h"
#include "messages.h"

#define TEST_CLIENT "org.KubOS.Client"

static int server_num = 10;
static int client_num = 0;

static KECPStatus server_cb(int16_t num)
{
    printf("server cb %d %d\n", server_num, client_num);
    server_num = client_num;
    printf("cb after %d %d\n", server_num, client_num);
}

CSP_DEFINE_TASK(server_task)
{
    ecp_context server_context;
    ecp_init(&server_context, TEST_SERVER_INTERFACE);
    on_test_method(&server_context, server_cb);

    for (int i = 0; i < 10; i++)
    {
        ecp_loop(&server_context, 100);
    }

    ecp_destroy(&server_context);
}

static void test_ecp_method_call(void ** arg)
{
    ecp_context         client_context;
    csp_thread_handle_t server_task_handle;

    csp_thread_create(server_task, "SERVER", 1024, NULL, 0,
                      &server_task_handle);

    assert_int_equal(ecp_init(&client_context, TEST_CLIENT), ECP_OK);

    // Give the server task time to get setup...we need some better testing
    // tools
    // or methods which allo synchronizing inside of integration tests
    usleep(100);
    call_test_method(&client_context, client_num);

    assert_int_equal(ecp_destroy(&client_context), ECP_OK);

    assert_int_equal(server_num, client_num);

    csp_thread_kill(server_task_handle);
}

int main(void)
{
    const struct CMUnitTest tests[]
        = { cmocka_unit_test(test_ecp_method_call) };

    return cmocka_run_group_tests(tests, NULL, NULL);
}
