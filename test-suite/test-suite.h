

#ifdef __cplusplus
extern "C" {
#endif

#define TEST_PASS 0
#define TEST_FAIL (-1)


typedef int (*kubos_test_function_t)(void);

typedef struct test_command {
    kubos_test_function_t test_func;
    const char*	 		  name;
} test_command_t;

#define DECLARE_TEST(n) extern int n ## _test(void)

#define TEST_COMMAND(n)  { n ## _test, #n }

#ifdef __cplusplus
}
#endif
