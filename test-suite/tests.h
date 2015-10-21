/*
 * KubOS Core Flight Services
 * Copyright (C) 2015 Kubos Corporation
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
#ifndef TESTS_H
#define TESTS_H

#ifdef __cplusplus
extern "C" {
#endif

#include <embUnit.h>
#include <math.h>

#define ASSERT_EQUAL_INT TEST_ASSERT_EQUAL_INT
#define ASSERT_EQUAL_STRING(s1, s2) TEST_ASSERT_EQUAL_STRING((const char *) s1, (const char *) s2)
#define ASSERT_FUZZY_EQUAL_FLOAT(f1, f2, precision) do { \
    float diff = powf(10, -precision); \
    TEST_ASSERT_MESSAGE(fabsf(f1 - f2) <= diff, #f1 " != " #f2); \
} while (0)


TestRef ax25_suite(void);
TestRef aprs_suite(void);
TestRef kiss_suite(void);
TestRef nmea_suite(void);

#ifdef __cplusplus
}
#endif

#endif // TESTS_H
