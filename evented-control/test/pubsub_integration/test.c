#include <cmocka.h>
#include <csp/arch/csp_thread.h>
#include <csp/csp.h>
#include "evented-control/ecp.h"
#include "messages.h"

#define TEST_SUB "org.KubOS.subscriber"

static tECP_Context sub_context;
static tECP_Context pub_context;
static int          pub_num = 10;
static int          sub_num = 0;

DBusHandlerResult sub_handler(DBusConnection * connection, DBusMessage * msg,
                              void * user_data)
{
    if (ECP_E_NOERR == ECP_Handle_Message(&sub_context, msg))
    {
        return DBUS_HANDLER_RESULT_HANDLED;
    }
    return DBUS_HANDLER_RESULT_NOT_YET_HANDLED;
}

DBusHandlerResult pub_handler(DBusConnection * connection, DBusMessage * msg,
                              void * user_data)
{
    if (ECP_E_NOERR == ECP_Handle_Message(&pub_context, msg))
    {
        return DBUS_HANDLER_RESULT_HANDLED;
    }
    return DBUS_HANDLER_RESULT_NOT_YET_HANDLED;
}

static tECP_Error sub_cb(int16_t num)
{
    sub_num = pub_num;
}

CSP_DEFINE_TASK(pub_task)
{
    ECP_Init(&pub_context, TEST_PUB_INTERFACE, &pub_handler);

    for (int i = 0; i < 10; i++)
    {
        DBusMessage * message;
        format_test_signal_message(pub_num, &message);
        ECP_Broadcast(&pub_context, message);
    }

    ECP_Destroy(&pub_context);
}

static void test_ecp_subscriber(void ** arg)
{
    csp_thread_handle_t pub_task_handle;

    csp_thread_create(pub_task, "PUB", 1024, NULL, 0, &pub_task_handle);

    assert_int_equal(ECP_Init(&sub_context, TEST_SUB, &sub_handler),
                     ECP_E_NOERR);

    assert_int_equal(on_test_signal(&sub_context, &sub_cb), ECP_E_NOERR);

    for (int i = 0; i < 15; i++)
    {
        assert_int_equal(ECP_Loop(&sub_context, 100), ECP_E_NOERR);
    }

    assert_int_equal(ECP_Destroy(&sub_context), ECP_E_NOERR);

    assert_int_equal(pub_num, sub_num);

    csp_thread_kill(pub_task_handle);
}

int main(void)
{
    const struct CMUnitTest tests[] = { cmocka_unit_test(test_ecp_subscriber) };

    return cmocka_run_group_tests(tests, NULL, NULL);
}
