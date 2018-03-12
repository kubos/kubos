# Kubos Linux  - iMTQ Test

This project tests the usage of the Kubos API for the ISIS iMTQ magnetorquer.

After this test has completed, there will be a file located in the calling user's home directory, `imtq-results.txt`,
which contains the JSON output from the self-test and telemetry calls. These results may be examined to further
confirm that the test has completed successfully.

Test Shell Command:

    $ isis-imtq

Expected Output:

    ADCS tests completed successfully