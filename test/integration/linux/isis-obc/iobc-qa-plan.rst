KubOS Linux on the ISIS-OBC QA Plan
===================================

Integration tests that should be run against an iOBC in preparation
for the newest official release of KubOS Linux.

Introduction
------------

Terms
~~~~~

C2 - Abbreviation for Kubos Command and Control
ISIS - Innovative Solutions In Space
iOBC - ISIS-OBC (on-board computer)

Reference Documents
~~~~~~~~~~~~~~~~~~~

- ISIS-OBC Datasheet
- ISIS-OBC Quickstart Guide
- docs.kubos.co/sphinx

Scope
~~~~~

The goal of this plan is to test the functionality of the major components
of KubOS Linux that a user would use:

- Command and control
- Telemetry
- SPI (iOBC Supervisor only)
- I2C

All tests will be run on an ISIS-OBC.

Out of Scope
^^^^^^^^^^^^

- KubOS Linux build process. CircleCI will regularly verify the build process,
  so it is not necessary to duplicate the effort here.

Testing Environment Setup
~~~~~~~~~~~~~~~~~~~~~~~~~

Files and projects required to run QA tests should be located in the
Kubos repo under the `test/integration/linux` folder.

The release-candidate Kubos Vagrant image should include all these files
and should be used to run the tests.

Hardware Requirements
^^^^^^^^^^^^^^^^^^^^^

- An iOBC
- A SAM-ICE JTAG
- A power supply
- A LSM303DLHC sensor

Hardware Setup
^^^^^^^^^^^^^^

-  Follow section 4 of the `ISIS-OBC Quickstart Guide` to assemble all the components of the iOBC
-  Attach the LSM303DLHC sensor:
    
    -  3V3 - H2.28
    -  GND - H2.30
    -  SDA - H1.41
    -  SCL - H1.43

Software Requirements
^^^^^^^^^^^^^^^^^^^^^

- An instance of the current KubOS SDK
- Atmel SAM-BA with ISIS' configuration files (can be installed using the ISIS SDK installer)

QA Management
-------------

Testing Process
~~~~~~~~~~~~~~~

Before running tests, verify that the master branch of the Kubos repo has passed all CircleCI
tests. This can be done by checking the badge displayed in the Readme section of the repo's
`main page<https://github.com/kubostech/kubos>`__. It should show a green "PASSED" icon.

All automatable tests will be executed by running the ``test_runner.py`` script against the
iOBC's test configuration file.

TODO: Add full script execution command example

The script will connect to the iOBC, run each test, confirm the results passed back through STDOUT,
and then return the overall pass/fail results.

All remaining tests should then be manually tested.

Role Expectations
~~~~~~~~~~~~~~~~~

ISIS
^^^^

ISIS should:

-  Run the test suite
-  Create a custom Kubos project which interacts with supported features on the iOBC

Kubos
^^^^^

Kubos should:

-  Run the test suite
-  Manually verify non-automatable tests

Approval Criteria
~~~~~~~~~~~~~~~~~

The master test script runs 100% successfully for two consecutive runs.

ISIS successfully builds and run a custom Kubos SDK project.

Kubos succesfully tests non-automatable things.

Defect Management
~~~~~~~~~~~~~~~~~

Kubers should add any bugs to the appropriate Kubos Trello board and mark them with the "Bug" label.

ISIS members should open `GitHub issues within the main Kubos repo<https://github.com/kubostech/kubos/issues`__
for any bugs found.

Testing Plan
------------

TODO: Move all "Test Steps" and "Expected Output" sections of automatable tests
to within the actual tests, once they've been created.

Kubos SDK
~~~~~~~~~

Flash project
^^^^^^^^^^^^^

This is should be the first test case run, as all of the other
test cases depend on this one passing.

Flash and test a "Hello World!" project

Flash Non-Application, Non-Upgrade File
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Flash a script to the board and verify that it executes successfully


iOBC Supervisor and SPI
~~~~~~~~~~~~~~~~~~~~~~~

Communication with the iOBC supervisor is currently the only way to test SPI 
communication.

TODO: Create a project to call the "supervisor_get_version()" function
(Since the version will never change, this test can be automated)

Test Steps
^^^^^^^^^^

1. Copy the "{TBD}" project into a new KubOS Linux project folder
2. Build the project for the iOBC target
3. Flash the project onto the iOBC
4. Log in to the iOBC
5. Issue command to run the program: ``{TBD}``

Expected Output
^^^^^^^^^^^^^^^

::

    ~ # {TBD}
    iOBC Supervisor Version: 53.53.48

Telemetry
~~~~~~~~~

Add New Subscriber
^^^^^^^^^^^^^^^^^^

Add New Publisher
^^^^^^^^^^^^^^^^^

Command and Control
~~~~~~~~~~~~~~~~~~~

Verify all of the built-in commands work.

Ping
^^^^

Test Steps
##########

Expected Output
###############

::

    ~ # c2 core ping
    Return Code: 0
    Execution Time: 0.000000
    Output: Pong!
    

Info
^^^^

Test Steps
##########

Expected Output
###############

::

    ~ # c2 core info
    Return Code: 0
    Execution Time: 0.000000
    Output: iOBC Supervisor Version: 53.53.48

Reboot
^^^^^^

I2C
~~~

The "linux-i2c" project can be used to test the LSM303DLHC sensor
on the iOBC.

Test Steps
^^^^^^^^^^

1. Copy the "linux-i2c" project into a new KubOS Linux project folder
2. Build the project for the iOBC target
3. Flash the project onto the iOBC
4. Log in to the iOBC
5. Issue command to run the program: ``linux-i2c``

Expected Output
^^^^^^^^^^^^^^^

:: 

    ~ # linux-i2c
    sh: syntax error: unexpected (
    Successfully opened i2c bus
    Successfully set slave address: 19
    Starting init_sensor
    Setting the operation mode
    Getting the operation mode
    Operation mode: 57
    Test completed successfully!
    sh: syntax error: unexpected 0

Complex Integration
~~~~~~~~~~~~~~~~~~~

Q: Create a test that will hit as many simultaneous areas as possible.

OS Upgrade
~~~~~~~~~~

**Note:** This is not an automated test

- Flash upgrade package to board
- Reboot board
- Verify that board is now running new version

    - Issue `fw_printenv kubos_curr_version` and check that the value matches
      the name of the upgrade package.

Watchdog
~~~~~~~~

**Note:** This is not an automated test

The red jumper should be removed from the iOBC programming board in order to
enable the watchdog.

There are no specific tests, however it should be documented if the iOBC
mysteriously reboots.

System Recovery
~~~~~~~~~~~~~~~

**Note** This is not an automated test case

- Recovery when current version is still available/good
- Recovery when current isn't, but previous is
- Recovery when only kpack-base.itb is available
- Recovery when nothing is available (-> U-Boot CLI)

Vague Steps:
- Delete /usr/ directory on board (to corrupt rootfs)
- Reboot
- Board should throw a kernel panic
- Recovery should happen (current version of KubOS Linux should be reloaded)


Test Plan Execution
-------------------

TODO: How to actually run the test suite. (open vagrant, run ``blah`` command, wait an eternity, check results)
