
#include <stdio.h>
#include "test-suite.h"


DECLARE_TEST(location);
//DECLARE_TEST(radio);

const test_command_t test_suite[] = {
    TEST_COMMAND(location),
 //   TEST_COMMAND(radio),
    { NULL, NULL }
};



int main(void)
{
    puts("Welcome to KubOS Test Suite!\n");

    int result;
    int test_num = 0;

    while (test_suite[test_num].name != NULL)
    {
    	printf("Beginning test of %s\n", test_suite[test_num].name);
    	result = (test_suite[test_num].test_func)();
    	printf("\nCompleted test of %s. Status = %s\n", 
    			test_suite[test_num].name,
    			result == TEST_PASS ? "PASS" : "FAIL");
    	test_num++;
    }

    printf("\n\nAll tests have been completed.\n");

    return 0;
}
