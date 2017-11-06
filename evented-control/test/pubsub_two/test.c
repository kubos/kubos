#include <cmocka.h>
#include <csp/arch/csp_thread.h>
#include <csp/csp.h>
#include <stdio.h>
#include <sys/wait.h>
#include "evented-control/ecp.h"
#include "messages.h"

#define TEST_SUB "org.KubOS.subscriber"

static int pub_one_num = 10;
static int pub_two_num = 99;
static int sub_one_num = 0;
static int sub_two_num = 0;

static KECPStatus sub_one_cb(int16_t num)
{
    sub_one_num = pub_one_num;
}

static KECPStatus sub_two_cb(int16_t num)
{
    sub_two_num = pub_two_num;
}

static void publisher_one(void)
{
    ecp_context pub_one_context;

    ecp_init(&pub_one_context, TEST_PUB_ONE_INTERFACE);
    usleep(100);

    for (int i = 0; i < 10; i++)
    {
        DBusMessage * message;
        format_test_signal_one_message(pub_one_num, &message);
        ecp_send(&pub_one_context, message);
        usleep(100);
    }

    ecp_destroy(&pub_one_context);
}

static void publisher_two(void)
{
    ecp_context pub_two_context;

    ecp_init(&pub_two_context, TEST_PUB_TWO_INTERFACE);
    usleep(100);

    for (int i = 0; i < 50; i++)
    {
        DBusMessage * message;
        format_test_signal_two_message(pub_two_num, &message);
        ecp_send(&pub_two_context, message);
        usleep(100);
    }

    ecp_destroy(&pub_two_context);
}

static void test_ecp_subscriber_two_pubs(void ** arg)
{
    ecp_context sub_context;

    assert_int_equal(ecp_init(&sub_context, TEST_SUB), ECP_OK);

    assert_int_equal(on_test_signal_one(&sub_context, &sub_one_cb), ECP_OK);
    assert_int_equal(on_test_signal_two(&sub_context, &sub_two_cb), ECP_OK);

    for (int i = 0; i < 50; i++)
    {
        assert_int_equal(ecp_loop(&sub_context, 500), ECP_OK);
        if ((pub_one_num == sub_one_num) && (pub_two_num == sub_two_num))
            break;
    }

    assert_int_equal(ecp_destroy(&sub_context), ECP_OK);

    assert_int_equal(pub_one_num, sub_one_num);
    assert_int_equal(pub_two_num, sub_two_num);
}

int main(void)
{
    const struct CMUnitTest tests[]
        = { cmocka_unit_test(test_ecp_subscriber_two_pubs) };
    pid_t pid;
    int   stat;

    pid = fork();
    if (0 == pid)
    {
        publisher_one();
    }
    else
    {
        pid = fork();
        if (0 == pid)
        {
            publisher_two();
        }
        else
        {
            int status = cmocka_run_group_tests(tests, NULL, NULL);
            wait(&stat);
            wait(&stat);
            return status;
        }
    }
}
