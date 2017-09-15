#include <cmocka.h>
#include <csp/arch/csp_thread.h>
#include <csp/csp.h>
#include "evented-control/ecp.h"
#include "messages.h"

#define TEST_SUB "org.KubOS.subscriber"

static tECP_Context sub_context;
static tECP_Context pub_one_context;
static int          pub_one_num = 10;
static tECP_Context pub_two_context;
static int          pub_two_num = 99;
static int          sub_one_num = 0;
static int          sub_two_num = 0;

DBusHandlerResult sub_handler(DBusConnection * connection, DBusMessage * msg,
                              void * user_data)
{
    if (ECP_E_NOERR == ECP_Handle_Message(&sub_context, msg))
    {
        return DBUS_HANDLER_RESULT_HANDLED;
    }
    return DBUS_HANDLER_RESULT_NOT_YET_HANDLED;
}

static tECP_Error sub_one_cb(int16_t num)
{
    sub_one_num = pub_one_num;
}

static tECP_Error sub_two_cb(int16_t num)
{
    sub_two_num = pub_two_num;
}

DBusHandlerResult pub_one_handler(DBusConnection * connection,
                                  DBusMessage * msg, void * user_data)
{
    if (ECP_E_NOERR == ECP_Handle_Message(&pub_one_context, msg))
    {
        return DBUS_HANDLER_RESULT_HANDLED;
    }
    return DBUS_HANDLER_RESULT_NOT_YET_HANDLED;
}

CSP_DEFINE_TASK(pub_one_task)
{
    ECP_Init(&pub_one_context, TEST_PUB_ONE_INTERFACE, &pub_one_handler);

    usleep(100);

    for (int i = 0; i < 50; i++)
    {
        DBusMessage * message;
        format_test_signal_one_message(pub_one_num, &message);
        printf("Pub one\n");
        ECP_Broadcast(&pub_one_context, message);
    }

    ECP_Destroy(&pub_one_context);
}

DBusHandlerResult pub_two_handler(DBusConnection * connection,
                                  DBusMessage * msg, void * user_data)
{
    if (ECP_E_NOERR == ECP_Handle_Message(&pub_two_context, msg))
    {
        return DBUS_HANDLER_RESULT_HANDLED;
    }
    return DBUS_HANDLER_RESULT_NOT_YET_HANDLED;
}

CSP_DEFINE_TASK(pub_two_task)
{
    ECP_Init(&pub_two_context, TEST_PUB_TWO_INTERFACE, &pub_two_handler);

    usleep(100);

    for (int i = 0; i < 50; i++)
    {
        DBusMessage * message;
        format_test_signal_two_message(pub_two_num, &message);
        printf("Pub two\n");
        ECP_Broadcast(&pub_two_context, message);
    }

    ECP_Destroy(&pub_two_context);
}

static void test_ecp_subscriber_two_pubs(void ** arg)
{
    csp_thread_handle_t pub_one_task_handle, pub_two_task_handle;

    assert_int_equal(ECP_Init(&sub_context, TEST_SUB, &sub_handler),
                     ECP_E_NOERR);

    assert_int_equal(on_test_signal_one(&sub_context, &sub_one_cb),
                     ECP_E_NOERR);
    assert_int_equal(on_test_signal_two(&sub_context, &sub_two_cb),
                     ECP_E_NOERR);

    csp_thread_create(pub_one_task, "PUB1", 1024, NULL, 0,
                      &pub_one_task_handle);
    csp_thread_create(pub_two_task, "PUB2", 1024, NULL, 0,
                      &pub_two_task_handle);

    for (int i = 0; i < 50; i++)
    {
        printf("Sub\n");
        assert_int_equal(ECP_Loop(&sub_context, 500), ECP_E_NOERR);
        if ((pub_one_num == sub_one_num) && (pub_two_num == sub_two_num))
            break;
    }

    assert_int_equal(ECP_Destroy(&sub_context), ECP_E_NOERR);

    assert_int_equal(pub_one_num, sub_one_num);
    assert_int_equal(pub_two_num, sub_two_num);

    csp_thread_kill(pub_one_task_handle);
    csp_thread_kill(pub_two_task_handle);
}

int main(void)
{
    const struct CMUnitTest tests[]
        = { cmocka_unit_test(test_ecp_subscriber_two_pubs) };

    return cmocka_run_group_tests(tests, NULL, NULL);
}
