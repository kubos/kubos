#include <cmocka.h>
#include <csp/arch/csp_thread.h>
#include <csp/csp.h>
#include <signal.h>
#include "evented-control/ecp.h"
#include "messages.h"

#define TEST_SUB "org.KubOS.subscriber"

static int pub_num = 10;
static int sub_num = 0;

static KECPStatus sub_cb(int16_t num)
{
    sub_num = pub_num;
}

CSP_DEFINE_TASK(pub_task)
{
    ecp_context pub_context;
    ecp_init(&pub_context, TEST_PUB_INTERFACE);

    usleep(100);

    for (int i = 0; i < 50; i++)
    {
        DBusMessage * message;
        format_test_signal_message(pub_num, &message);
        ecp_send(&pub_context, message);
    }

    ecp_destroy(&pub_context);
}

static void test_ecp_subscriber(void ** arg)
{
    ecp_context         sub_context;
    csp_thread_handle_t pub_task_handle;

    csp_thread_create(pub_task, "PUB", 1024, NULL, 0, &pub_task_handle);

    assert_int_equal(ecp_init(&sub_context, TEST_SUB), ECP_OK);

    assert_int_equal(on_test_signal(&sub_context, &sub_cb), ECP_OK);

    for (int i = 0; i < 50; i++)
    {
        assert_int_equal(ecp_loop(&sub_context, 100), ECP_OK);
        if (pub_num == sub_num) break;
    }

    assert_int_equal(ecp_destroy(&sub_context), ECP_OK);

    assert_int_equal(pub_num, sub_num);

    csp_thread_kill(pub_task_handle);
}

int main(void)
{
    const struct CMUnitTest tests[] = { cmocka_unit_test(test_ecp_subscriber) };

    return cmocka_run_group_tests(tests, NULL, NULL);
}
