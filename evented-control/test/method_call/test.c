#include <cmocka.h>
#include <csp/arch/csp_thread.h>
#include <csp/csp.h>
#include <sys/wait.h>
#include "evented-control/ecp.h"
#include "messages.h"

#define TEST_CLIENT "org.KubOS.Client"

static int server_num = 10;
static int client_num = 0;

static KECPStatus server_cb(int16_t num)
{
    server_num = client_num;
}

static void test_ecp_method_server(void ** arg)
{
    ecp_context server_context;

    ecp_init(&server_context, TEST_SERVER_INTERFACE);
    on_test_method(&server_context, server_cb);

    for (int i = 0; i < 10; i++)
    {
        ecp_loop(&server_context, 100);
    }

    ecp_destroy(&server_context);

    assert_int_equal(server_num, client_num);
}

static void test_ecp_method_call(void ** arg)
{
    ecp_context client_context;
    KECPStatus  stat;
    assert_int_equal(ecp_init(&client_context, TEST_CLIENT), ECP_OK);

    // Give the server task time to get setup...we need some better testing
    // tools
    // or methods which allo synchronizing inside of integration tests
    for (int i = 0; i < 10; i++)
    {
        stat = call_test_method(&client_context, client_num);
        if (ECP_OK == stat) break;
        usleep(100);
    }
    assert_int_equal(stat, ECP_OK);

    assert_int_equal(ecp_destroy(&client_context), ECP_OK);
}

int main(void)
{
    pid_t pid;
    int   stat;

    pid = fork();
    if (0 == pid)
    {
        const struct CMUnitTest tests[]
            = { cmocka_unit_test(test_ecp_method_server) };
        return cmocka_run_group_tests(tests, NULL, NULL);
    }
    else
    {
        const struct CMUnitTest tests[]
            = { cmocka_unit_test(test_ecp_method_call) };
        int status = cmocka_run_group_tests(tests, NULL, NULL);
        wait(&stat);
        return status;
    }
}
