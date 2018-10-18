Using an ISIS Anntenna System in a Kubos Project
================================================

This document covers the particular capabilities and requirements of the Kubos API for the
`ISIS Antenna Systems <https://www.isispace.nl/product-category/products/antenna-systems/>`__.

.. toctree::
    :maxdepth: 1
    
    ISIS AntS API <ants_api>

Reference Documents
-------------------

ISIS
~~~~

    - Antenna System Interface Control Document - The main hardware guide for the antenna
    - Antenna Systems Options Sheet - Allows customers to specify non-default options that their antenna should be manufactured with

Kubos
~~~~~

    - :doc:`Creating a Kubos Project <../../../tutorials/first-project>`
    - :doc:`Configuring a Kubos Project <../../../sdk-docs/sdk-project-config>`
    - :doc:`Using Kubos Linux <../../../os-docs/using-kubos-linux>`
    - :doc:`Working with the iOBC <../../../os-docs/working-with-the-iobc>`

Run-Time Configuration
----------------------

The ISIS antenna system supports in-flight configuration changes via the :cpp:func:`k_ants_configure` function.
The function takes a :cpp:type:`KANTSController` value which indicates which microcontroller should be used
to issue future antenna system commands.

Arming the System
-----------------

Deployment can be enabled using the :cpp:func:`k_ants_arm` function.

Deployment can subsequently be disabled using the :cpp:func:`k_ants_disarm` function.

.. note:: Arming/Disarming is done on a per-microcontroller basis
    
Deployment
----------

There are several functions which can be used to control the deployment of the systems antennas.

Manual Deployment
~~~~~~~~~~~~~~~~~

Deployment of a single antenna can be controlled using the :cpp:func:`k_ants_deploy` function.

The function takes three arguments:

    - The antenna to deploy, specified by :cpp:type:`KANTSAnt`
    - Whether prior deployment should be overridden
    - The deployment timeout value, in seconds (the default is 30 seconds)

For example:

.. code-block:: c

    #include <isis-ants-api/ants-api.h>
    
    KANTSStatus status;

    /* Deploy antenna #3 with a 20 second timeout value */
    status = k_ants_deploy(ANT_3, false, 20);
    if (status != ANTS_OK)
    {
        fprintf(stderr, "Failed to deploy antenna: %d\n", status);
        return ANTS_ERROR;
    }

Automatic Deployment
~~~~~~~~~~~~~~~~~~~~

The :cpp:func:`k_ants_auto_deploy` function can be used to automatically deploy each antenna in sequential order.

The function takes a single argument of the deployment timeout value for each antenna, in seconds (the default is 30 seconds).

Canceling Deployment
~~~~~~~~~~~~~~~~~~~~

All current deployment actions can be canceled by using :cpp:func:`k_ants_cancel_deploy`.

Watchdog
--------

Each antenna microcontroller has a watchdog which will restart the system if it has not been fed within the required interval.

There are three provided functions to assist with maintenance:

    - :cpp:func:`k_ants_watchdog_kick` - Send a single kick command to each microcontroller's watchdog
    - :cpp:func:`k_ants_watchdog_start` - Start a thread to send a kick command every (watchdog_interval/3) seconds to keep the watchdogs from starving
    - :cpp:func:`k_ants_watchdog_stop` - Terminate the watchdog thread

Other Functions
---------------

    - :cpp:func:`k_ants_reset` - Reset both microcontrollers.
    - :cpp:func:`k_ants_get_uptime` - Returns the uptime of the system, in seconds. Value is zero if the system is offline.
    - :cpp:func:`k_ants_get_activation_count` - Returns the number of times deployment has been attempted for an antenna.
    - :cpp:func:`k_ants_get_activation_time` - Returns the amount of time spent deploying a single antenna.

Telemetry Information Available
-------------------------------

    - :cpp:member:`System uptime <ants_telemetry::uptime>`

        - :cpp:func:`k_ants_get_system_telemetry`

    - :cpp:member:`Raw system temperature <ants_telemetry::raw_temp>`

        - :cpp:func:`k_ants_get_system_telemetry`
        - See section 7.5.1 of the ICD for conversion information

    - :c:macro:`Whether system is armed <SYS_ARMED>`

        - :cpp:func:`k_ants_get_deploy_status`
        - :cpp:func:`k_ants_get_system_telemetry`

    - :c:macro:`Whether state of the deployment switches is being ignored <SYS_IGNORE_DEPLOY>`

        - :cpp:func:`k_ants_get_deploy_status`
        - :cpp:func:`k_ants_get_system_telemetry`

    - :c:macro:`Whether the independent burn system is active <SYS_BURN_ACTIVE>`

        - :cpp:func:`k_ants_get_deploy_status`
        - :cpp:func:`k_ants_get_system_telemetry`

    - :c:macro:`Whether each antenna deployment is active <ANT_1_ACTIVE>`

        - :cpp:func:`k_ants_get_deploy_status`
        - :cpp:func:`k_ants_get_system_telemetry`

    - :c:macro:`Whether each antenna is deployed <ANT_1_NOT_DEPLOYED>`

        - :cpp:func:`k_ants_get_deploy_status`
        - :cpp:func:`k_ants_get_system_telemetry`
        - **Note:** The flag actually indicates that the antenna has **not** been deployed

    - :c:macro:`Whether each antenna deployment stopped due to timeout <ANT_1_STOPPED_TIME>`

        - :cpp:func:`k_ants_get_deploy_status`
        - :cpp:func:`k_ants_get_system_telemetry`

    - Number of times deployment has been attempted for an antenna

        - :cpp:func:`k_ants_get_activation_count`

    - Amount of time spent attempting deployment for an antenna

        - :cpp:func:`k_ants_get_activation_time`

