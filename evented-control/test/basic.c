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
#include "evented-control/ecp.h"

#define TEST_NAME "org.KubOS.test"
#define TEST_LISTEN "org.KubOS.server"

static void test_ecp_init(void ** arg)
{
    ECPContext context;
    ECPStatus   err;

    err = ECP_Init(&context, TEST_NAME);

    assert_int_equal(err, ECP_OK);
}

static void test_ecp_init_listen(void ** arg)
{
    ECPContext context;
    ECPStatus   err;

    err = ECP_Init(&context, TEST_NAME);

    assert_int_equal(err, ECP_OK);

    err = ECP_Listen(&context, TEST_LISTEN);

    assert_int_equal(err, ECP_OK);
}

int main(void)
{
    const struct CMUnitTest tests[]
        = { cmocka_unit_test(test_ecp_init),
            cmocka_unit_test(test_ecp_init_listen) };

    return cmocka_run_group_tests(tests, NULL, NULL);
}
