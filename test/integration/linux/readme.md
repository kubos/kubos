# The Integration Test Script

This script runs a series of test described in the `test_config.json` file.

## Usage

        $ ./test_runner.py <Path-to-config-file> <device-path>

*NOTE* device-path defaults to `/dev/FTDI` if not provided

The JSON schema is as follows:

### Example Test Config:
```
{
    "device": {
        "baudrate": 115200,
        "prompt": "~ # ",
        "target": "kubos-linux-isis-gcc",
        "timeout": 2
    },
    "login": {
        "username": "root",
        "password": "Kubos123"
    },
    "tests": [
        {
            "name": "Command and Control Ping Test",
            "pre_test": "ls",
            "build_source": "https://github.com/kubos/kubos-linux-example",
            "flash_source": "Path to some file",
            "test_command" : "/usr/bin/c2 exec core ping",
            "post_test": "pwd",
            "expected_regex": true,
            "expected_test_output": "Return Code: 0\nExecution Time \\d\\.\\d{6}\nOutput: Pong!"
        }
    ]
}

```

### Device

The device description that will be used during the test. This is a required section

* baudrate - The baudrate for the serial connection the target device is using. Defaults to `115200` if not provided.
* prompt - The command line prompt that the target device uses. This is needed for response parsing purposes.
* target - The Kubos CLI Target name for building projects.
* timeout - The maximum time to wait for reading serial output from the board. Defaults to 10 seconds if not provided.


### Login

This section's options should be self descriptive

* username - defaults to "root" if not provided
* password - defaults to "Kubos123" if not provided

This is not a required section. If it's not provided no login will be attempted.  It's assumed that your device is already logged into and has an active shell running.

### Tests

This section is an array of test descriptions. The tests are run in the order they are listed in the array.

* name - The name of the test. For user recognition only. This has no effect on the test.
* abort_on_failure - If true and this test fails, the runner will quit and not run any following tests. If not provided, it defaults to false.
* pre_test - A shell command that will be executed before cloning, building, or running any tests. This attribute is not required
* build_source - A description of where the test source is located. This can be a local directory or a git repo url. This section is not required. If not provided, no binaries will be built.
* flash_source - The path to what should be flashed to the target. If you have a build source specified this attribute is not necessary. It is intended to be used to upload non-built resources (like shell scripts)
* test_command - The actual command that will be executed on the board to start the test on the target device.
* post_test - A shell command that will be run on the Host after all the tests and assertions have been made.  This attribute is not required.
* exepected_regex - A boolean that tells the test runner if the expected text is a python regex or not. If not provided, it is assumed false.
* expected_test_output - A string, or regex that will be checked against the shell output from the target device

